use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "DecreaseLiquidityV2")]
pub struct DecreaseLiquidityV2GraphQL {
        pub liquidity: U128,
        pub amount0_min: U64,
        pub amount1_min: U64,
}

impl From<crate::instructions::postgres::DecreaseLiquidityV2Row> for DecreaseLiquidityV2GraphQL {
    fn from(row: crate::instructions::postgres::DecreaseLiquidityV2Row) -> Self {
        Self {
            liquidity: carbon_core::graphql::primitives::U128(*row.liquidity),
            amount0_min: carbon_core::graphql::primitives::U64(*row.amount0_min),
            amount1_min: carbon_core::graphql::primitives::U64(*row.amount1_min),
        }
    }
}