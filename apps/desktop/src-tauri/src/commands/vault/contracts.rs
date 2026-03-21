use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultParams {
    pub path: String,
    pub name: String,
    pub password: Vec<u8>,
    pub encryption: String,
    pub compression: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultResult {
    pub path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultMetaResult {
    pub name: String,
    pub path: String,
    pub size_in_bytes: u64,
    pub version: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenVaultParams {
    pub path: String,
    pub password: Vec<u8>,
}
