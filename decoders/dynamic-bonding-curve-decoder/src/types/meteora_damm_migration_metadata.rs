use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MeteoraDammMigrationMetadata {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub padding_0: [u8; 32],
    pub partner: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub partner_locked_lp: u64,
    pub partner_lp: u64,
    pub creator_locked_lp: u64,
    pub creator_lp: u64,
    pub _padding_0: u8,
    pub creator_locked_status: u8,
    pub partner_locked_status: u8,
    pub creator_claim_status: u8,
    pub partner_claim_status: u8,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 107],
}
