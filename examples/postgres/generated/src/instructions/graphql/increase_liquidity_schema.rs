use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "IncreaseLiquidity")]
pub struct IncreaseLiquidityGraphQL {
        pub liquidity: U128,
        pub amount0_max: U64,
        pub amount1_max: U64,
}

impl From<crate::instructions::postgres::IncreaseLiquidityRow> for IncreaseLiquidityGraphQL {
    fn from(row: crate::instructions::postgres::IncreaseLiquidityRow) -> Self {
        Self {
            liquidity: carbon_core::graphql::primitives::U128(*row.liquidity),
            amount0_max: carbon_core::graphql::primitives::U64(*row.amount0_max),
            amount1_max: carbon_core::graphql::primitives::U64(*row.amount1_max),
        }
    }
}