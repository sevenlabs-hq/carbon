use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6175b77375e008e5")]
pub struct ResumeEvent {
    pub state: solana_sdk::pubkey::Pubkey,
}
