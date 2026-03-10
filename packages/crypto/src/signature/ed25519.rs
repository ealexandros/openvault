use ed25519_dalek::{Signature, Signer as DalekSigner, Verifier};
use ed25519_dalek::{SigningKey, VerifyingKey};

use crate::keys::signing::SignatureKeyType;
use crate::signature::Signer;

#[derive(Debug, Default)]
pub struct Ed25519Signer;

impl Signer for Ed25519Signer {
    fn sign(&self, private: &SignatureKeyType, message: &[u8]) -> Vec<u8> {
        let signing_key = SigningKey::from_bytes(private);
        let signature = signing_key.sign(message);
        signature.to_bytes().to_vec()
    }

    fn verify(&self, public: &SignatureKeyType, message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }

        let Ok(verifying_key) = VerifyingKey::from_bytes(public) else {
            return false;
        };

        let signature = Signature::from_bytes(signature.try_into().unwrap());

        verifying_key.verify(message, &signature).is_ok()
    }
}
