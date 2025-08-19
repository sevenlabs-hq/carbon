use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x119b8dd7cf04859c")]
pub struct MeteoraDammMigrationMetadata {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub pool_creator: solana_pubkey::Pubkey,
    pub partner: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub partner_locked_lp: u64,
    pub partner_lp: u64,
    pub creator_locked_lp: u64,
    pub creator_lp: u64,
    pub padding_0: u8,
    pub creator_locked_status: u8,
    pub partner_locked_status: u8,
    pub creator_claim_status: u8,
    pub partner_claim_status: u8,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 107],
}
