use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SealedRecord {
    pub media_sha256: String,
    pub prev_hash: String,
    pub signer_id: String,
    pub signature: String,
    pub public_key: String,
    pub timestamp: String,
    pub capture_metadata: serde_json::Value,
}
