use std::{
    fmt::{self, Display},
    ops::Deref,
    str::FromStr,
};

use juniper::{GraphQLScalar, InputValue, ScalarValue, Value};
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey as SolanaPubkey;

#[derive(Clone, Debug, Deserialize, Eq, GraphQLScalar, PartialEq, Serialize)]
#[graphql(parse_token(String), description = "A Solana's public key")]
pub struct Pubkey(pub SolanaPubkey);

impl Pubkey {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Pubkey::from(value.into())
    }

    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .map(str::to_owned)
            .or_else(|| v.as_int_value().as_ref().map(ToString::to_string))
            .map(|s| {
                SolanaPubkey::from_str(&s)
                    .map_err(|e| e.to_string())
                    .map(Self)
            })
            .ok_or_else(|| format!("Expected `String`, found: {v}"))?
    }
}

impl From<String> for Pubkey {
    fn from(s: String) -> Pubkey {
        Pubkey(SolanaPubkey::from_str(&s).expect("invalid pubkey"))
    }
}

impl Deref for Pubkey {
    type Target = str;

    fn deref(&self) -> &str {
        std::str::from_utf8(self.0.as_ref()).expect("invalid pubkey")
    }
}

impl Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
