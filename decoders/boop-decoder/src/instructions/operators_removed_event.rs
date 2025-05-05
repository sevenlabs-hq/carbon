use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2c484b46972a3559")]
pub struct OperatorsRemovedEvent {
    pub operators: Vec<solana_pubkey::Pubkey>,
}
