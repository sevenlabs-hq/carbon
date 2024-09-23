use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5f72619cd42e9808")]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
}
