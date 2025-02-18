use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xc35380d5cc5b1396")]
pub struct OpenOrdersIndexer {
    pub bump: u8,
    pub created_counter: u32,
    pub addresses: Vec<solana_sdk::pubkey::Pubkey>,
}
