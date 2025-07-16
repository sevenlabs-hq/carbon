use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb6a1fc657ba1cdb8")]
pub struct InsuranceDepositAccount {
    pub nonce: u8,
    pub amount: u64,
}
