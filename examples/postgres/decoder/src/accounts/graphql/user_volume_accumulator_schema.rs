use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UserVolumeAccumulator")]
pub struct UserVolumeAccumulatorGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub user: Pubkey,
        pub needs_claim: bool,
        pub total_unclaimed_tokens: U64,
        pub total_claimed_tokens: U64,
        pub current_sol_volume: U64,
        pub last_update_timestamp: I64,
        pub has_total_claimed_tokens: bool,
}

impl From<crate::accounts::postgres::UserVolumeAccumulatorRow> for UserVolumeAccumulatorGraphQL {
    fn from(row: crate::accounts::postgres::UserVolumeAccumulatorRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            user: carbon_core::graphql::primitives::Pubkey(*row.user),
            needs_claim: row.needs_claim,
            total_unclaimed_tokens: carbon_core::graphql::primitives::U64(*row.total_unclaimed_tokens),
            total_claimed_tokens: carbon_core::graphql::primitives::U64(*row.total_claimed_tokens),
            current_sol_volume: carbon_core::graphql::primitives::U64(*row.current_sol_volume),
            last_update_timestamp: carbon_core::graphql::primitives::I64(row.last_update_timestamp),
            has_total_claimed_tokens: row.has_total_claimed_tokens,
        }
    }
}