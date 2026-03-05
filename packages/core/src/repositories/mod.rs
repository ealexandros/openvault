use crate::errors::Result;
use crate::features::shared::codec::FeatureCodec;
use crate::vault::runtime::VaultSession;

pub mod filesystem;

pub use filesystem::FilesystemRepository;

pub trait FeatureRepository {
    type Store;
    type Change;
    type Codec: FeatureCodec;

    fn load(session: &mut VaultSession) -> Result<Self::Store>;
    fn commit(session: &mut VaultSession, store: &mut Self::Store) -> Result;
}
