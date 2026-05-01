//! `#[serde(with = "hex_deserialize")]` adapter that decodes a JSON string
//! field into `Vec<u8>` via hex.

use {
    super::*,
    serde::{de, Deserializer},
};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let field_string = String::deserialize(deserializer)?;
    hex::decode(field_string).map_err(|e| de::Error::custom(format!("hex decoding error: {e:?}")))
}
