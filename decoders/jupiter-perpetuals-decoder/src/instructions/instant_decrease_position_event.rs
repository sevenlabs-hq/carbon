use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dabad6a19efbe3a3b")]
pub struct InstantDecreasePositionEvent {
    pub position_key: solana_pubkey::Pubkey,
    pub position_side: u8,
    pub position_custody: solana_pubkey::Pubkey,
    pub position_collateral_custody: solana_pubkey::Pubkey,
    pub position_size_usd: u64,
    pub position_mint: solana_pubkey::Pubkey,
    pub desired_mint: solana_pubkey::Pubkey,
    pub has_profit: bool,
    pub pnl_delta: u64,
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub size_usd_delta: u64,
    pub transfer_amount_usd: u64,
    pub transfer_token: u64,
    pub price: u64,
    pub price_slippage: u64,
    pub fee_usd: u64,
    pub open_time: i64,
    pub referral: Option<solana_pubkey::Pubkey>,
}
