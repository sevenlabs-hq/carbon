use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct Deposit {
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}
