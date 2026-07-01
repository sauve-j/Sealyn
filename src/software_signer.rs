use crate::signer::{Signer, SignerError};
use p256::ecdsa::{signature::hazmat::PrehashSigner, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;

pub struct SoftwareSigner {
    id: String,
    signing_key: SigningKey,
}

impl SoftwareSigner {
    /// Generate a fresh random P-256 key.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            signing_key: SigningKey::random(&mut OsRng),
        }
    }

    /// The verifying (public) key
    pub fn verifying_key(&self) -> VerifyingKey {
        *self.signing_key.verifying_key()
    }
}

impl Signer for SoftwareSigner {
    fn sign(&self, digest: &[u8; 32]) -> Result<Vec<u8>, SignerError> {
        let signature: Signature = self
            .signing_key
            .sign_prehash(digest)
            .map_err(|e| SignerError::Backend(e.to_string()))?;
        Ok(signature.to_bytes().to_vec())
    }

    fn public_key(&self) -> Vec<u8> {
        self.verifying_key().to_encoded_point(false).as_bytes().to_vec()
    }

    fn signer_id(&self) -> String {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sha256, verify};

    #[test]
    fn sign_then_verify_succeeds() {
        let signer = SoftwareSigner::new("sw-p256-0001");
        let digest = sha256(b"evidence.jpg");
        let sig = signer.sign(&digest).unwrap();
        assert!(verify(&signer.public_key(), &digest, &sig));
    }

    #[test]
    fn tampered_digest_fails_verification() {
        let signer = SoftwareSigner::new("sw-p256-0001");
        let digest = sha256(b"evidence.jpg");
        let sig = signer.sign(&digest).unwrap();
        let tampered = sha256(b"evidence.jpg (edited)");
        assert!(!verify(&signer.public_key(), &tampered, &sig));
    }

    #[test]
    fn wrong_key_fails_verification() {
        let a = SoftwareSigner::new("a");
        let b = SoftwareSigner::new("b");
        let digest = sha256(b"evidence.jpg");
        let sig = a.sign(&digest).unwrap();
        assert!(!verify(&b.public_key(), &digest, &sig));
    }
}
