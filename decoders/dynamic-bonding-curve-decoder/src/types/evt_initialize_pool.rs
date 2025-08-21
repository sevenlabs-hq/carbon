use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtInitializePool {
    pub pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub pool_type: u8,
    pub activation_point: u64,
}
