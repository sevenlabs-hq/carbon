use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d0aa3557381e050c0")]
pub struct InstantCreateLimitOrderEvent {
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position_key: solana_pubkey::Pubkey,
    pub position_side: u8,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_custody: solana_pubkey::Pubkey,
    pub position_collateral_mint: solana_pubkey::Pubkey,
    pub position_collateral_custody: solana_pubkey::Pubkey,
    pub position_request_key: solana_pubkey::Pubkey,
    pub position_request_mint: solana_pubkey::Pubkey,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub open_time: i64,
}
