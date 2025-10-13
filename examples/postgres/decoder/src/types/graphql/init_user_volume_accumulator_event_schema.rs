use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitUserVolumeAccumulatorEvent")]
pub struct InitUserVolumeAccumulatorEventGraphQL {
        pub payer: Pubkey,
        pub user: Pubkey,
        pub timestamp: I64,
}

impl From<crate::types::InitUserVolumeAccumulatorEvent> for InitUserVolumeAccumulatorEventGraphQL {
    fn from(original: crate::types::InitUserVolumeAccumulatorEvent) -> Self {
        Self {
            payer: carbon_core::graphql::primitives::Pubkey(original.payer),
            user: carbon_core::graphql::primitives::Pubkey(original.user),
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
        }
    }
}