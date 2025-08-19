use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Pool {
    pub owner: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority_bump: u8,
    pub is_active: bool,
    pub amp_initial_factor: u16,
    pub amp_target_factor: u16,
    pub ramp_start_ts: i64,
    pub ramp_stop_ts: i64,
    pub swap_fee: u64,
    pub tokens: Vec<PoolToken>,
    pub pending_owner: Option<solana_pubkey::Pubkey>,
    pub max_supply: u64,
}
