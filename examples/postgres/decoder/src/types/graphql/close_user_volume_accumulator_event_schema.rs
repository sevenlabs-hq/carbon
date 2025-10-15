use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CloseUserVolumeAccumulatorEvent")]
pub struct CloseUserVolumeAccumulatorEventGraphQL {
    pub user: Pubkey,
    pub timestamp: I64,
    pub total_unclaimed_tokens: U64,
    pub total_claimed_tokens: U64,
    pub current_sol_volume: U64,
    pub last_update_timestamp: I64,
}

impl From<crate::types::CloseUserVolumeAccumulatorEvent>
    for CloseUserVolumeAccumulatorEventGraphQL
{
    fn from(original: crate::types::CloseUserVolumeAccumulatorEvent) -> Self {
        Self {
            user: carbon_core::graphql::primitives::Pubkey(original.user),
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            total_unclaimed_tokens: carbon_core::graphql::primitives::U64(
                original.total_unclaimed_tokens,
            ),
            total_claimed_tokens: carbon_core::graphql::primitives::U64(
                original.total_claimed_tokens,
            ),
            current_sol_volume: carbon_core::graphql::primitives::U64(original.current_sol_volume),
            last_update_timestamp: carbon_core::graphql::primitives::I64(
                original.last_update_timestamp,
            ),
        }
    }
}
