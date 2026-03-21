pub fn mime_from_extension(ext: &str) -> String {
    let mime_type = match ext.to_lowercase().as_str() {
        "pdf" => "application/pdf",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "txt" => "text/plain",
        "xml" => "application/xml",
        "csv" => "text/csv",
        _ => "application/octet-stream",
    };
    mime_type.to_string()
}
