pub mod store;

mod codec;
mod crypto;
mod error;
mod events;
mod models;

pub use codec::{MESSAGES_WIRE_VERSION, MessagesCodec};
pub use crypto::{MessageEnvelope, open_message, seal_message};
pub use error::{MessagesError, Result};
pub use events::{MessagesChange, MessagesDelta, MessagesSnapshot};
pub use models::{MessageContact, MessageContactPatch, MessageCredentials};
pub use store::MessagesStore;
