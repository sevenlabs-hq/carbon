use juniper::{GraphQLScalar, InputValue, ScalarValue, Value};
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey as SolanaPubkey;
use std::{
    fmt::{self, Display},
    ops::Deref,
    str::FromStr,
};

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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String))]
pub struct I64(pub i64);

impl I64 {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .ok_or_else(|| "Expected a string".to_string())
            .and_then(|s| {
                i64::from_str(s)
                    .map(I64)
                    .map_err(|_| "Invalid i64".to_string())
            })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String))]
pub struct I128(pub i128);

impl I128 {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .ok_or_else(|| "Expected a string".to_string())
            .and_then(|s| {
                i128::from_str(s)
                    .map(I128)
                    .map_err(|_| "Invalid i128".to_string())
            })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String))]
pub struct U8(pub u8);

impl U8 {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .ok_or_else(|| "Expected a string".to_string())
            .and_then(|s| {
                u8::from_str(s)
                    .map(U8)
                    .map_err(|_| "Invalid u8".to_string())
            })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String))]
pub struct U32(pub u32);

impl U32 {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .ok_or_else(|| "Expected a string".to_string())
            .and_then(|s| {
                u32::from_str(s)
                    .map(U32)
                    .map_err(|_| "Invalid u32".to_string())
            })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String))]
pub struct U64(pub u64);

impl U64 {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .ok_or_else(|| "Expected a string".to_string())
            .and_then(|s| {
                u64::from_str(s)
                    .map(U64)
                    .map_err(|_| "Invalid u64".to_string())
            })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String))]
pub struct U128(pub u128);

impl U128 {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .ok_or_else(|| "Expected a string".to_string())
            .and_then(|s| {
                u128::from_str(s)
                    .map(U128)
                    .map_err(|_| "Invalid u128".to_string())
            })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String), description = "Arbitrary JSON value")]
pub struct Json(pub serde_json::Value);

impl Json {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        json_to_graphql_value::<S>(&self.0)
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        // Accept JSON provided as a string; other inputs are unsupported
        v.as_string_value()
            .ok_or_else(|| "Expected JSON string".to_string())
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).map_err(|e| e.to_string()))
            .map(Json)
    }
}

fn json_to_graphql_value<S: ScalarValue>(v: &serde_json::Value) -> Value<S> {
    match v {
        serde_json::Value::Null => Value::null(),
        serde_json::Value::Bool(b) => Value::scalar(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if let Ok(ii) = i32::try_from(i) {
                    Value::scalar(ii)
                } else {
                    // Out of i32 range; render losslessly as string
                    Value::scalar(i.to_string())
                }
            } else if let Some(u) = n.as_u64() {
                // Represent u64 losslessly as string
                Value::scalar(u.to_string())
            } else if let Some(f) = n.as_f64() {
                Value::scalar(f)
            } else {
                Value::null()
            }
        }
        serde_json::Value::String(s) => Value::scalar(s.clone()),
        serde_json::Value::Array(a) => {
            Value::list(a.iter().map(json_to_graphql_value::<S>).collect())
        }
        serde_json::Value::Object(obj) => Value::object(
            obj.iter()
                .map(|(k, v)| (k.clone(), json_to_graphql_value::<S>(v)))
                .collect(),
        ),
    }
}
