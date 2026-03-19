use crate::AppState;
use tauri::http::{Response, Uri};
use tauri::{AppHandle, Manager};

pub const PROTOCOL_SCHEME: &str = "secure";
pub const PROTOCOL_HOST: &str = "localhost";

pub fn protocol_uri(token: &str) -> String {
    format!("{PROTOCOL_SCHEME}://{PROTOCOL_HOST}/{token}")
}

pub fn handle_secure_protocol(app: &AppHandle, request_uri: &Uri) -> Response<Vec<u8>> {
    let state = app.state::<AppState>();

    let request_uri = request_uri.to_string();
    let token = request_uri
        .strip_prefix(&protocol_uri(""))
        .unwrap_or_default();

    let mut map = state.secure_proto.lock().unwrap();

    // @todo-now fix the unwraps

    match map.remove(token) {
        Some(mut data) => {
            let data = std::mem::take(&mut data);

            let response = Response::builder()
                .status(200)
                .header("Content-Type", "application/octet-stream")
                .header("Access-Control-Allow-Origin", "*")
                .header("Cache-Control", "no-store")
                .body(data.as_bytes().to_vec())
                .unwrap();

            drop(data);

            response
        }
        None => Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .header("Cache-Control", "no-store")
            .body(Vec::new())
            .unwrap(),
    }
}
