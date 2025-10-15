use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SyncUserVolumeAccumulatorEvent")]
pub struct SyncUserVolumeAccumulatorEventGraphQL {
    pub user: Pubkey,
    pub total_claimed_tokens_before: U64,
    pub total_claimed_tokens_after: U64,
    pub timestamp: I64,
}

impl From<crate::types::SyncUserVolumeAccumulatorEvent> for SyncUserVolumeAccumulatorEventGraphQL {
    fn from(original: crate::types::SyncUserVolumeAccumulatorEvent) -> Self {
        Self {
            user: carbon_core::graphql::primitives::Pubkey(original.user),
            total_claimed_tokens_before: carbon_core::graphql::primitives::U64(
                original.total_claimed_tokens_before,
            ),
            total_claimed_tokens_after: carbon_core::graphql::primitives::U64(
                original.total_claimed_tokens_after,
            ),
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
        }
    }
}
