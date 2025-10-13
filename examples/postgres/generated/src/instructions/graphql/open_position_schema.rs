use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "OpenPosition")]
pub struct OpenPositionGraphQL {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        pub liquidity: U128,
        pub amount0_max: U64,
        pub amount1_max: U64,
}

impl From<crate::instructions::postgres::OpenPositionRow> for OpenPositionGraphQL {
    fn from(row: crate::instructions::postgres::OpenPositionRow) -> Self {
        Self {
            tick_lower_index: row.tick_lower_index,
            tick_upper_index: row.tick_upper_index,
            tick_array_lower_start_index: row.tick_array_lower_start_index,
            tick_array_upper_start_index: row.tick_array_upper_start_index,
            liquidity: carbon_core::graphql::primitives::U128(*row.liquidity),
            amount0_max: carbon_core::graphql::primitives::U64(*row.amount0_max),
            amount1_max: carbon_core::graphql::primitives::U64(*row.amount1_max),
        }
    }
}