use juniper::GraphQLEnum;


#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "RewardState")]
pub enum RewardStateGraphQL {
        Uninitialized,
        Initialized,
        Opening,
        Ended,
}

impl From<crate::types::RewardState> for RewardStateGraphQL {
    fn from(original: crate::types::RewardState) -> Self {
        match original {
            crate::types::RewardState::Uninitialized => Self::Uninitialized,
            crate::types::RewardState::Initialized => Self::Initialized,
            crate::types::RewardState::Opening => Self::Opening,
            crate::types::RewardState::Ended => Self::Ended,
        }
    }
}


