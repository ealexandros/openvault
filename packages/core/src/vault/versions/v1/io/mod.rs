pub mod aad;
pub mod blob;
pub mod checkpoint;
pub mod frame;
pub mod record;
pub mod subheader;

pub use blob::{read_blob, write_blob};
pub use checkpoint::{read_checkpoint, write_checkpoint};
pub use record::{append_record, read_record};
pub use subheader::{read_subheader, write_subheader};

use crate::errors::Result;
use crate::internal::io_ext::Rw;
use crate::vault::crypto::keyring::Keyring;
use crate::vault::versions::shared::subheader::Subheader;

pub fn init_layout(rw: &mut Rw, keyring: &Keyring) -> Result<Subheader> {
    let subheader = Subheader::default();
    write_subheader(rw, &subheader, keyring)?;
    Ok(subheader)
}
