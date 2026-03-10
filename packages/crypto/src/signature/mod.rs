pub mod ed25519;

pub use ed25519::Ed25519Signer;

use crate::keys::{SigningPrivateKey, SigningPublicKey};

pub trait Signer {
    fn sign(private: &SigningPrivateKey, message: &[u8]) -> Vec<u8>;
    fn verify(public: &SigningPublicKey, message: &[u8], signature: &[u8]) -> bool;
}
