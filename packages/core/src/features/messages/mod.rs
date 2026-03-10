pub mod store;

mod codec;
mod crypto;
mod error;
mod events;
mod models;
mod patch;

pub use codec::{MESSAGES_WIRE_VERSION, MessagesCodec};
pub use crypto::{open_message, seal_message};
pub use error::{MessagesError, Result};
pub use events::{MessagesChange, MessagesDelta, MessagesSnapshot};
pub use models::{MessageContact, MessageCredentials, MessageCredentialsView};
pub use patch::MessageContactPatch;
pub use store::MessagesStore;
