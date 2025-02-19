use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x032c5eecdd46ac85")]
pub struct Multisig {
    pub m: u8,
    pub n: u8,
    pub is_initialized: bool,
    pub signers: [solana_sdk::pubkey::Pubkey; 11],
}
