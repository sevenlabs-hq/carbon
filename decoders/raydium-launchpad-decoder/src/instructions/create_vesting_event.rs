use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d96980bb334d2bf7d")]
pub struct CreateVestingEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub share_amount: u64,
}
