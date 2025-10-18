use carbon_core::graphql::primitives::U128;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Product")]
pub struct ProductGraphQL {
    pub v: U128,
}

impl From<crate::types::Product> for ProductGraphQL {
    fn from(original: crate::types::Product) -> Self {
        Self {
            v: carbon_core::graphql::primitives::U128(original.v),
        }
    }
}
