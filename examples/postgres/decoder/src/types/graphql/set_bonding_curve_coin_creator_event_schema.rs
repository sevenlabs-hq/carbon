use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SetBondingCurveCoinCreatorEvent")]
pub struct SetBondingCurveCoinCreatorEventGraphQL {
    pub timestamp: I64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub bonding_curve: Pubkey,
    pub coin_creator: Pubkey,
}

impl From<crate::types::SetBondingCurveCoinCreatorEvent>
    for SetBondingCurveCoinCreatorEventGraphQL
{
    fn from(original: crate::types::SetBondingCurveCoinCreatorEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            base_mint: carbon_core::graphql::primitives::Pubkey(original.base_mint),
            pool: carbon_core::graphql::primitives::Pubkey(original.pool),
            bonding_curve: carbon_core::graphql::primitives::Pubkey(original.bonding_curve),
            coin_creator: carbon_core::graphql::primitives::Pubkey(original.coin_creator),
        }
    }
}
