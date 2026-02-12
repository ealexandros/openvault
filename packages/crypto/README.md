# OpenVault Crypto

This package provides the core cryptographic primitives and utilities for the OpenVault ecosystem. It wraps proven cryptographic libraries to ensure secure key derivation, encryption, and data compression.

## Features

- **Key Derivation**: Implements **Argon2id** for password hashing and **HKDF** (HMAC-based Key Derivation Function) for key expansion.
- **Encryption**: Uses **XChaCha20Poly1305** for authenticated encryption with extended nonces.
- **Compression**: Integrated **Zstd** compression to optimize storage usage before encryption.
- **Memory Security**: Utilizes `zeroize` to securely clear sensitive data from memory when it is dropped.

## Usage

### Key Derivation

```rust
use openvault_crypto::keys::MasterKey;

// Derive a master key from a user password and salt
let salt = openvault_crypto::keys::generate_default_salt();
let master_key = MasterKey::derive(b"user_password", &salt)?;
```

### Encryption

```rust
use openvault_crypto::encryption::{factory, Cipher};

// Create a cipher instance
let cipher = factory::xchacha20poly1305();

// Encrypt data
let ciphertext = cipher.encrypt(master_key.as_bytes(), b"secret data")?;

// Decrypt data
let plaintext = cipher.decrypt(master_key.as_bytes(), &ciphertext)?;
```
