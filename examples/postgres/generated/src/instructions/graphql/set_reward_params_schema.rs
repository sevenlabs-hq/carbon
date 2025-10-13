use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SetRewardParams")]
pub struct SetRewardParamsGraphQL {
        pub reward_index: U8,
        pub emissions_per_second_x64: U128,
        pub open_time: U64,
        pub end_time: U64,
}

impl From<crate::instructions::postgres::SetRewardParamsRow> for SetRewardParamsGraphQL {
    fn from(row: crate::instructions::postgres::SetRewardParamsRow) -> Self {
        Self {
            reward_index: carbon_core::graphql::primitives::U8((*row.reward_index) as u8),
            emissions_per_second_x64: carbon_core::graphql::primitives::U128(*row.emissions_per_second_x64),
            open_time: carbon_core::graphql::primitives::U64(*row.open_time),
            end_time: carbon_core::graphql::primitives::U64(*row.end_time),
        }
    }
}