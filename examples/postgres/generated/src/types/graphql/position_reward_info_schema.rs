use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "PositionRewardInfo")]
pub struct PositionRewardInfoGraphQL {
        pub growth_inside_last_x64: U128,
        pub reward_amount_owed: U64,
}

impl From<crate::types::PositionRewardInfo> for PositionRewardInfoGraphQL {
    fn from(original: crate::types::PositionRewardInfo) -> Self {
        Self {
            growth_inside_last_x64: carbon_core::graphql::primitives::U128(original.growth_inside_last_x64),
            reward_amount_owed: carbon_core::graphql::primitives::U64(original.reward_amount_owed),
        }
    }
}