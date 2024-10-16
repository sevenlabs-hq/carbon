use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AmmConfig {
    pub pnl_owner: solana_sdk::pubkey::Pubkey,
    pub cancel_owner: solana_sdk::pubkey::Pubkey,
    pub pending1: [u64; 28],
    pub pending2: [u64; 32],
}
