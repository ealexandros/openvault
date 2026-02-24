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
use crate::internal::io_ext::ReadWriter;
use crate::vault::versions::shared::subheader::Subheader;
use crate::vault::versions::shared::traits::FormatContext;

pub fn init_layout(rw: &mut ReadWriter, context: &FormatContext) -> Result<Subheader> {
    let subheader = Subheader::default();
    write_subheader(rw, &subheader, context)?;
    Ok(subheader)
}
