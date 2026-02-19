# OpenVault Architecture V1

## 1. Package Model

OpenVault is split into clear layers:

1. `packages/crypto`

- Cryptographic primitives only (Argon2id, HKDF, AEAD, compression).
- No vault-domain policy.

2. `packages/core`

- Vault container engine.
- Owns file layout, version handling, append/replay/compaction, and key policy.
- Owns frame/envelope wiring and persistence invariants.

3. Feature modules/crates (`secrets`, `notes`, `filesystem`, `logs`)

- Own domain logic and validation.
- Own payload encode/decode schema and feature payload versions.
- Emit `snapshot` and `delta` changes.

4. `packages/sdk` (facade)

- App-facing API used by Tauri.
- Exposes unified operations: `open`, `close`, `compact`, and feature accessors.

## 2. Versioning Model

OpenVault uses two independent version axes:

1. `version` (core/container)

- How to read the vault file structure.

2. `feature_payload_version` (per feature)

- How to decode one feature payload (`secrets`, `notes`, etc).

This allows feature schema evolution without forcing a vault format bump.

## 3. Vault File Layout (V1)

```text
[Boot Header - clear]
  magic
  version
  salt

[Subheader Frame - encrypted payload]
  checkpoint_offset
  delta_offset

[Append Region]
  checkpoint frames
  record frames
```

Frame wire format:

```text
[Frame Prefix - clear]
  ciphertext_size (u32, LE)
  nonce (24 bytes)

[Ciphertext - encrypted]
  domain-specific plaintext bytes
```

Important invariant:

1. Frame header nonce is the single source of truth for nonce in core paths.
2. Ciphertext does not carry a second nonce copy in core frame persistence.

## 4. AAD Model (V1)

AAD is used for context binding (integrity of cleartext context), not secrecy.

Current V1 AAD encoding:

1. Domain byte (`Subheader`, `Checkpoint`, `Record`)
2. Frame offset (`u64`, LE)
3. Version prefix (`openvault/v1/...` namespace bytes)

This binds ciphertext usage to both logical domain and position.

## 5. Core Terms

### Record

A record is the logical event envelope stored in the append log:

1. `kind`
2. `feature_id`
3. `payload_version`
4. `sequence`
5. `prev_offset`
6. `payload_size`
7. `key_epoch`

Record plaintext for storage is encoded as `{ record, payload }` before encryption.

### Frame

A frame is the physical disk wrapper for encrypted bytes:

1. cleartext boundaries (`ciphertext_size`)
2. nonce
3. ciphertext payload

### Envelope

Envelope is the wrapping process used before persistence:

1. For records/checkpoints: compress -> encrypt.
2. For subheader: encrypt-only (no compression) to keep in-place rewrite size stable.

## 6. Key Model (V1)

1. Derive `MasterKey` from password + per-vault salt (Argon2id).
2. Derive purpose-specific keys from master key (HKDF labels):

- Core envelope key.
- Feature-purpose keys when needed.

3. Nonce must be unique per encryption operation.

## 7. Pointer Model (V1)

Subheader currently stores:

1. `checkpoint_offset`
2. `delta_offset`

Write behavior:

1. `write_checkpoint` appends checkpoint frame, then rewrites subheader with new `checkpoint_offset`.
2. `append_record` appends record frame, then rewrites subheader with new `delta_offset`.

## 8. Runtime Model

On open:

1. Read boot header.
2. Derive keyring.
3. Read/decrypt subheader.
4. Read checkpoint if present.
5. Read/replay records from chosen start offset.
6. Build in-memory feature projections.

Sequence guidance:

1. Sequence is monotonic and does not reset on checkpoint.
2. Typical session counter: `next_sequence = last_known_sequence + 1`.

## 9. Operation Flows

### Add / Update / Delete

1. Feature validates command and emits delta/snapshot payload.
2. Core builds `Record`.
3. Core encodes `{record, payload}`, seals it, and appends frame.
4. Core updates subheader pointer(s).

### Checkpoint

1. Core serializes checkpoint payload.
2. Core seals and appends checkpoint frame.
3. Core updates subheader `checkpoint_offset`.

### Compact

Compaction remains deferred in current V1 implementation planning.

## 10. Current V1 Status

Implemented:

1. Subheader init/read/write.
2. Checkpoint append/read.
3. Record append/read.
4. Record replay scanner.
5. Offset-bound AAD generation and use.
6. Subheader pointer rewrite on checkpoint/record append.

Pending:

1. Compaction.
2. Final replay policy across mixed frame domains.
3. Sequence/checkpoint state integration at SDK session level.

## 11. Why This Architecture

1. Clear separation between container and feature concerns.
2. Independent evolution of vault format and feature payload schemas.
3. Append-only persistence keeps writes simple and auditable.
4. SDK can expose one stable app-facing API over evolving internals.
