use juniper::{GraphQLScalar, InputValue, ScalarValue, Value};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
