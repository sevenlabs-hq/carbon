use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum MetaplexProgrammableConfig {
    V1 {
        rule_set: Option<solana_pubkey::Pubkey>,
    },
}
