use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RewardInfo")]
pub struct RewardInfoGraphQL {
        pub reward_state: U8,
        pub open_time: U64,
        pub end_time: U64,
        pub last_update_time: U64,
        pub emissions_per_second_x64: U128,
        pub reward_total_emissioned: U64,
        pub reward_claimed: U64,
        pub token_mint: Pubkey,
        pub token_vault: Pubkey,
        pub authority: Pubkey,
        pub reward_growth_global_x64: U128,
}

impl From<crate::types::RewardInfo> for RewardInfoGraphQL {
    fn from(original: crate::types::RewardInfo) -> Self {
        Self {
            reward_state: carbon_core::graphql::primitives::U8(original.reward_state),
            open_time: carbon_core::graphql::primitives::U64(original.open_time),
            end_time: carbon_core::graphql::primitives::U64(original.end_time),
            last_update_time: carbon_core::graphql::primitives::U64(original.last_update_time),
            emissions_per_second_x64: carbon_core::graphql::primitives::U128(original.emissions_per_second_x64),
            reward_total_emissioned: carbon_core::graphql::primitives::U64(original.reward_total_emissioned),
            reward_claimed: carbon_core::graphql::primitives::U64(original.reward_claimed),
            token_mint: carbon_core::graphql::primitives::Pubkey(original.token_mint),
            token_vault: carbon_core::graphql::primitives::Pubkey(original.token_vault),
            authority: carbon_core::graphql::primitives::Pubkey(original.authority),
            reward_growth_global_x64: carbon_core::graphql::primitives::U128(original.reward_growth_global_x64),
        }
    }
}