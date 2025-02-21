use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x512a686f7b5992b4")]
pub struct SettlementAccount {
    pub settlement_price: u64,
    pub strikes: [u64; 23],
}
