use crate::domain::records::{SecretDelta, SecretsChange, Snapshot};
use crate::domain::secrets::crypto::EncryptedField;
use crate::domain::secrets::login::{LoginEntry, LoginEntryPatch};
use crate::domain::secrets::totp::TOTP;
use crate::errors::{Result, SecretError};

use super::changes::{
    LoginEntryPatchV1, LoginEntryV1, SecretDeltaV1, SecretsChangeV1, SecretsChunkV1, SnapshotV1,
    TotpV1, WIRE_VERSION_V1,
};

pub fn encode_changes(changes: Vec<SecretsChange>) -> SecretsChunkV1 {
    let changes = changes.into_iter().map(to_wire_change).collect();
    SecretsChunkV1::new(changes)
}

pub fn decode_chunk(chunk: SecretsChunkV1) -> Result<Vec<SecretsChange>> {
    if chunk.version != WIRE_VERSION_V1 {
        return Err(SecretError::InvalidInput(format!(
            "Unsupported secrets wire version: {}",
            chunk.version
        )));
    }

    Ok(chunk.changes.into_iter().map(to_domain_change).collect())
}

fn to_wire_change(change: SecretsChange) -> SecretsChangeV1 {
    match change {
        SecretsChange::Snapshot(snapshot) => SecretsChangeV1::Snapshot(SnapshotV1 {
            entries: snapshot
                .entries
                .into_iter()
                .map(|(id, entry)| (id, to_wire_entry(&entry)))
                .collect(),
        }),
        SecretsChange::Deltas(deltas) => {
            SecretsChangeV1::Deltas(deltas.into_iter().map(to_wire_delta).collect())
        }
    }
}

fn to_domain_change(change: SecretsChangeV1) -> SecretsChange {
    match change {
        SecretsChangeV1::Snapshot(snapshot) => SecretsChange::Snapshot(Snapshot {
            entries: snapshot
                .entries
                .into_iter()
                .map(|(id, entry)| (id, to_domain_entry(entry)))
                .collect(),
        }),
        SecretsChangeV1::Deltas(deltas) => {
            SecretsChange::Deltas(deltas.into_iter().map(to_domain_delta).collect())
        }
    }
}

fn to_wire_delta(delta: SecretDelta) -> SecretDeltaV1 {
    match delta {
        SecretDelta::Added(entry) => SecretDeltaV1::Added(to_wire_entry(&entry)),
        SecretDelta::Updated { id, patch } => SecretDeltaV1::Updated {
            id,
            patch: to_wire_patch(&patch),
        },
        SecretDelta::Deleted { id } => SecretDeltaV1::Deleted { id },
    }
}

fn to_domain_delta(delta: SecretDeltaV1) -> SecretDelta {
    match delta {
        SecretDeltaV1::Added(entry) => SecretDelta::Added(to_domain_entry(entry)),
        SecretDeltaV1::Updated { id, patch } => SecretDelta::Updated {
            id,
            patch: to_domain_patch(patch),
        },
        SecretDeltaV1::Deleted { id } => SecretDelta::Deleted { id },
    }
}

fn to_wire_entry(entry: &LoginEntry) -> LoginEntryV1 {
    LoginEntryV1 {
        id: entry.id,
        folder: entry.folder.clone(),
        name: entry.name.clone(),
        username: entry.username.clone(),
        password: entry.password.as_bytes().to_vec(),
        website: entry.website.clone(),
        comments: entry.comments.clone(),
        created_at: entry.created_at,
        updated_at: entry.updated_at,
        totp: entry.totp.as_ref().map(to_wire_totp),
    }
}

fn to_domain_entry(entry: LoginEntryV1) -> LoginEntry {
    LoginEntry {
        id: entry.id,
        folder: entry.folder,
        name: entry.name,
        username: entry.username,
        password: EncryptedField::new(entry.password),
        website: entry.website,
        comments: entry.comments,
        created_at: entry.created_at,
        updated_at: entry.updated_at,
        totp: entry.totp.map(to_domain_totp),
    }
}

fn to_wire_patch(patch: &LoginEntryPatch) -> LoginEntryPatchV1 {
    LoginEntryPatchV1 {
        folder: patch.folder.clone(),
        name: patch.name.clone(),
        username: patch.username.clone(),
        password: patch.password.as_ref().map(|f| f.as_bytes().to_vec()),
        website: patch.website.clone(),
        comments: patch.comments.clone(),
        totp: patch
            .totp
            .as_ref()
            .map(|nested| nested.as_ref().map(to_wire_totp)),
        updated_at: patch.updated_at,
    }
}

fn to_domain_patch(patch: LoginEntryPatchV1) -> LoginEntryPatch {
    LoginEntryPatch {
        folder: patch.folder,
        name: patch.name,
        username: patch.username,
        password: patch.password.map(EncryptedField::new),
        website: patch.website,
        comments: patch.comments,
        totp: patch.totp.map(|nested| nested.map(to_domain_totp)),
        updated_at: patch.updated_at,
    }
}

fn to_wire_totp(totp: &TOTP) -> TotpV1 {
    TotpV1 {
        secret: totp.secret.clone(),
        period: totp.period,
        digits: totp.digits,
    }
}

fn to_domain_totp(totp: TotpV1) -> TOTP {
    TOTP {
        secret: totp.secret,
        period: totp.period,
        digits: totp.digits,
    }
}
