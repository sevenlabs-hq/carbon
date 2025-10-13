use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "GlobalVolumeAccumulator")]
pub struct GlobalVolumeAccumulatorGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub start_time: I64,
        pub end_time: I64,
        pub seconds_in_a_day: I64,
        pub mint: Pubkey,
        pub total_token_supply: Vec<U64>,
        pub sol_volumes: Vec<U64>,
}

impl From<crate::accounts::postgres::GlobalVolumeAccumulatorRow> for GlobalVolumeAccumulatorGraphQL {
    fn from(row: crate::accounts::postgres::GlobalVolumeAccumulatorRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            start_time: carbon_core::graphql::primitives::I64(row.start_time),
            end_time: carbon_core::graphql::primitives::I64(row.end_time),
            seconds_in_a_day: carbon_core::graphql::primitives::I64(row.seconds_in_a_day),
            mint: carbon_core::graphql::primitives::Pubkey(*row.mint),
            total_token_supply: row.total_token_supply.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
            sol_volumes: row.sol_volumes.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
        }
    }
}