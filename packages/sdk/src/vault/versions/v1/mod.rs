pub mod io;
pub mod mapper;
pub mod records;
pub mod schema;

use openvault_crypto::keys::MasterKey;

use crate::domain::Vault;
use crate::errors::Result;
use crate::versions::shared::traits::{ReadSeek, VersionHandler, WriteSeek};

pub struct V1Handler;

// @todo-now implement this..

impl VersionHandler for V1Handler {
    fn read(&self, _reader: &mut dyn ReadSeek, _key: &MasterKey) -> Result<Vault> {
        todo!()
    }

    fn add_delta(&self, _writer: &mut dyn WriteSeek, _data: &[u8], _key: &MasterKey) -> Result {
        todo!()
    }

    fn add_snapshot(
        &self,
        _writer: &mut dyn WriteSeek,
        _vault: &Vault,
        _key: &MasterKey,
    ) -> Result {
        todo!()
    }
}
