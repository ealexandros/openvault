use crate::vault::shared::commands::VaultCommands;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum VaultVersion {
    V1,
}

impl VaultVersion {
    pub fn latest() -> Self {
        VaultVersion::V1
    }

    pub fn commands(self) -> Box<dyn VaultCommands> {
        match self {
            VaultVersion::V1 => Box::new(crate::vault::v1::commands::Commands),
        }
    }
}
