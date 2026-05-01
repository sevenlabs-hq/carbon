//! Base58 helper for inline test data.
//!
//! Unlike the sibling `base64_deserialize` / `hex_deserialize` modules (used as
//! `#[serde(with = "...")]` adapters), this is a direct decoder for hand-pasted
//! base58 strings — convenient for embedding instruction data in test sources.

pub fn ix_data(ix: &str) -> Vec<u8> {
    bs58::decode(ix).into_vec().expect("hex decode failed")
}
