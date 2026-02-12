# OpenVault Secrets

This package implements the core logic for managing sensitive data, including secure storage, retrieval, and cryptographic operations for passwords and other secrets.

## Overview

The Secrets package acts as the centralized manager for user credentials and confidential information. It maps vault bytes into secret domain structs and exports updated bytes for persistence.

## Key Components

- **Secret Manager**: The primary entry point for interacting with the vault. It handles unlocking, adding, updating, and retrieving secrets.
- **Vault**: An in-memory domain aggregate that manages snapshots, deltas, and folder-oriented indexing.
- **Domain Models**: Strongly typed structures for `SecretEntry`, `TOTP`, metadata, and folders.

## Usage

### Initialize & Unlock

```rust
use openvault_crypto::keys::MasterKey;
use openvault_secrets::SecretManager;

// Unlock an existing vault or create a new one
let master_key = MasterKey::derive(b"password", &salt)?;
let manager = SecretManager::unlock(master_key, encrypted_chunks)?;
```

### Manage Secrets

```rust
use openvault_crypto::keys::MasterKey;
use openvault_secrets::manager::params::AddSecretEntryParams;
use openvault_secrets::SecretManager;

let key = MasterKey::new([0u8; 32]);
let mut manager = SecretManager::create(key);

let id = manager.add(AddSecretEntryParams {
    folder: "/work".to_string(),
    name: "github".to_string(),
    username: "alex".to_string(),
    password: "s3cr3t".to_string(),
    website: "https://github.com".to_string(),
    comments: "".to_string(),
    totp: None,
})?;

// Retrieve a secret view (safe for UI)
if let Some(view) = manager.get(&id) {
    println!("User: {:?}", view.username);
}

// Reveal sensitive password (decrypted on demand)
let password = manager.show_password(&id)?;
```

### Export Data

```rust
// Export the full vault snapshot encrypted
let encrypted_blob = manager.export()?;

// or export only pending changes (deltas)
let encrypted_deltas = manager.export_changes()?;
```
