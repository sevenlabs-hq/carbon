use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeReward")]
pub struct InitializeRewardGraphQL {
        pub open_time: U64,
        pub end_time: U64,
        pub emissions_per_second_x64: U128,
}

impl From<crate::instructions::postgres::InitializeRewardRow> for InitializeRewardGraphQL {
    fn from(row: crate::instructions::postgres::InitializeRewardRow) -> Self {
        Self {
            open_time: carbon_core::graphql::primitives::U64(*row.open_time),
            end_time: carbon_core::graphql::primitives::U64(*row.end_time),
            emissions_per_second_x64: carbon_core::graphql::primitives::U128(*row.emissions_per_second_x64),
        }
    }
}