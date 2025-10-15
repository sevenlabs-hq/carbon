use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SetMetaplexCoinCreatorEvent")]
pub struct SetMetaplexCoinCreatorEventGraphQL {
    pub timestamp: I64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub metadata: Pubkey,
    pub coin_creator: Pubkey,
}

impl From<crate::types::SetMetaplexCoinCreatorEvent> for SetMetaplexCoinCreatorEventGraphQL {
    fn from(original: crate::types::SetMetaplexCoinCreatorEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            base_mint: carbon_core::graphql::primitives::Pubkey(original.base_mint),
            pool: carbon_core::graphql::primitives::Pubkey(original.pool),
            metadata: carbon_core::graphql::primitives::Pubkey(original.metadata),
            coin_creator: carbon_core::graphql::primitives::Pubkey(original.coin_creator),
        }
    }
}
