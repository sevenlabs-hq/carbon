use carbon_core::graphql::primitives::Json;
use serde_json;

// Data-carrying enum exposed as Json in GraphQL
pub type AccountsTypeGraphQL = Json;

impl From<crate::types::AccountsType> for AccountsTypeGraphQL {
    fn from(original: crate::types::AccountsType) -> Self {
        carbon_core::graphql::primitives::Json(
            serde_json::to_value(&original).unwrap_or(serde_json::Value::Null),
        )
    }
}


