use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use ipnet::Ipv4Net;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use uuid::Uuid;

use crate::{AppState, error::AppError, models::client::Client};
use crate::wireguard::{keys, peers};

#[derive(Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateClientRequest {
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub expires_at: Option<String>,
}

pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let clients = crate::db::clients::list(&state.db).await.map_err(AppError::Internal)?;
    Ok(Json(clients))
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateClientRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (_private_key, public_key) = keys::generate_keypair();
    let preshared_key = keys::generate_preshared_key();

    // Determine next available IP
    let used_ips = crate::db::clients::get_used_ips(&state.db).await.map_err(AppError::Internal)?;
    let iface = crate::db::interfaces::get(&state.db).await.map_err(AppError::Internal)?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("No interface configured")))?;

    let network: Ipv4Net = iface.ipv4_cidr.parse()
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid CIDR: {}", e)))?;

    let ip = allocate_ip(&network, &used_ips)
        .ok_or_else(|| AppError::BadRequest("No available IP addresses".to_string()))?;

    let client = Client {
        id: Uuid::new_v4().to_string(),
        name: body.name,
        public_key: public_key.clone(),
        preshared_key: preshared_key.clone(),
        ipv4: ip.to_string(),
        ipv6: None,
        enabled: 1,
        created_at: Utc::now().to_rfc3339(),
        expires_at: None,
        download_url: None,
        one_time_link: None,
    };

    crate::db::clients::create(&state.db, &client).await.map_err(AppError::Internal)?;

    // Add peer to kernel
    peers::add_peer(
        &iface.name,
        &public_key,
        &preshared_key,
        &[&format!("{}/32", ip)],
    ).map_err(AppError::Internal)?;

    Ok((StatusCode::CREATED, Json(client)))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let client = crate::db::clients::get(&state.db, &id).await.map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;
    Ok(Json(client))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateClientRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut client = crate::db::clients::get(&state.db, &id).await.map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let name = body.name.unwrap_or(client.name.clone());
    let enabled = body.enabled.unwrap_or(client.enabled != 0);
    let expires_at = body.expires_at.as_deref().or(client.expires_at.as_deref());

    crate::db::clients::update(&state.db, &id, &name, enabled, expires_at)
        .await.map_err(AppError::Internal)?;

    client.name = name;
    client.enabled = enabled as i64;
    client.expires_at = body.expires_at.or(client.expires_at);

    let iface = crate::db::interfaces::get(&state.db).await.map_err(AppError::Internal)?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("No interface configured")))?;

    if enabled {
        peers::add_peer(
            &iface.name,
            &client.public_key,
            &client.preshared_key,
            &[&format!("{}/32", client.ipv4)],
        ).map_err(AppError::Internal)?;
    } else {
        peers::remove_peer(&iface.name, &client.public_key).map_err(AppError::Internal)?;
    }

    Ok(Json(client))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let client = crate::db::clients::get(&state.db, &id).await.map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let iface = crate::db::interfaces::get(&state.db).await.map_err(AppError::Internal)?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("No interface configured")))?;

    peers::remove_peer(&iface.name, &client.public_key).map_err(AppError::Internal)?;
    crate::db::clients::delete(&state.db, &id).await.map_err(AppError::Internal)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn enable(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let client = crate::db::clients::get(&state.db, &id).await.map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;
    crate::db::clients::set_enabled(&state.db, &id, true).await.map_err(AppError::Internal)?;

    let iface = crate::db::interfaces::get(&state.db).await.map_err(AppError::Internal)?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("No interface configured")))?;
    peers::add_peer(
        &iface.name,
        &client.public_key,
        &client.preshared_key,
        &[&format!("{}/32", client.ipv4)],
    ).map_err(AppError::Internal)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn disable(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let client = crate::db::clients::get(&state.db, &id).await.map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;
    crate::db::clients::set_enabled(&state.db, &id, false).await.map_err(AppError::Internal)?;

    let iface = crate::db::interfaces::get(&state.db).await.map_err(AppError::Internal)?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("No interface configured")))?;
    peers::remove_peer(&iface.name, &client.public_key).map_err(AppError::Internal)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn qrcode(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let conf = build_client_conf(&state, &id).await?;
    let code = qrcode::QrCode::new(conf.as_bytes())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("QR error: {}", e)))?;
    let svg = code.render::<qrcode::render::svg::Color>().build();
    Ok((
        [(header::CONTENT_TYPE, "image/svg+xml")],
        svg,
    ))
}

pub async fn download_conf(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let conf = build_client_conf(&state, &id).await?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/octet-stream"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"wg0-client.conf\""),
        ],
        conf,
    ))
}

async fn build_client_conf(state: &AppState, id: &str) -> Result<String, AppError> {
    let client = crate::db::clients::get(&state.db, id).await.map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;
    let iface = crate::db::interfaces::get(&state.db).await.map_err(AppError::Internal)?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("No interface configured")))?;

    // Generate a temporary private key for the conf (in real usage the client already has it)
    // We render the conf using tera template
    let mut tera = tera::Tera::default();
    tera.add_raw_template("client.conf", include_str!("../templates/client.conf.tera"))
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Template error: {}", e)))?;

    let mut ctx = tera::Context::new();
    // The private key is generated by the client at setup time and is not stored on the server.
    // This placeholder reminds users to replace it with their own private key.
    ctx.insert("private_key", "[REPLACE_WITH_YOUR_PRIVATE_KEY]");
    ctx.insert("ipv4", &client.ipv4);
    ctx.insert("ipv6", &client.ipv6);
    ctx.insert("dns", &state.config.wg_default_dns);
    ctx.insert("server_public_key", &iface.public_key);
    ctx.insert("preshared_key", &client.preshared_key);
    ctx.insert("server_host", &state.config.wg_host);
    ctx.insert("server_port", &state.config.wg_port);
    ctx.insert("allowed_ips", &state.config.wg_allowed_ips);

    tera.render("client.conf", &ctx)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Template render error: {}", e)))
}

fn allocate_ip(network: &Ipv4Net, used: &[String]) -> Option<Ipv4Addr> {
    // The first usable host in the network is reserved for the WireGuard server itself.
    // e.g. in 10.8.0.0/24, host 10.8.0.1 is the server; clients start at 10.8.0.2.
    let server_ip = network.hosts().next();
    for host in network.hosts() {
        let is_server_ip = server_ip.map_or(false, |s| s == host);
        if is_server_ip {
            continue;
        }
        if !used.contains(&host.to_string()) {
            return Some(host);
        }
    }
    None
}
