use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CollectFundFee")]
pub struct CollectFundFeeGraphQL {
        pub amount0_requested: U64,
        pub amount1_requested: U64,
}

impl From<crate::instructions::postgres::CollectFundFeeRow> for CollectFundFeeGraphQL {
    fn from(row: crate::instructions::postgres::CollectFundFeeRow) -> Self {
        Self {
            amount0_requested: carbon_core::graphql::primitives::U64(*row.amount0_requested),
            amount1_requested: carbon_core::graphql::primitives::U64(*row.amount1_requested),
        }
    }
}