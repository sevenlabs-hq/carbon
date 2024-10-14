use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use serde_json::{Number, Value};

/// Deserializes a `String` to a `Number`
///
/// # Arguments
/// * `deserializer` - The deserializer instance from which to read the JSON value
///
/// # Returns a `Result` that is:
/// - `Ok(Number)` when the input is either a valid `Number` string or a JSON `Number`
/// - `Err(D::Error)` when there's an error from the deserializer if the input is neither a stringified number nor a direct `Number`,
///  or if the `String` cannot be parsed into a `Number`
pub fn deserialize_str_to_number<'de, D>(deserializer: D) -> Result<Number, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(s) => s.parse::<Number>().map_err(SerdeError::custom),
        Value::Number(n) => Ok(n),
        _ => Err(SerdeError::custom("Expected a string or a number")),
    }
}
