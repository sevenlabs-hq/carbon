use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U32;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Observation")]
pub struct ObservationGraphQL {
        pub block_timestamp: U32,
        pub tick_cumulative: I64,
        pub padding: Vec<U64>,
}

impl From<crate::types::Observation> for ObservationGraphQL {
    fn from(original: crate::types::Observation) -> Self {
        Self {
            block_timestamp: carbon_core::graphql::primitives::U32(original.block_timestamp),
            tick_cumulative: carbon_core::graphql::primitives::I64(original.tick_cumulative),
            padding: original.padding.into_iter().map(|item| carbon_core::graphql::primitives::U64(item)).collect(),
        }
    }
}