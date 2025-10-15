use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AdminSetCoinCreatorEvent")]
pub struct AdminSetCoinCreatorEventGraphQL {
    pub timestamp: I64,
    pub admin_set_coin_creator_authority: Pubkey,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub old_coin_creator: Pubkey,
    pub new_coin_creator: Pubkey,
}

impl From<crate::types::AdminSetCoinCreatorEvent> for AdminSetCoinCreatorEventGraphQL {
    fn from(original: crate::types::AdminSetCoinCreatorEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            admin_set_coin_creator_authority: carbon_core::graphql::primitives::Pubkey(
                original.admin_set_coin_creator_authority,
            ),
            base_mint: carbon_core::graphql::primitives::Pubkey(original.base_mint),
            pool: carbon_core::graphql::primitives::Pubkey(original.pool),
            old_coin_creator: carbon_core::graphql::primitives::Pubkey(original.old_coin_creator),
            new_coin_creator: carbon_core::graphql::primitives::Pubkey(original.new_coin_creator),
        }
    }
}
