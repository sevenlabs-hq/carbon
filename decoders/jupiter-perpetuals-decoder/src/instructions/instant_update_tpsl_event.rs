use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db1162f2578f61165")]
pub struct InstantUpdateTpslEvent {
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position_key: solana_pubkey::Pubkey,
    pub position_side: u8,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_custody: solana_pubkey::Pubkey,
    pub position_collateral_custody: solana_pubkey::Pubkey,
    pub position_request_key: solana_pubkey::Pubkey,
    pub position_request_mint: solana_pubkey::Pubkey,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub entire_position: bool,
    pub update_time: i64,
}
