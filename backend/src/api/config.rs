use crate::{error::AppError, AppState};
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ConfigResponse {
    pub wg_host: String,
    pub wg_port: u16,
    pub wg_default_dns: String,
    pub wg_allowed_ips: String,
    pub wg_default_address: String,
}

#[derive(Deserialize)]
pub struct UpdateConfigRequest {
    pub wg_default_dns: Option<String>,
    pub wg_allowed_ips: Option<String>,
}

pub async fn get_config(State(state): State<AppState>) -> impl IntoResponse {
    Json(ConfigResponse {
        wg_host: state.config.wg_host.clone(),
        wg_port: state.config.wg_port,
        wg_default_dns: state.config.wg_default_dns.clone(),
        wg_allowed_ips: state.config.wg_allowed_ips.clone(),
        wg_default_address: state.config.wg_default_address.clone(),
    })
}

pub async fn update_config(
    State(state): State<AppState>,
    Json(body): Json<UpdateConfigRequest>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(dns) = &body.wg_default_dns {
        crate::db::settings::set(&state.db, "wg_default_dns", dns)
            .await
            .map_err(AppError::Internal)?;
    }
    if let Some(ips) = &body.wg_allowed_ips {
        crate::db::settings::set(&state.db, "wg_allowed_ips", ips)
            .await
            .map_err(AppError::Internal)?;
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}
