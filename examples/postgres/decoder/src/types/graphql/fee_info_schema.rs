use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "FeeInfo")]
pub struct FeeInfoGraphQL {
        pub fee_x_per_token_complete: U128,
        pub fee_y_per_token_complete: U128,
        pub fee_x_pending: U64,
        pub fee_y_pending: U64,
}

impl From<crate::types::FeeInfo> for FeeInfoGraphQL {
    fn from(original: crate::types::FeeInfo) -> Self {
        Self {
            fee_x_per_token_complete: carbon_core::graphql::primitives::U128(original.fee_x_per_token_complete),
            fee_y_per_token_complete: carbon_core::graphql::primitives::U128(original.fee_y_per_token_complete),
            fee_x_pending: carbon_core::graphql::primitives::U64(original.fee_x_pending),
            fee_y_pending: carbon_core::graphql::primitives::U64(original.fee_y_pending),
        }
    }
}