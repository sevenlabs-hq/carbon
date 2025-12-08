use serde::{de, Deserializer};

use super::*;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let field_string = String::deserialize(deserializer)?;
    hex::decode(field_string).map_err(|e| de::Error::custom(format!("hex decoding error: {e:?}")))
}
