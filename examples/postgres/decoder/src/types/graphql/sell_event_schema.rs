use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SellEvent")]
pub struct SellEventGraphQL {
    pub timestamp: I64,
    pub base_amount_in: U64,
    pub min_quote_amount_out: U64,
    pub user_base_token_reserves: U64,
    pub user_quote_token_reserves: U64,
    pub pool_base_token_reserves: U64,
    pub pool_quote_token_reserves: U64,
    pub quote_amount_out: U64,
    pub lp_fee_basis_points: U64,
    pub lp_fee: U64,
    pub protocol_fee_basis_points: U64,
    pub protocol_fee: U64,
    pub quote_amount_out_without_lp_fee: U64,
    pub user_quote_amount_out: U64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: U64,
    pub coin_creator_fee: U64,
}

impl From<crate::types::SellEvent> for SellEventGraphQL {
    fn from(original: crate::types::SellEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            base_amount_in: carbon_core::graphql::primitives::U64(original.base_amount_in),
            min_quote_amount_out: carbon_core::graphql::primitives::U64(
                original.min_quote_amount_out,
            ),
            user_base_token_reserves: carbon_core::graphql::primitives::U64(
                original.user_base_token_reserves,
            ),
            user_quote_token_reserves: carbon_core::graphql::primitives::U64(
                original.user_quote_token_reserves,
            ),
            pool_base_token_reserves: carbon_core::graphql::primitives::U64(
                original.pool_base_token_reserves,
            ),
            pool_quote_token_reserves: carbon_core::graphql::primitives::U64(
                original.pool_quote_token_reserves,
            ),
            quote_amount_out: carbon_core::graphql::primitives::U64(original.quote_amount_out),
            lp_fee_basis_points: carbon_core::graphql::primitives::U64(
                original.lp_fee_basis_points,
            ),
            lp_fee: carbon_core::graphql::primitives::U64(original.lp_fee),
            protocol_fee_basis_points: carbon_core::graphql::primitives::U64(
                original.protocol_fee_basis_points,
            ),
            protocol_fee: carbon_core::graphql::primitives::U64(original.protocol_fee),
            quote_amount_out_without_lp_fee: carbon_core::graphql::primitives::U64(
                original.quote_amount_out_without_lp_fee,
            ),
            user_quote_amount_out: carbon_core::graphql::primitives::U64(
                original.user_quote_amount_out,
            ),
            pool: carbon_core::graphql::primitives::Pubkey(original.pool),
            user: carbon_core::graphql::primitives::Pubkey(original.user),
            user_base_token_account: carbon_core::graphql::primitives::Pubkey(
                original.user_base_token_account,
            ),
            user_quote_token_account: carbon_core::graphql::primitives::Pubkey(
                original.user_quote_token_account,
            ),
            protocol_fee_recipient: carbon_core::graphql::primitives::Pubkey(
                original.protocol_fee_recipient,
            ),
            protocol_fee_recipient_token_account: carbon_core::graphql::primitives::Pubkey(
                original.protocol_fee_recipient_token_account,
            ),
            coin_creator: carbon_core::graphql::primitives::Pubkey(original.coin_creator),
            coin_creator_fee_basis_points: carbon_core::graphql::primitives::U64(
                original.coin_creator_fee_basis_points,
            ),
            coin_creator_fee: carbon_core::graphql::primitives::U64(original.coin_creator_fee),
        }
    }
}
