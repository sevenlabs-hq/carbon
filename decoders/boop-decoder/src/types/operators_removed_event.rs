use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OperatorsRemovedEvent {
    pub operators: Vec<solana_pubkey::Pubkey>,
}
