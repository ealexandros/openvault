use crate::AppState;
use tauri::http::{Response, Uri};
use tauri::{AppHandle, Manager};
use zeroize::Zeroize;

pub const PROTOCOL_SCHEME: &str = "secure";
pub const PROTOCOL_HOST: &str = "localhost";

pub fn handle_secure_protocol(app: &AppHandle, request_uri: &Uri) -> Response<Vec<u8>> {
    let state = app.state::<AppState>();

    let request_uri = request_uri.to_string();
    let token = request_uri
        .strip_prefix(&format!("{PROTOCOL_SCHEME}://{PROTOCOL_HOST}/"))
        .unwrap_or_default();

    let mut map = state.inner.lock().unwrap();

    match map.remove(token) {
        Some(mut data) => {
            let body = std::mem::take(&mut data);

            data.zeroize();

            Response::builder()
                .status(200)
                .header("Content-Type", "application/octet-stream")
                .header("Access-Control-Allow-Origin", "*")
                .body(body)
                .unwrap()
        }
        None => Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body(Vec::new())
            .unwrap(),
    }
}
