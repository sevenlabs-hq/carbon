use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d7a027f010ebf0caf")]
pub struct CollectCreatorFeeEvent {
    pub timestamp: i64,
    pub creator: solana_pubkey::Pubkey,
    pub creator_fee: u64,
}
