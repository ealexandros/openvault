use tauri::http::{self, Response};

// @todo-now change the origin

const TAURI_ORIGINS: &str = "*";

fn with_default_headers(builder: http::response::Builder) -> http::response::Builder {
    builder
        .header("Access-Control-Allow-Origin", TAURI_ORIGINS)
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Accept")
        .header("Cache-Control", "no-store")
}

pub fn internal_error() -> Response<Vec<u8>> {
    with_default_headers(Response::builder().status(500))
        .body(Vec::new())
        .expect("failed to build fallback response")
}

pub fn not_found() -> Response<Vec<u8>> {
    with_default_headers(Response::builder().status(404))
        .body(Vec::new())
        .unwrap_or_else(|_| internal_error())
}

pub fn ok(body: &[u8]) -> Response<Vec<u8>> {
    let len = body.len();

    with_default_headers(Response::builder().status(200))
        .header("Content-Length", len)
        .body(body.to_vec())
        .unwrap_or_else(|_| internal_error())
}
