use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateRewardFunder")]
pub struct UpdateRewardFunderGraphQL {
        pub lb_pair: Pubkey,
        pub reward_index: U64,
        pub old_funder: Pubkey,
        pub new_funder: Pubkey,
}

impl From<crate::types::UpdateRewardFunder> for UpdateRewardFunderGraphQL {
    fn from(original: crate::types::UpdateRewardFunder) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            reward_index: carbon_core::graphql::primitives::U64(original.reward_index),
            old_funder: carbon_core::graphql::primitives::Pubkey(original.old_funder),
            new_funder: carbon_core::graphql::primitives::Pubkey(original.new_funder),
        }
    }
}