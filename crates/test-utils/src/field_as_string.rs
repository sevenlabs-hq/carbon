use {
    serde::Deserialize,
    serde::{de, Deserializer},
    std::str::FromStr,
};

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    D: Deserializer<'de>,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse()
        .map_err(|e| de::Error::custom(format!("Parse error: {e:?}")))
}
