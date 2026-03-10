use ed25519_dalek::{Signature, Signer as DalekSigner, Verifier};
use ed25519_dalek::{SigningKey, VerifyingKey};

use crate::keys::signing::{SigningPrivateKey, SigningPublicKey};
use crate::signature::Signer;

pub struct Ed25519Signer;

impl Signer for Ed25519Signer {
    fn sign(private: &SigningPrivateKey, message: &[u8]) -> Vec<u8> {
        let signing_key = SigningKey::from_bytes(private.as_bytes());
        let signature = signing_key.sign(message);
        signature.to_bytes().to_vec()
    }

    fn verify(public: &SigningPublicKey, message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }

        let Ok(verifying_key) = VerifyingKey::from_bytes(public.as_bytes()) else {
            return false;
        };

        let signature = Signature::from_bytes(signature.try_into().unwrap());

        verifying_key.verify(message, &signature).is_ok()
    }
}
