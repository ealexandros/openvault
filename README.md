# OpenVault

<!-- @todo-now improve docs -->

## 🏗️ How it Works

OpenVault uses an append-only, log-structured design. All data—files, notes, logs, and secrets—are stored as encrypted **Blobs**. A single encrypted **Metadata Block** is stored at the end of the file (EOF), ensuring that metadata can grow freely without shifting existing data.

- **Everything is a Blob**: Unified handling for all data types.
- **Metadata at EOF**: Fast opens and flexible growth.
- **Soft Deletes**: Instant deletions by marking metadata, cleaned up via periodic compaction.

Detailed documentation can be found in [Architecture Docs](./docs/architecture.md).

## 🛠️ Getting Started

### Prerequisites

- [Bun](https://bun.com/docs/installation)
- [Rust](https://rust-lang.org/tools/install)

### Installation

```bash
bun install
```

### Development

To start the development environment for all packages:

```bash
bun dev
```

### Build

To build all packages for production:

```bash
bun build
```

### ✨ Conclustion

Made with ❤️ – Shared with the Community 🤲
