# OpenVault V1 Architecture

This document describes the internal structure and design of the OpenVault V1 format.

## Overview

OpenVault V1 uses an **append-only, log-structured design**. The primary goal is to provide a secure, efficient, and flexible storage format that avoids expensive "file shifting" operations during updates.

## File Layout

A vault file is structured as follows:

```text
┌─────────────────────────────────────────────────────────────────┐
│  HEADER (fixed bytes)                                           │
│    - magic: "OPENV0"                                            │
│    - metadata_offset: Points to the Metadata Block (at EOF)     │
│    - metadata_size: Size of the metadata                        │
├─────────────────────────────────────────────────────────────────┤
│  BLOB REGION (variable size)                                    │
│    [Blob 0] [Blob 1] [DEAD SPACE] [Blob 2] ...                  │
├─────────────────────────────────────────────────────────────────┤
│  METADATA BLOCK (at EOF)                                        │
│    - Encrypted VaultMeta (JSON/Postcard)                        │
└─────────────────────────────────────────────────────────────────┘
```

### 1. The Header (Fixed Size)

The header is always fixed size and located at the very start of the file. It contains the essential entry points needed to "boot" the vault. Crucially, it stores the **offset** and **size** for the Metadata Block.

### 2. The Blob Region

Everything in OpenVault is treated as a **Blob**.

- **Files**: Encrypted (and optionally compressed) file contents.
- **Notes**: Encrypted text/markdown content.
- **Logs**: Encrypted audit trails.
- **Secrets**: Encrypted password/TOTP data.

Blobs are immutable and identified by their `offset` and `size` within the vault. Each blob is encrypted with its own unique nonce (usually prepended to the ciphertext).

### 3. Metadata Block (at EOF)

Instead of a central index that points to individual metadata chunks, OpenVault V1 stores the **entire** metadata state (`VaultMeta`) in a single encrypted block at the end of the file.

This block contains:

- **FileSystem**: All folder/file entries and their blob references.
- **Notes**: Titles, timestamps, and blob references to content.
- **Logs**: Typed events and blob references.
- **Secrets**: Labels, types, and blob references to credential data.

---

## Core Operations

### Unified Metadata Logic

Because metadata is stored at the end of the file, it can grow indefinitely. When any metadata changes (e.g., adding a file, updating a note), the system:

1. Appends any new blobs required.
2. Serializes the entire new `VaultMeta`.
3. Writes the new metadata block at the **new** end of the file (overwriting the old metadata space or appending after new blobs).
4. Updates the header to point to the new location.

### Soft Deletion

When a file, note, or secret is deleted:

1. The entry in `VaultMeta` is marked with `deleted: true`.
2. The metadata block is rewritten at EOF.
3. The actual blob data remains in the "Blob Region" as **dead space**. This makes deletions instantaneous regardless of file size.

### Update Strategy (Immutable Blobs)

If a note or secret is updated and its content grows:

1. A new blob is appended at the end of the current blob region.
2. The `NoteMeta`/`SecretMeta` is updated to point to the new blob offset.
3. The metadata block is rewritten at EOF.
4. The old blob becomes dead space.

### Compaction

Over time, dead space accumulates from deletions and updates. A **Compaction** operation can be triggered to:

1. Read only the "live" (non-deleted) entries from metadata.
2. Rewrite the vault file from scratch, copying only the live blobs.
3. Re-index all offsets to be sequential.
4. This results in the smallest possible vault size.

---

## Security Model

- **Master Key**: Derived from password + salt via Argon2id.
- **Authenticated Encryption**: Uses XChaCha20Poly1305 for all data (metadata and blobs).
- **Metadata Protection**: Metadata is never leaked; even the structure of the filesystem is encrypted. Someone without the password only sees a file full of random-looking bytes.
