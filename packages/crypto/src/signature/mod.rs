pub mod ed25519;
pub mod factory;

pub use ed25519::Ed25519Signer;
pub use factory::{SignatureAlgorithm, SignerRef};

use crate::keys::signing::SignatureKeyType;

pub trait Signer {
    fn sign(&self, private: &SignatureKeyType, message: &[u8]) -> Vec<u8>;
    fn verify(&self, public: &SignatureKeyType, message: &[u8], signature: &[u8]) -> bool;
}
