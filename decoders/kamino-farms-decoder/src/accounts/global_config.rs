use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x95089ccaa0fcb0d9")]
pub struct GlobalConfig {
    pub global_admin: solana_pubkey::Pubkey,
    pub treasury_fee_bps: u64,
    pub treasury_vaults_authority: solana_pubkey::Pubkey,
    pub treasury_vaults_authority_bump: u64,
    pub pending_global_admin: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding1: [u128; 126],
}
