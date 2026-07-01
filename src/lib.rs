//! Sealyn core library.

pub mod record;
pub mod signer;

pub use record::SealedRecord;
pub use signer::{Signer, SignerError};

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sha256_matches_nist_vector() {
        // Standard SHA-256("abc") test vector.
        assert_eq!(
            hex::encode(sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn record_roundtrips_through_json() {
        let r = SealedRecord {
            media_sha256: hex::encode(sha256(b"evidence.jpg")),
            prev_hash: "genesis".to_string(),
            signer_id: "dev-sw-0001".to_string(),
            signature: String::new(),
            public_key: String::new(),
            timestamp: "2026-07-01T12:00:00Z".to_string(),
            capture_metadata: json!({ "operator": "test", "note": "day 1 scaffold" }),
        };
        let s = serde_json::to_string(&r).unwrap();
        let back: SealedRecord = serde_json::from_str(&s).unwrap();
        assert_eq!(r, back);
    }
}
