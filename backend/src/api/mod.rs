use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use metrics_exporter_prometheus::PrometheusHandle;
use tower_http::services::{ServeDir, ServeFile};

use crate::AppState;

pub mod auth;
pub mod clients;
pub mod config;
pub mod interface;
pub mod metrics;
pub mod session;
pub mod stats;

pub fn build_router(state: AppState, prom_handle: PrometheusHandle) -> Router {
    let sessions = state.sessions.clone();

    let protected = Router::new()
        .route("/api/client", get(clients::list))
        .route("/api/client", post(clients::create))
        .route("/api/client/{id}", get(clients::get_one))
        .route("/api/client/{id}", put(clients::update))
        .route("/api/client/{id}", delete(clients::delete))
        .route("/api/client/{id}/enable", put(clients::enable))
        .route("/api/client/{id}/disable", put(clients::disable))
        .route("/api/client/{id}/qrcode.svg", get(clients::qrcode))
        .route(
            "/api/client/{id}/configuration",
            get(clients::download_conf),
        )
        .route("/api/interface", get(interface::get_interface))
        .route("/api/interface", put(interface::update_interface))
        .route("/api/stats", get(stats::get_stats))
        .route("/api/config", get(config::get_config))
        .route("/api/config", put(config::update_config))
        .route_layer(middleware::from_fn_with_state(
            sessions,
            session::require_auth,
        ));

    Router::new()
        // Public auth routes
        .route("/api/session", post(auth::login))
        .route("/api/session", get(auth::check))
        .route("/api/session", delete(auth::logout))
        // Prometheus metrics (no auth)
        .route("/metrics", get(metrics::prometheus))
        .layer(axum::Extension(prom_handle))
        // Protected routes
        .merge(protected)
        // React SPA fallback
        .fallback_service(ServeDir::new(&state.config.static_path).not_found_service(
            ServeFile::new(format!("{}/index.html", &state.config.static_path)),
        ))
        .with_state(state)
}
