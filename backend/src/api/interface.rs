use crate::wireguard::peers;
use crate::{error::AppError, AppState};
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateInterfaceRequest {
    pub listen_port: Option<i64>,
    pub ipv4_cidr: Option<String>,
    pub ipv6_cidr: Option<String>,
}

pub async fn get_interface(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let iface = crate::db::interfaces::get(&state.db)
        .await
        .map_err(AppError::Internal)?
        .ok_or_else(|| AppError::NotFound)?;
    // Don't expose private key
    Ok(Json(serde_json::json!({
        "id": iface.id,
        "name": iface.name,
        "public_key": iface.public_key,
        "listen_port": iface.listen_port,
        "ipv4_cidr": iface.ipv4_cidr,
        "ipv6_cidr": iface.ipv6_cidr,
    })))
}

pub async fn update_interface(
    State(state): State<AppState>,
    Json(body): Json<UpdateInterfaceRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut iface = crate::db::interfaces::get(&state.db)
        .await
        .map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    if let Some(port) = body.listen_port {
        iface.listen_port = port;
    }
    if let Some(cidr) = body.ipv4_cidr {
        iface.ipv4_cidr = cidr;
    }
    iface.ipv6_cidr = body.ipv6_cidr.or(iface.ipv6_cidr);

    crate::db::interfaces::upsert(&state.db, &iface)
        .await
        .map_err(AppError::Internal)?;

    // Re-apply to kernel
    peers::configure_interface(&iface.name, &iface.private_key, iface.listen_port as u16)
        .map_err(AppError::Internal)?;

    Ok(Json(serde_json::json!({ "ok": true })))
}
