use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xb6a1fc657ba1cdb8")]
pub struct InsuranceDepositAccount {
    pub nonce: u8,
    pub amount: u64,
}
