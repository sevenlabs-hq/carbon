

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d43a4bec09c9ca8d2")]
pub struct RemoveValidatorEvent{
    pub state: solana_sdk::pubkey::Pubkey,
    pub validator: solana_sdk::pubkey::Pubkey,
    pub index: u32,
    pub operational_sol_balance: u64,
}
