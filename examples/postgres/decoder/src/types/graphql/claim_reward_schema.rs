use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ClaimReward")]
pub struct ClaimRewardGraphQL {
        pub lb_pair: Pubkey,
        pub position: Pubkey,
        pub owner: Pubkey,
        pub reward_index: U64,
        pub total_reward: U64,
}

impl From<crate::types::ClaimReward> for ClaimRewardGraphQL {
    fn from(original: crate::types::ClaimReward) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            owner: carbon_core::graphql::primitives::Pubkey(original.owner),
            reward_index: carbon_core::graphql::primitives::U64(original.reward_index),
            total_reward: carbon_core::graphql::primitives::U64(original.total_reward),
        }
    }
}