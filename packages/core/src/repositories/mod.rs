use crate::errors::Result;
use crate::features::shared::FeatureCodec;
use crate::operations::replay::replay_since_checkpoint;
use crate::vault::features::FeatureType;
use crate::vault::runtime::VaultSession;
use crate::vault::versions::shared::checkpoint::CheckpointFeature;
use crate::vault::versions::shared::replay::ReplayState;

pub mod filesystem;

pub use filesystem::FilesystemRepository;

pub trait FeatureRepository {
    type Store;
    type Change;
    type Codec: FeatureCodec;

    fn restore_from_replay(state: &ReplayState) -> Result<Self::Store>;
    fn commit(session: &mut VaultSession, store: &mut Self::Store) -> Result<CommitOutcome>;
    fn create_checkpoint(store: &Self::Store) -> Result<CheckpointFeature>;

    fn load(session: &mut VaultSession) -> Result<Self::Store> {
        let replay = replay_since_checkpoint(session)?;
        Self::restore_from_replay(&replay)
    }
}

pub struct CommitOutcome {
    pub did_persist: bool,
    pub feature_type: FeatureType,
}

impl CommitOutcome {
    pub fn no_change(feature_type: FeatureType) -> Self {
        Self {
            did_persist: false,
            feature_type,
        }
    }

    pub fn persisted(feature_type: FeatureType) -> Self {
        Self {
            did_persist: true,
            feature_type,
        }
    }
}
