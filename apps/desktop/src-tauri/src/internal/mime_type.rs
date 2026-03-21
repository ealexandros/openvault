pub fn mime_from_extension(ext: &str) -> String {
    let mime_type = match ext.to_lowercase().as_str() {
        // Documents
        "pdf" => "application/pdf",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" | "mjs" => "text/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "csv" => "text/csv",
        "txt" => "text/plain",
        "wasm" => "application/wasm",

        // Images
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",

        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "m4a" => "audio/mp4",
        "flac" => "audio/flac",
        "aac" => "audio/aac",

        // Video
        "mp4" => "video/mp4",
        "mov" => "video/quicktime",
        "webm" => "video/webm",
        "ogv" => "video/ogg",
        "mkv" => "video/x-matroska",
        "avi" => "video/x-msvideo",
        "wmv" => "video/x-ms-wmv",

        // Fallback
        _ => "application/octet-stream",
    };
    mime_type.to_string()
}

pub fn is_mime_video(mime_type: &str) -> bool {
    mime_type.starts_with("video/")
}

pub fn is_mime_audio(mime_type: &str) -> bool {
    mime_type.starts_with("audio/")
}
