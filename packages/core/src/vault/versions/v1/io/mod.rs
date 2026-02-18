pub mod aad;
pub mod checkpoint;
pub mod record;
pub mod subheader;

pub use checkpoint::{read_checkpoint, write_checkpoint};
pub use record::{append_record, read_record, replay_from};
pub use subheader::{read_subheader, write_subheader};

use crate::errors::Result;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::subheader::Subheader;
use crate::vault::versions::shared::traits::WriteSeek;

pub fn init_layout(writer: &mut dyn WriteSeek, keyring: &Keyring) -> Result<Subheader> {
    let subheader = Subheader::default();
    write_subheader(writer, &subheader, keyring)?;
    Ok(subheader)
}
