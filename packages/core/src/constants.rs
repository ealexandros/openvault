pub const VAULT_MAGIC: &[u8; 6] = b"OPENV0";
pub const VAULT_MAGIC_LEN: usize = VAULT_MAGIC.len();

pub const SALT_LEN: usize = 16;
pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 24;
pub const TAG_LEN: usize = 16;

pub const DEFAULT_COMPRESSION: &str = "zstd";
pub const DEFAULT_ENCRYPTION: &str = "xchacha20poly1305";
