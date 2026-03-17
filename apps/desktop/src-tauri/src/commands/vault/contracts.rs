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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenVaultParams {
    pub path: String,
    pub password: Vec<u8>,
}
