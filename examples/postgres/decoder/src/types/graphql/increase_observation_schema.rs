use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "IncreaseObservation")]
pub struct IncreaseObservationGraphQL {
        pub oracle: Pubkey,
        pub new_observation_length: U64,
}

impl From<crate::types::IncreaseObservation> for IncreaseObservationGraphQL {
    fn from(original: crate::types::IncreaseObservation) -> Self {
        Self {
            oracle: carbon_core::graphql::primitives::Pubkey(original.oracle),
            new_observation_length: carbon_core::graphql::primitives::U64(original.new_observation_length),
        }
    }
}