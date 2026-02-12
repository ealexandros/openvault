# OpenVault Secrets

This package implements the core logic for managing sensitive data, including secure storage, retrieval, and cryptographic operations for passwords and other secrets.

## Overview

The Secrets package acts as the centralized manager for user credentials and confidential information. It handles encryption/decryption of the vault, secure memory management, and data serialization.

## Key Components

- **Secret Manager**: The primary entry point for interacting with the vault. It handles unlocking, adding, updating, and retrieving secrets.
- **Store**: An in-memory representation of the vault state, managing snapshots and delta updates for efficient synchronization.
- **Domain Models**: Strongly typed structures for `SecretEntry`, `TOTP`, and metadata.
- **Codec**: Handles serialization (using `postcard`) and encryption of the vault data.

## Usage

### Initialize & Unlock

```rust
use openvault_crypto::keys::MasterKey;
use openvault_secrets::manager::SecretManager;

// Unlock an existing vault or create a new one
let master_key = MasterKey::derive(b"password", &salt)?;
let manager = SecretManager::unlock(master_key, encrypted_chunks)?;
```

### Manage Secrets

```rust
use openvault_secrets::manager::params::AddSecretEntryParams;

// Add a new secret
manager.add(AddSecretEntryParams {
    name: "github.com".to_string(),
    username: Some("alex".to_string()),
    password: Some("s3cr3t".to_string()),
    ..Default::default()
})?;

// Retrieve a secret view (safe for UI)
if let Some(view) = manager.get("github.com") {
    println!("User: {:?}", view.username);
}

// Reveal sensitive password (decrypted on demand)
let password = manager.reveal_password("github.com")?;
```

### Export Data

```rust
// Export the full vault snapshot encrypted
let encrypted_blob = manager.export()?;

// or export only pending changes (deltas)
let encrypted_deltas = manager.export_changes()?;
```
