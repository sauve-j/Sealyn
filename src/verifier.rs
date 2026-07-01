use p256::ecdsa::{signature::hazmat::PrehashVerifier, Signature, VerifyingKey};

/// Verify a raw 64-byte ECDSA P-256 signature over a 32-byte digest.
pub fn verify(public_key: &[u8], digest: &[u8; 32], signature: &[u8]) -> bool {
    let Ok(vk) = VerifyingKey::from_sec1_bytes(public_key) else {
        return false;
    };
    let Ok(sig) = Signature::from_slice(signature) else {
        return false;
    };
    vk.verify_prehash(digest, &sig).is_ok()
}
