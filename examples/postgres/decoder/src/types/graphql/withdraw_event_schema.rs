use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "WithdrawEvent")]
pub struct WithdrawEventGraphQL {
        pub timestamp: I64,
        pub lp_token_amount_in: U64,
        pub min_base_amount_out: U64,
        pub min_quote_amount_out: U64,
        pub user_base_token_reserves: U64,
        pub user_quote_token_reserves: U64,
        pub pool_base_token_reserves: U64,
        pub pool_quote_token_reserves: U64,
        pub base_amount_out: U64,
        pub quote_amount_out: U64,
        pub lp_mint_supply: U64,
        pub pool: Pubkey,
        pub user: Pubkey,
        pub user_base_token_account: Pubkey,
        pub user_quote_token_account: Pubkey,
        pub user_pool_token_account: Pubkey,
}

impl From<crate::types::WithdrawEvent> for WithdrawEventGraphQL {
    fn from(original: crate::types::WithdrawEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            lp_token_amount_in: carbon_core::graphql::primitives::U64(original.lp_token_amount_in),
            min_base_amount_out: carbon_core::graphql::primitives::U64(original.min_base_amount_out),
            min_quote_amount_out: carbon_core::graphql::primitives::U64(original.min_quote_amount_out),
            user_base_token_reserves: carbon_core::graphql::primitives::U64(original.user_base_token_reserves),
            user_quote_token_reserves: carbon_core::graphql::primitives::U64(original.user_quote_token_reserves),
            pool_base_token_reserves: carbon_core::graphql::primitives::U64(original.pool_base_token_reserves),
            pool_quote_token_reserves: carbon_core::graphql::primitives::U64(original.pool_quote_token_reserves),
            base_amount_out: carbon_core::graphql::primitives::U64(original.base_amount_out),
            quote_amount_out: carbon_core::graphql::primitives::U64(original.quote_amount_out),
            lp_mint_supply: carbon_core::graphql::primitives::U64(original.lp_mint_supply),
            pool: carbon_core::graphql::primitives::Pubkey(original.pool),
            user: carbon_core::graphql::primitives::Pubkey(original.user),
            user_base_token_account: carbon_core::graphql::primitives::Pubkey(original.user_base_token_account),
            user_quote_token_account: carbon_core::graphql::primitives::Pubkey(original.user_quote_token_account),
            user_pool_token_account: carbon_core::graphql::primitives::Pubkey(original.user_pool_token_account),
        }
    }
}