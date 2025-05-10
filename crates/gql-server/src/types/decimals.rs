use juniper::{GraphQLScalar, InputValue, ScalarValue, Value};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, GraphQLScalar, PartialEq)]
#[graphql(parse_token(String))]
pub struct Decimals(pub u8);

impl Decimals {
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        Value::scalar(self.0.to_string())
    }

    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        v.as_string_value()
            .ok_or_else(|| "Expected a string".to_string())
            .and_then(|s| {
                u8::from_str(s)
                    .map(Decimals)
                    .map_err(|_| "Invalid decimals".to_string())
            })
    }
}
