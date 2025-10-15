use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeReward")]
pub struct InitializeRewardGraphQL {
        pub lb_pair: Pubkey,
        pub reward_mint: Pubkey,
        pub funder: Pubkey,
        pub reward_index: U64,
        pub reward_duration: U64,
}

impl From<crate::types::InitializeReward> for InitializeRewardGraphQL {
    fn from(original: crate::types::InitializeReward) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            reward_mint: carbon_core::graphql::primitives::Pubkey(original.reward_mint),
            funder: carbon_core::graphql::primitives::Pubkey(original.funder),
            reward_index: carbon_core::graphql::primitives::U64(original.reward_index),
            reward_duration: carbon_core::graphql::primitives::U64(original.reward_duration),
        }
    }
}