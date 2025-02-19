use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x6a5edd53c00a4a4a")]
pub struct Mint {
    pub mint_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<solana_sdk::pubkey::Pubkey>,
}
