use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;

pub const SESSION_COOKIE: &str = "wg_session";

/// In-memory session store: session_id → username
pub type SessionStore = Arc<RwLock<HashMap<String, String>>>;

pub fn new_store() -> SessionStore {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Extract session_id from headers.
pub fn get_session_id_from_headers(headers: &HeaderMap) -> Option<String> {
    let val = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in val.split(';') {
        if let Ok(cookie) = Cookie::parse(part.trim().to_owned()) {
            if cookie.name() == SESSION_COOKIE {
                return Some(cookie.value().to_string());
            }
        }
    }
    None
}

/// Middleware that rejects unauthenticated requests with 401.
pub async fn require_auth(
    State(sessions): State<SessionStore>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(session_id) = get_session_id_from_headers(req.headers()) {
        if sessions.read().unwrap().contains_key(&session_id) {
            return Ok(next.run(req).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
