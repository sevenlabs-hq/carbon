

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct OperatorsAddedEvent {
    pub operators: Vec<solana_pubkey::Pubkey>,
}
