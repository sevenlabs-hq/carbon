use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x032c5eecdd46ac85")]
pub struct Multisig {
    pub m: u8,
    pub n: u8,
    pub is_initialized: bool,
    pub signers: [solana_pubkey::Pubkey; 11],
}
