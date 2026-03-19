use tauri::http::{Response, Uri};
use tauri::{AppHandle, Manager};

use super::response;
use crate::AppState;

pub const PROTOCOL_SCHEME: &str = "secure";
pub const PROTOCOL_HOST: &str = "localhost";

pub fn protocol_uri(token: &str) -> String {
    format!("{PROTOCOL_SCHEME}://{PROTOCOL_HOST}/{token}")
}

pub fn handle_secure_protocol(app: &AppHandle, request_uri: &Uri) -> Response<Vec<u8>> {
    let state = app.state::<AppState>();

    let request_uri_str = request_uri.to_string();
    let token = request_uri_str
        .strip_prefix(&protocol_uri(""))
        .unwrap_or_default();

    let mut secure_proto = match state.secure_proto.lock() {
        Ok(lock) => lock,
        Err(_) => return response::internal_error(),
    };

    match secure_proto.remove(token) {
        Some(data) => response::ok(data.as_bytes()),
        None => response::not_found(),
    }
}
