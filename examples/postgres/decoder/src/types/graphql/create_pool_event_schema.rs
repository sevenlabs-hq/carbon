use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreatePoolEvent")]
pub struct CreatePoolEventGraphQL {
    pub timestamp: I64,
    pub index: i32,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: U8,
    pub quote_mint_decimals: U8,
    pub base_amount_in: U64,
    pub quote_amount_in: U64,
    pub pool_base_amount: U64,
    pub pool_quote_amount: U64,
    pub minimum_liquidity: U64,
    pub initial_liquidity: U64,
    pub lp_token_amount_out: U64,
    pub pool_bump: U8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub coin_creator: Pubkey,
}

impl From<crate::types::CreatePoolEvent> for CreatePoolEventGraphQL {
    fn from(original: crate::types::CreatePoolEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            index: original.index as i32,
            creator: carbon_core::graphql::primitives::Pubkey(original.creator),
            base_mint: carbon_core::graphql::primitives::Pubkey(original.base_mint),
            quote_mint: carbon_core::graphql::primitives::Pubkey(original.quote_mint),
            base_mint_decimals: carbon_core::graphql::primitives::U8(original.base_mint_decimals),
            quote_mint_decimals: carbon_core::graphql::primitives::U8(original.quote_mint_decimals),
            base_amount_in: carbon_core::graphql::primitives::U64(original.base_amount_in),
            quote_amount_in: carbon_core::graphql::primitives::U64(original.quote_amount_in),
            pool_base_amount: carbon_core::graphql::primitives::U64(original.pool_base_amount),
            pool_quote_amount: carbon_core::graphql::primitives::U64(original.pool_quote_amount),
            minimum_liquidity: carbon_core::graphql::primitives::U64(original.minimum_liquidity),
            initial_liquidity: carbon_core::graphql::primitives::U64(original.initial_liquidity),
            lp_token_amount_out: carbon_core::graphql::primitives::U64(
                original.lp_token_amount_out,
            ),
            pool_bump: carbon_core::graphql::primitives::U8(original.pool_bump),
            pool: carbon_core::graphql::primitives::Pubkey(original.pool),
            lp_mint: carbon_core::graphql::primitives::Pubkey(original.lp_mint),
            user_base_token_account: carbon_core::graphql::primitives::Pubkey(
                original.user_base_token_account,
            ),
            user_quote_token_account: carbon_core::graphql::primitives::Pubkey(
                original.user_quote_token_account,
            ),
            coin_creator: carbon_core::graphql::primitives::Pubkey(original.coin_creator),
        }
    }
}
