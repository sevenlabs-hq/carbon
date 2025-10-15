use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

/// Stores the state relevant for tracking liquidity mining rewards
#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RewardInfo")]
pub struct RewardInfoGraphQL {
            /// Reward token mint.
            pub mint: Pubkey,
            /// Reward vault token account.
            pub vault: Pubkey,
            /// Authority account that allows to fund rewards
            pub funder: Pubkey,
            /// TODO check whether we need to store it in pool
            pub reward_duration: U64,
            /// TODO check whether we need to store it in pool
            pub reward_duration_end: U64,
            /// TODO check whether we need to store it in pool
            pub reward_rate: U128,
            /// The last time reward states were updated.
            pub last_update_time: U64,
            /// Accumulated seconds where when farm distribute rewards, but the bin is empty. The reward will be accumulated for next reward time window.
            pub cumulative_seconds_with_empty_liquidity_reward: U64,
}

impl From<crate::types::RewardInfo> for RewardInfoGraphQL {
    fn from(original: crate::types::RewardInfo) -> Self {
        Self {
            mint: carbon_core::graphql::primitives::Pubkey(original.mint),
            vault: carbon_core::graphql::primitives::Pubkey(original.vault),
            funder: carbon_core::graphql::primitives::Pubkey(original.funder),
            reward_duration: carbon_core::graphql::primitives::U64(original.reward_duration),
            reward_duration_end: carbon_core::graphql::primitives::U64(original.reward_duration_end),
            reward_rate: carbon_core::graphql::primitives::U128(original.reward_rate),
            last_update_time: carbon_core::graphql::primitives::U64(original.last_update_time),
            cumulative_seconds_with_empty_liquidity_reward: carbon_core::graphql::primitives::U64(original.cumulative_seconds_with_empty_liquidity_reward),
        }
    }
}