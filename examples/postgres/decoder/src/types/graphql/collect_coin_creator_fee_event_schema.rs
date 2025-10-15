use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CollectCoinCreatorFeeEvent")]
pub struct CollectCoinCreatorFeeEventGraphQL {
    pub timestamp: I64,
    pub coin_creator: Pubkey,
    pub coin_creator_fee: U64,
    pub coin_creator_vault_ata: Pubkey,
    pub coin_creator_token_account: Pubkey,
}

impl From<crate::types::CollectCoinCreatorFeeEvent> for CollectCoinCreatorFeeEventGraphQL {
    fn from(original: crate::types::CollectCoinCreatorFeeEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            coin_creator: carbon_core::graphql::primitives::Pubkey(original.coin_creator),
            coin_creator_fee: carbon_core::graphql::primitives::U64(original.coin_creator_fee),
            coin_creator_vault_ata: carbon_core::graphql::primitives::Pubkey(
                original.coin_creator_vault_ata,
            ),
            coin_creator_token_account: carbon_core::graphql::primitives::Pubkey(
                original.coin_creator_token_account,
            ),
        }
    }
}
