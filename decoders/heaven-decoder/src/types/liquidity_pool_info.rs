use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolInfo {
    pub creator: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub open_at: u64,
    pub created_at: u64,
    pub protocol_config_version: u16,
    pub r_type: u8,
    pub pool_authority_bump: u8,
    pub temp_sol_holder_bump: u8,
    pub pad: [u8; 3],
}
