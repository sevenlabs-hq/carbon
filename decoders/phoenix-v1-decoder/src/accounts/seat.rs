use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x5ae4165aa256ad1a")]
pub struct Seat {
    pub discriminant: u64,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trader: solana_sdk::pubkey::Pubkey,
    pub approval_status: u64,
    pub padding: [u64; 6],
}
