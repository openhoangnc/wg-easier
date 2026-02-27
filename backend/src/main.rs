use anyhow::Context;
use std::net::SocketAddr;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

mod api;
mod config;
mod db;
mod error;
mod models;
mod wireguard;

use api::session::SessionStore;
pub use config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: db::Db,
    pub config: std::sync::Arc<AppConfig>,
    pub sessions: SessionStore,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Init tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // 2. Load config from env
    let config = AppConfig::from_env().context("Failed to load configuration")?;
    info!("Starting wg-easy-rs on port {}", config.port);
    info!("WireGuard host: {}", config.wg_host);

    // 3. Open SQLite pool + run migrations
    let db = db::init_db(&config.db_path).await?;

    // 4. Load or seed interface config
    let iface = match db::interfaces::get(&db).await? {
        Some(i) => i,
        None => {
            info!("No interface found, seeding defaults");
            let (private_key, public_key) = wireguard::keys::generate_keypair();
            let iface = models::interface::Interface {
                id: uuid::Uuid::new_v4().to_string(),
                name: "wg0".to_string(),
                private_key,
                public_key,
                listen_port: config.wg_port as i64,
                ipv4_cidr: "10.8.0.0/24".to_string(),
                ipv6_cidr: None,
            };
            db::interfaces::upsert(&db, &iface).await?;
            iface
        }
    };

    // 5–8. WireGuard interface setup (requires NET_ADMIN + Linux kernel)
    #[cfg(target_os = "linux")]
    {
        use wireguard::interface as wgiface;
        use wireguard::peers;

        let (conn, netlink_handle, _) =
            rtnetlink::new_connection().context("Failed to open rtnetlink connection")?;
        tokio::spawn(conn);

        // Create interface if it doesn't exist
        if !wgiface::link_exists(&netlink_handle, &iface.name).await {
            wgiface::create_wireguard_link(&netlink_handle, &iface.name)
                .await
                .context("Failed to create WireGuard interface")?;
        }

        // Configure WireGuard (private key + listen port)
        peers::configure_interface(&iface.name, &iface.private_key, iface.listen_port as u16)
            .context("Failed to configure WireGuard interface")?;

        // Assign IP address + bring up + add route
        let idx = wgiface::get_link_index(&netlink_handle, &iface.name).await?;
        let net: ipnet::IpNet = iface.ipv4_cidr.parse().context("Invalid interface CIDR")?;
        wgiface::assign_address(&netlink_handle, idx, &net)
            .await
            .unwrap_or_else(|e| tracing::warn!("assign_address: {} (may already exist)", e));
        wgiface::set_link_up(&netlink_handle, idx).await?;
        wgiface::add_route(&netlink_handle, idx, &net)
            .await
            .unwrap_or_else(|e| tracing::warn!("add_route: {} (may already exist)", e));

        // Bulk sync enabled peers
        let clients = db::clients::list_enabled(&db).await?;
        let peer_tuples: Vec<(String, String, Vec<String>)> = clients
            .into_iter()
            .map(|c| {
                (
                    c.public_key,
                    c.preshared_key,
                    vec![format!("{}/32", c.ipv4)],
                )
            })
            .collect();
        peers::sync_peers(&iface.name, &peer_tuples).context("Failed to sync peers")?;

        // 9. Setup NAT
        let outbound = std::env::var("WG_OUTBOUND_IFACE").unwrap_or_else(|_| "eth0".to_string());
        wireguard::nat::setup_nat(&iface.ipv4_cidr, &outbound)
            .unwrap_or_else(|e| tracing::warn!("NAT setup failed (may need root): {}", e));
    }

    // 10. Build app state
    let state = AppState {
        db: db.clone(),
        config: std::sync::Arc::new(config.clone()),
        sessions: api::session::new_store(),
    };

    // 11. Prometheus metrics
    let prom_builder = metrics_exporter_prometheus::PrometheusBuilder::new();
    let prom_handle = prom_builder
        .install_recorder()
        .context("Failed to install Prometheus recorder")?;

    // 12. Build router
    let app =
        api::build_router(state, prom_handle).layer(tower_http::trace::TraceLayer::new_for_http());

    // 13. Bind and serve
    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    info!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Graceful shutdown: teardown NAT and WireGuard interface
    #[cfg(target_os = "linux")]
    {
        wireguard::nat::teardown_nat()
            .unwrap_or_else(|e| tracing::warn!("NAT teardown failed: {}", e));

        let (conn, handle, _) =
            rtnetlink::new_connection().context("Failed to open rtnetlink for shutdown")?;
        tokio::spawn(conn);
        wireguard::interface::delete_link(&handle, "wg0")
            .await
            .unwrap_or_else(|e| tracing::warn!("Failed to delete wg0: {}", e));
    }

    db.close().await;
    info!("Shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    signal(SignalKind::terminate())
        .expect("failed to listen for SIGTERM")
        .recv()
        .await;
    info!("Received SIGTERM, shutting down");
}
