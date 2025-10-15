use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "BuyEvent")]
pub struct BuyEventGraphQL {
    pub timestamp: I64,
    pub base_amount_out: U64,
    pub max_quote_amount_in: U64,
    pub user_base_token_reserves: U64,
    pub user_quote_token_reserves: U64,
    pub pool_base_token_reserves: U64,
    pub pool_quote_token_reserves: U64,
    pub quote_amount_in: U64,
    pub lp_fee_basis_points: U64,
    pub lp_fee: U64,
    pub protocol_fee_basis_points: U64,
    pub protocol_fee: U64,
    pub quote_amount_in_with_lp_fee: U64,
    pub user_quote_amount_in: U64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: U64,
    pub coin_creator_fee: U64,
    pub track_volume: bool,
    pub total_unclaimed_tokens: U64,
    pub total_claimed_tokens: U64,
    pub current_sol_volume: U64,
    pub last_update_timestamp: I64,
}

impl From<crate::types::BuyEvent> for BuyEventGraphQL {
    fn from(original: crate::types::BuyEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            base_amount_out: carbon_core::graphql::primitives::U64(original.base_amount_out),
            max_quote_amount_in: carbon_core::graphql::primitives::U64(
                original.max_quote_amount_in,
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
            quote_amount_in: carbon_core::graphql::primitives::U64(original.quote_amount_in),
            lp_fee_basis_points: carbon_core::graphql::primitives::U64(
                original.lp_fee_basis_points,
            ),
            lp_fee: carbon_core::graphql::primitives::U64(original.lp_fee),
            protocol_fee_basis_points: carbon_core::graphql::primitives::U64(
                original.protocol_fee_basis_points,
            ),
            protocol_fee: carbon_core::graphql::primitives::U64(original.protocol_fee),
            quote_amount_in_with_lp_fee: carbon_core::graphql::primitives::U64(
                original.quote_amount_in_with_lp_fee,
            ),
            user_quote_amount_in: carbon_core::graphql::primitives::U64(
                original.user_quote_amount_in,
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
            track_volume: original.track_volume,
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
