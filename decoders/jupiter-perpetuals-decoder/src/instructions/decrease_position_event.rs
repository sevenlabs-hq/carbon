use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d409c2b4a6d83107f")]
pub struct DecreasePositionEvent {
    pub position_key: solana_pubkey::Pubkey,
    pub position_side: u8,
    pub position_custody: solana_pubkey::Pubkey,
    pub position_collateral_custody: solana_pubkey::Pubkey,
    pub position_size_usd: u64,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_request_key: solana_pubkey::Pubkey,
    pub position_request_mint: solana_pubkey::Pubkey,
    pub position_request_change: u8,
    pub position_request_type: u8,
    pub has_profit: bool,
    pub pnl_delta: u64,
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub size_usd_delta: u64,
    pub transfer_amount_usd: u64,
    pub transfer_token: Option<u64>,
    pub price: u64,
    pub price_slippage: Option<u64>,
    pub fee_usd: u64,
    pub open_time: i64,
    pub referral: Option<solana_pubkey::Pubkey>,
}
