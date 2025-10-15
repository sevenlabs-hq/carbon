use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ProtocolFee")]
pub struct ProtocolFeeGraphQL {
        pub amount_x: U64,
        pub amount_y: U64,
}

impl From<crate::types::ProtocolFee> for ProtocolFeeGraphQL {
    fn from(original: crate::types::ProtocolFee) -> Self {
        Self {
            amount_x: carbon_core::graphql::primitives::U64(original.amount_x),
            amount_y: carbon_core::graphql::primitives::U64(original.amount_y),
        }
    }
}