use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9849150f4257359d")]
pub struct EvtCreatorWithdrawSurplusEvent {
    pub pool: Pubkey,
    pub surplus_amount: u64,
}
