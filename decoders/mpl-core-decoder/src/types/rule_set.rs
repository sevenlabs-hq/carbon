use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum RuleSet {
    None,
    ProgramAllowList(Vec<solana_pubkey::Pubkey>),
    ProgramDenyList(Vec<solana_pubkey::Pubkey>),
}
