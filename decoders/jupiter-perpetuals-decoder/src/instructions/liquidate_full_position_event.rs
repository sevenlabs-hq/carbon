use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d806547a880485654")]
pub struct LiquidateFullPositionEvent {
    pub position_key: solana_sdk::pubkey::Pubkey,
    pub position_side: u8,
    pub position_custody: solana_sdk::pubkey::Pubkey,
    pub position_collateral_custody: solana_sdk::pubkey::Pubkey,
    pub position_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_size_usd: u64,
    pub has_profit: bool,
    pub pnl_delta: u64,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub transfer_amount_usd: u64,
    pub transfer_token: u64,
    pub price: u64,
    pub fee_usd: u64,
    pub liquidation_fee_usd: u64,
    pub open_time: i64,
}
