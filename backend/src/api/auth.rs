use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::session::{get_session_id_from_headers, SESSION_COOKIE};
use crate::{error::AppError, AppState};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub authenticated: bool,
    pub username: Option<String>,
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Already logged in?
    if let Some(sid) = get_session_id_from_headers(&headers) {
        if let Some(username) = state.sessions.read().unwrap().get(&sid).cloned() {
            return Ok((
                StatusCode::OK,
                HeaderMap::new(),
                Json(SessionResponse {
                    authenticated: true,
                    username: Some(username),
                }),
            )
                .into_response());
        }
    }

    // INSECURE mode: skip password check
    if !state.config.insecure {
        let hash = state.config.password_hash.as_deref().unwrap_or("");
        let valid = bcrypt::verify(&body.password, hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("bcrypt error: {}", e)))?;
        if !valid {
            return Err(AppError::Unauthorized);
        }

        // TOTP check
        if let Some(user) = crate::db::users::find_by_username(&state.db, &body.username)
            .await
            .map_err(AppError::Internal)?
        {
            if let Some(secret) = &user.totp_secret {
                let code = body.totp_code.as_deref().unwrap_or("");
                let totp = totp_rs::TOTP::new(
                    totp_rs::Algorithm::SHA1,
                    6,
                    1,
                    30,
                    secret.as_bytes().to_vec(),
                    None,
                    body.username.clone(),
                )
                .map_err(|e| AppError::Internal(anyhow::anyhow!("TOTP error: {}", e)))?;
                if !totp
                    .check_current(code)
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("TOTP check: {}", e)))?
                {
                    return Err(AppError::Unauthorized);
                }
            }
        }
    }

    let session_id = Uuid::new_v4().to_string();
    state
        .sessions
        .write()
        .unwrap()
        .insert(session_id.clone(), body.username.clone());

    let cookie = Cookie::build((SESSION_COOKIE, session_id))
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok((
        StatusCode::OK,
        resp_headers,
        Json(SessionResponse {
            authenticated: true,
            username: Some(body.username),
        }),
    )
        .into_response())
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    if let Some(sid) = get_session_id_from_headers(&headers) {
        state.sessions.write().unwrap().remove(&sid);
    }
    let cookie = Cookie::build((SESSION_COOKIE, ""))
        .http_only(true)
        .path("/")
        .max_age(cookie::time::Duration::ZERO)
        .build();
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    (StatusCode::NO_CONTENT, resp_headers)
}

pub async fn check(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let username = get_session_id_from_headers(&headers)
        .and_then(|sid| state.sessions.read().unwrap().get(&sid).cloned());
    Json(SessionResponse {
        authenticated: username.is_some(),
        username,
    })
}
