use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4b7a9a308c4a7ba3")]
pub struct ClaimFeeEvent {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}
