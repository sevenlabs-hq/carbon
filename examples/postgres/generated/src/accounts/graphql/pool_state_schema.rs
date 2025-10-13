use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::RewardInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "PoolState")]
pub struct PoolStateGraphQL {
        pub bump: Vec<U8>,
        pub amm_config: Pubkey,
        pub owner: Pubkey,
        pub token_mint0: Pubkey,
        pub token_mint1: Pubkey,
        pub token_vault0: Pubkey,
        pub token_vault1: Pubkey,
        pub observation_key: Pubkey,
        pub mint_decimals0: U8,
        pub mint_decimals1: U8,
        pub tick_spacing: i32,
        pub liquidity: U128,
        pub sqrt_price_x64: U128,
        pub tick_current: i32,
        pub padding3: i32,
        pub padding4: i32,
        pub fee_growth_global0_x64: U128,
        pub fee_growth_global1_x64: U128,
        pub protocol_fees_token0: U64,
        pub protocol_fees_token1: U64,
        pub swap_in_amount_token0: U128,
        pub swap_out_amount_token1: U128,
        pub swap_in_amount_token1: U128,
        pub swap_out_amount_token0: U128,
        pub status: U8,
        pub padding: Vec<U8>,
        pub reward_infos: Vec<RewardInfoGraphQL>,
        pub tick_array_bitmap: Vec<U64>,
        pub total_fees_token0: U64,
        pub total_fees_claimed_token0: U64,
        pub total_fees_token1: U64,
        pub total_fees_claimed_token1: U64,
        pub fund_fees_token0: U64,
        pub fund_fees_token1: U64,
        pub open_time: U64,
        pub recent_epoch: U64,
        pub padding1: Vec<U64>,
        pub padding2: Vec<U64>,
}

impl From<crate::accounts::postgres::PoolStateRow> for PoolStateGraphQL {
    fn from(row: crate::accounts::postgres::PoolStateRow) -> Self {
        Self {
            bump: row.bump.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            amm_config: carbon_core::graphql::primitives::Pubkey(*row.amm_config),
            owner: carbon_core::graphql::primitives::Pubkey(*row.owner),
            token_mint0: carbon_core::graphql::primitives::Pubkey(*row.token_mint0),
            token_mint1: carbon_core::graphql::primitives::Pubkey(*row.token_mint1),
            token_vault0: carbon_core::graphql::primitives::Pubkey(*row.token_vault0),
            token_vault1: carbon_core::graphql::primitives::Pubkey(*row.token_vault1),
            observation_key: carbon_core::graphql::primitives::Pubkey(*row.observation_key),
            mint_decimals0: carbon_core::graphql::primitives::U8((*row.mint_decimals0) as u8),
            mint_decimals1: carbon_core::graphql::primitives::U8((*row.mint_decimals1) as u8),
            tick_spacing: (*row.tick_spacing) as i32,
            liquidity: carbon_core::graphql::primitives::U128(*row.liquidity),
            sqrt_price_x64: carbon_core::graphql::primitives::U128(*row.sqrt_price_x64),
            tick_current: row.tick_current,
            padding3: (*row.padding3) as i32,
            padding4: (*row.padding4) as i32,
            fee_growth_global0_x64: carbon_core::graphql::primitives::U128(*row.fee_growth_global0_x64),
            fee_growth_global1_x64: carbon_core::graphql::primitives::U128(*row.fee_growth_global1_x64),
            protocol_fees_token0: carbon_core::graphql::primitives::U64(*row.protocol_fees_token0),
            protocol_fees_token1: carbon_core::graphql::primitives::U64(*row.protocol_fees_token1),
            swap_in_amount_token0: carbon_core::graphql::primitives::U128(*row.swap_in_amount_token0),
            swap_out_amount_token1: carbon_core::graphql::primitives::U128(*row.swap_out_amount_token1),
            swap_in_amount_token1: carbon_core::graphql::primitives::U128(*row.swap_in_amount_token1),
            swap_out_amount_token0: carbon_core::graphql::primitives::U128(*row.swap_out_amount_token0),
            status: carbon_core::graphql::primitives::U8((*row.status) as u8),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            reward_infos: row.reward_infos.0.into_iter().map(|item| item.into()).collect(),
            tick_array_bitmap: row.tick_array_bitmap.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
            total_fees_token0: carbon_core::graphql::primitives::U64(*row.total_fees_token0),
            total_fees_claimed_token0: carbon_core::graphql::primitives::U64(*row.total_fees_claimed_token0),
            total_fees_token1: carbon_core::graphql::primitives::U64(*row.total_fees_token1),
            total_fees_claimed_token1: carbon_core::graphql::primitives::U64(*row.total_fees_claimed_token1),
            fund_fees_token0: carbon_core::graphql::primitives::U64(*row.fund_fees_token0),
            fund_fees_token1: carbon_core::graphql::primitives::U64(*row.fund_fees_token1),
            open_time: carbon_core::graphql::primitives::U64(*row.open_time),
            recent_epoch: carbon_core::graphql::primitives::U64(*row.recent_epoch),
            padding1: row.padding1.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
            padding2: row.padding2.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
        }
    }
}