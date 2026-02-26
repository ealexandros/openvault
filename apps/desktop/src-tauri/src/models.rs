use serde::Serialize;

#[derive(Serialize)]
pub struct FilesystemItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub details: Option<String>,
    pub mime_type: Option<String>,
}
