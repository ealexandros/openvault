use std::fmt::Debug;

/// Marker trait for vault states
pub trait VaultState: Debug + Send + Sync {}

/// State representing a vault that has not been decrypted.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Locked;

impl VaultState for Locked {}

/// State representing a decrypted vault.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Unlocked;

impl VaultState for Unlocked {}
