use openvault_sdk::MessageContact;
use openvault_sdk::{EphemeralPublicKey, SigningPublicKey};

use super::contracts::{
    AddContactParams, CreateCredentialsParams, DecryptFileParams, DecryptMessageParams,
    EncryptFileParams, EncryptMessageParams, MessageContactItem, MessageCredentialsItem,
    RemoveContactParams, RenameContactParams,
};
use crate::errors::{Error, Result};
use crate::internal::parser::{parse_optional_datetime, parse_uuid};
use crate::state::TauriState;

macro_rules! vault_messages {
    ($state:expr, mut $messages:ident, $vault:ident) => {
        let mut $vault = $state.vault.lock().unwrap();
        let $vault = $vault.as_mut().ok_or(Error::VaultNotOpened)?;
        let mut $messages = $vault.messages();
    };
    ($state:expr, $messages:ident, $vault:ident) => {
        let mut $vault = $state.vault.lock().unwrap();
        let $vault = $vault.as_mut().ok_or(Error::VaultNotOpened)?;
        let $messages = $vault.messages();
    };
}

#[tauri::command]
pub async fn get_message_credentials(
    state: TauriState<'_>,
) -> Result<Option<MessageCredentialsItem>> {
    vault_messages!(state, messages, vault);

    let credentials = messages.credentials().map(|creds| MessageCredentialsItem {
        name: creds.name,
        signing_pub_key: creds.signing_pub_key.as_bytes().to_vec(),
        ephemeral_pub_key: creds.ephemeral_pub_key.as_bytes().to_vec(),
        secure: creds.secure,
        expires_at: creds.expires_at.map(|d| d.to_rfc3339()),
    });

    Ok(credentials)
}

#[tauri::command]
pub async fn create_message_credentials(
    state: TauriState<'_>,
    params: CreateCredentialsParams,
) -> Result {
    vault_messages!(state, mut messages, vault);

    let expires_at = parse_optional_datetime(params.expires_at)?;
    messages.create_credentials(params.name, expires_at)?;

    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn renew_message_credentials(state: TauriState<'_>) -> Result {
    vault_messages!(state, mut messages, vault);

    messages.renew_credentials()?;
    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn reset_message_credentials(state: TauriState<'_>) -> Result {
    vault_messages!(state, mut messages, vault);

    messages.reset_credentials()?;
    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn list_contacts(state: TauriState<'_>) -> Result<Vec<MessageContactItem>> {
    vault_messages!(state, messages, vault);

    let contacts = messages
        .list_contacts()
        .into_iter()
        .map(|contact| MessageContactItem {
            id: contact.id.to_string(),
            name: contact.name,
            signing_pub_key: contact.signing_pub_key.as_bytes().to_vec(),
            ephemeral_pub_key: contact.ephemeral_pub_key.as_bytes().to_vec(),
            secure: contact.secure,
            expires_at: contact.expires_at.map(|d| d.to_rfc3339()),
            created_at: contact.created_at.to_rfc3339(),
        })
        .collect();

    Ok(contacts)
}

#[tauri::command]
pub async fn add_contact(state: TauriState<'_>, params: AddContactParams) -> Result<String> {
    vault_messages!(state, mut messages, vault);

    let signing_pub_key = SigningPublicKey::from_bytes(
        params
            .signing_pub_key
            .try_into()
            .map_err(|_| Error::InvalidInput("Invalid signing public key".to_string()))?,
    );

    let ephemeral_pub_key = EphemeralPublicKey::from_bytes(
        params
            .ephemeral_pub_key
            .try_into()
            .map_err(|_| Error::InvalidInput("Invalid ephemeral public key".to_string()))?,
    );

    let contact = MessageContact::new(params.name, signing_pub_key, ephemeral_pub_key, true, None);

    let id = messages.add_contact(contact)?;
    vault.commit()?;

    Ok(id.to_string())
}

#[tauri::command]
pub async fn rename_contact(state: TauriState<'_>, params: RenameContactParams) -> Result {
    vault_messages!(state, mut messages, vault);

    let id = parse_uuid(&params.id)?;
    messages.rename_contact(id, params.new_name)?;

    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn remove_contact(state: TauriState<'_>, params: RemoveContactParams) -> Result {
    vault_messages!(state, mut messages, vault);

    let id = parse_uuid(&params.id)?;
    messages.remove_contact(id)?;
    vault.commit()?;

    Ok(())
}

#[tauri::command]
pub async fn encrypt_message(
    state: TauriState<'_>,
    params: EncryptMessageParams,
) -> Result<String> {
    vault_messages!(state, messages, vault);

    let id = parse_uuid(&params.id)?;
    let encrypted = messages.encrypt_for_contact(id, params.payload.as_bytes())?;

    Ok(encrypted)
}

#[tauri::command]
pub async fn decrypt_message(
    state: TauriState<'_>,
    params: DecryptMessageParams,
) -> Result<String> {
    vault_messages!(state, messages, vault);

    let id = parse_uuid(&params.id)?;
    let decrypted = messages.decrypt_from_contact(id, params.payload.as_bytes())?;

    Ok(decrypted)
}

#[tauri::command]
pub async fn encrypt_file(state: TauriState<'_>, params: EncryptFileParams) -> Result {
    vault_messages!(state, messages, vault);

    let id = parse_uuid(&params.contact_id)?;

    let bytes = std::fs::read(&params.source_path).map_err(Error::Io)?;

    if bytes.len() > 10 * 1024 * 1024 {
        return Err(Error::InvalidInput("File too large (max 10MB)".to_string()));
    }

    let encrypted = messages.encrypt_for_contact(id, &bytes)?;

    std::fs::write(&params.destination_path, encrypted.as_bytes()).map_err(Error::Io)?;

    Ok(())
}

#[tauri::command]
pub async fn decrypt_file(state: TauriState<'_>, params: DecryptFileParams) -> Result {
    vault_messages!(state, messages, vault);

    let id = parse_uuid(&params.contact_id)?;

    let ciphertext = std::fs::read_to_string(&params.source_path).map_err(Error::Io)?;

    let decrypted = messages.decrypt_from_contact(id, ciphertext.as_bytes())?;

    std::fs::write(&params.destination_path, decrypted.as_bytes()).map_err(Error::Io)?;

    Ok(())
}
