use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "FundReward")]
pub struct FundRewardGraphQL {
        pub lb_pair: Pubkey,
        pub funder: Pubkey,
        pub reward_index: U64,
        pub amount: U64,
}

impl From<crate::types::FundReward> for FundRewardGraphQL {
    fn from(original: crate::types::FundReward) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            funder: carbon_core::graphql::primitives::Pubkey(original.funder),
            reward_index: carbon_core::graphql::primitives::U64(original.reward_index),
            amount: carbon_core::graphql::primitives::U64(original.amount),
        }
    }
}