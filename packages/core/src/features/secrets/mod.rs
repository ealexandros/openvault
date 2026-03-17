mod codec;
mod error;
mod indexes;
mod models;
mod patch;
mod records;
mod store;
mod validate;

pub use codec::{SECRETS_FEATURE_ID, SecretCodec};
pub use error::{Result, SecretError};
pub use models::{
    EncryptedTotp, LoginEntry, LoginEntryView, NewLoginSecret, SECRETS_ROOT_FOLDER_ID,
    SECRETS_ROOT_FOLDER_NAME, SealedValue, SecretFolder, TOTP,
};
pub use patch::{LoginEntryPatch, NewLoginSecretPatch, SecretFolderPatch};
pub use records::{SECRETS_WIRE_VERSION, SecretDelta, SecretSnapshot, SecretsChange};
pub use store::SecretStore;
