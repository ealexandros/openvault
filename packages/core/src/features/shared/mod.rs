pub mod blob_ref;
pub mod codec;

pub use blob_ref::BlobRef;
pub use codec::FeatureCodec;

pub const DEFAULT_SNAPSHOT_THRESHOLD: usize = 64;
