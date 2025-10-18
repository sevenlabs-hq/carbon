use carbon_core::graphql::primitives::U128;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "FixedPoint")]
pub struct FixedPointGraphQL {
    pub v: U128,
}

impl From<crate::types::FixedPoint> for FixedPointGraphQL {
    fn from(original: crate::types::FixedPoint) -> Self {
        Self {
            v: carbon_core::graphql::primitives::U128(original.v),
        }
    }
}
