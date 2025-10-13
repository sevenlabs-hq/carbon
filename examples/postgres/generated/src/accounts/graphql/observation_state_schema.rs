use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::ObservationGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ObservationState")]
pub struct ObservationStateGraphQL {
        pub initialized: bool,
        pub recent_epoch: U64,
        pub observation_index: i32,
        pub pool_id: Pubkey,
        pub observations: Vec<ObservationGraphQL>,
        pub padding: Vec<U64>,
}

impl From<crate::accounts::postgres::ObservationStateRow> for ObservationStateGraphQL {
    fn from(row: crate::accounts::postgres::ObservationStateRow) -> Self {
        Self {
            initialized: row.initialized,
            recent_epoch: carbon_core::graphql::primitives::U64(*row.recent_epoch),
            observation_index: (*row.observation_index) as i32,
            pool_id: carbon_core::graphql::primitives::Pubkey(*row.pool_id),
            observations: row.observations.0.into_iter().map(|item| item.into()).collect(),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
        }
    }
}