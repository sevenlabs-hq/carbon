use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum RuleSet {
    None,
    ProgramAllowList(Vec<solana_sdk::pubkey::Pubkey>),
    ProgramDenyList(Vec<solana_sdk::pubkey::Pubkey>),
}
