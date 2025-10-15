use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ClaimTokenIncentivesEvent")]
pub struct ClaimTokenIncentivesEventGraphQL {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: U64,
    pub timestamp: I64,
    pub total_claimed_tokens: U64,
    pub current_sol_volume: U64,
}

impl From<crate::types::ClaimTokenIncentivesEvent> for ClaimTokenIncentivesEventGraphQL {
    fn from(original: crate::types::ClaimTokenIncentivesEvent) -> Self {
        Self {
            user: carbon_core::graphql::primitives::Pubkey(original.user),
            mint: carbon_core::graphql::primitives::Pubkey(original.mint),
            amount: carbon_core::graphql::primitives::U64(original.amount),
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            total_claimed_tokens: carbon_core::graphql::primitives::U64(
                original.total_claimed_tokens,
            ),
            current_sol_volume: carbon_core::graphql::primitives::U64(original.current_sol_volume),
        }
    }
}
