# Sealyn — Record Schema (v1)

Each sealed item becomes one record with these fields:

| Field | Type | Purpose |
|---|---|---|
| `media_sha256` | hex string | SHA-256 fingerprint of the captured file |
| `prev_hash` | hex string | Hash of the previous record, or `"genesis"` |
| `signer_id` | string | Which signer/device sealed this record |
| `signature` | hex string | Signature over the record's other fields |
| `public_key` | hex string | Signer's public key (self-verifying) |
| `timestamp` | RFC 3339 string | Capture time (RFC 3161 token in a later version) |
| `capture_metadata` | JSON object | Free-form context (operator note, device, GPS…) |

Example record:

```json
{
  "media_sha256": "6bf78f5a247f72cb5c71a6a2e3065f51c458e7aaf79df89bdadbd081c384a0d9",
  "prev_hash": "genesis",
  "signer_id": "dev-sw-0001",
  "signature": "3045022100...",
  "public_key": "04a1b2...",
  "timestamp": "2026-07-01T12:00:00Z",
  "capture_metadata": { "operator": "field note", "device": "prototype-1" }
}
```

Notes:
- The `signature` is computed over all fields *except* `signature` itself.
- Each record's hash (used as the next record's `prev_hash`) is computed over its canonical contents, forming a tamper-evident chain.