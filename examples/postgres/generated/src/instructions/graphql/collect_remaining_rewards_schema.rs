use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CollectRemainingRewards")]
pub struct CollectRemainingRewardsGraphQL {
        pub reward_index: U8,
}

impl From<crate::instructions::postgres::CollectRemainingRewardsRow> for CollectRemainingRewardsGraphQL {
    fn from(row: crate::instructions::postgres::CollectRemainingRewardsRow) -> Self {
        Self {
            reward_index: carbon_core::graphql::primitives::U8((*row.reward_index) as u8),
        }
    }
}