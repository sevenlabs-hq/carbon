use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Token")]
pub struct TokenGraphQL {
    pub v: U64,
}

impl From<crate::types::Token> for TokenGraphQL {
    fn from(original: crate::types::Token) -> Self {
        Self {
            v: carbon_core::graphql::primitives::U64(original.v),
        }
    }
}
