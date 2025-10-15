use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "WithdrawIneligibleReward")]
pub struct WithdrawIneligibleRewardGraphQL {
        pub lb_pair: Pubkey,
        pub reward_mint: Pubkey,
        pub amount: U64,
}

impl From<crate::types::WithdrawIneligibleReward> for WithdrawIneligibleRewardGraphQL {
    fn from(original: crate::types::WithdrawIneligibleReward) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            reward_mint: carbon_core::graphql::primitives::Pubkey(original.reward_mint),
            amount: carbon_core::graphql::primitives::U64(original.amount),
        }
    }
}