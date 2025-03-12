

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d496bcee1a13b400f")]
pub struct UserSwapBalancesEvent{
    pub user_lamports: u64,
    pub input_ta_balance: u64,
    pub output_ta_balance: u64,
}
