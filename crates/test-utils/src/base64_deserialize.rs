use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{de, Deserializer};

use super::*;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let field_string = String::deserialize(deserializer)?;
    STANDARD
        .decode(field_string)
        .map_err(|e| de::Error::custom(format!("base64 decoding error: {:?}", e)))
}
