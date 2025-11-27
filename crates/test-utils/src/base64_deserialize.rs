use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Deserializer, de};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let field_string = String::deserialize(deserializer)?;
    STANDARD
        .decode(field_string)
        .map_err(|e| de::Error::custom(format!("base64 decoding error: {:?}", e)))
}
