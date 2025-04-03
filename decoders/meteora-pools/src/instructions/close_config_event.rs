use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df9b56c5904965aae")]
pub struct CloseConfigEvent {
    pub config: solana_pubkey::Pubkey,
}
