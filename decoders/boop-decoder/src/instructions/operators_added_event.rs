use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df73a7038cbba7098")]
pub struct OperatorsAddedEvent {
    pub operators: Vec<solana_pubkey::Pubkey>,
}
