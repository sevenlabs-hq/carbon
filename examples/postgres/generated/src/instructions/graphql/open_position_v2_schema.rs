use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "OpenPositionV2")]
pub struct OpenPositionV2GraphQL {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        pub liquidity: U128,
        pub amount0_max: U64,
        pub amount1_max: U64,
        pub with_matedata: bool,
        pub base_flag: Option<bool>,
}

impl From<crate::instructions::postgres::OpenPositionV2Row> for OpenPositionV2GraphQL {
    fn from(row: crate::instructions::postgres::OpenPositionV2Row) -> Self {
        Self {
            tick_lower_index: row.tick_lower_index,
            tick_upper_index: row.tick_upper_index,
            tick_array_lower_start_index: row.tick_array_lower_start_index,
            tick_array_upper_start_index: row.tick_array_upper_start_index,
            liquidity: carbon_core::graphql::primitives::U128(*row.liquidity),
            amount0_max: carbon_core::graphql::primitives::U64(*row.amount0_max),
            amount1_max: carbon_core::graphql::primitives::U64(*row.amount1_max),
            with_matedata: row.with_matedata,
            base_flag: row.base_flag.map(|v| v),
        }
    }
}