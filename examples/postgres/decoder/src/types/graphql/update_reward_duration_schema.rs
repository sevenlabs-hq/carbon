use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateRewardDuration")]
pub struct UpdateRewardDurationGraphQL {
        pub lb_pair: Pubkey,
        pub reward_index: U64,
        pub old_reward_duration: U64,
        pub new_reward_duration: U64,
}

impl From<crate::types::UpdateRewardDuration> for UpdateRewardDurationGraphQL {
    fn from(original: crate::types::UpdateRewardDuration) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            reward_index: carbon_core::graphql::primitives::U64(original.reward_index),
            old_reward_duration: carbon_core::graphql::primitives::U64(original.old_reward_duration),
            new_reward_duration: carbon_core::graphql::primitives::U64(original.new_reward_duration),
        }
    }
}