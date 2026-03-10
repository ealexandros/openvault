use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialsParams {
    pub name: String,
    pub expires_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddContactParams {
    pub name: String,
    pub signing_pub_key: Vec<u8>,
    pub ephemeral_pub_key: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameContactParams {
    pub id: String,
    pub new_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveContactParams {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptMessageParams {
    pub id: String,
    pub payload: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptMessageParams {
    pub id: String,
    pub payload: Vec<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContactItem {
    pub id: String,
    pub name: String,
    pub signing_pub_key: Vec<u8>,
    pub ephemeral_pub_key: Vec<u8>,
    pub secure: bool,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageCredentialsItem {
    pub name: String,
    pub signing_pub_key: Vec<u8>,
    pub ephemeral_pub_key: Vec<u8>,
    pub secure: bool,
    pub expires_at: Option<String>,
}
