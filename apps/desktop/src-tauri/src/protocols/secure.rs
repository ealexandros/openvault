use openvault_sdk::SecretVec;
use tauri::http::{Response, Uri};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use super::response;
use crate::AppState;

pub const PROTOCOL_SCHEME: &str = "secure";
pub const PROTOCOL_HOST: &str = "localhost";

pub struct SecurePayload {
    pub data: SecretVec,
    pub content_type: String,
}

impl SecurePayload {
    pub fn new(data: SecretVec, content_type: String) -> Self {
        Self { data, content_type }
    }
}

pub fn protocol_uri(token: &str) -> String {
    format!("{PROTOCOL_SCHEME}://{PROTOCOL_HOST}/{token}")
}

pub fn handle_secure_protocol(app: &AppHandle, request_uri: &Uri) -> Response<Vec<u8>> {
    let state = app.state::<AppState>();

    let request_uri_str = request_uri.to_string();
    let token = request_uri_str
        .strip_prefix(&protocol_uri(""))
        .unwrap_or_default()
        .to_string();

    if Uuid::parse_str(&token).is_err() {
        return response::not_found();
    }

    let mut secure_payloads = match state.secure_payloads.lock() {
        Ok(lock) => lock,
        Err(_) => return response::internal_error(),
    };

    match secure_payloads.remove(&token) {
        Some(response) => response::ok(response.data.as_bytes(), &response.content_type),
        None => response::not_found(),
    }
}
