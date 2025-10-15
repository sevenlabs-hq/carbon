use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UserRewardInfo")]
pub struct UserRewardInfoGraphQL {
        pub reward_per_token_completes: Vec<U128>,
        pub reward_pendings: Vec<U64>,
}

impl From<crate::types::UserRewardInfo> for UserRewardInfoGraphQL {
    fn from(original: crate::types::UserRewardInfo) -> Self {
        Self {
            reward_per_token_completes: original.reward_per_token_completes.into_iter().map(|item| carbon_core::graphql::primitives::U128(item)).collect(),
            reward_pendings: original.reward_pendings.into_iter().map(|item| carbon_core::graphql::primitives::U64(item)).collect(),
        }
    }
}