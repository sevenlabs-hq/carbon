use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe0fbfe63b1ae8904")]
pub struct StubOracle {
    pub owner: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub price: f64,
    pub last_update_ts: i64,
    pub last_update_slot: u64,
    pub deviation: f64,
    pub reserved: [u8; 104],
}
