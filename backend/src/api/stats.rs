use crate::wireguard::peers;
use crate::{error::AppError, AppState};
use axum::{extract::State, response::IntoResponse, Json};

pub async fn get_stats(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let iface = crate::db::interfaces::get(&state.db)
        .await
        .map_err(AppError::Internal)?
        .ok_or(AppError::NotFound)?;
    let stats = peers::get_stats(&iface.name).map_err(AppError::Internal)?;
    Ok(Json(stats))
}
