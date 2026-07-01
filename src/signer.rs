use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("signing backend failed: {0}")]
    Backend(String),
}

pub trait Signer {
    fn sign(&self, digest: &[u8; 32]) -> Result<Vec<u8>, SignerError>;

    fn public_key(&self) -> Vec<u8>;

    fn signer_id(&self) -> String;
}
