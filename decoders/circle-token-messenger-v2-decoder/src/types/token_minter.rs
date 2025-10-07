use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenMinter {
    pub token_controller: solana_pubkey::Pubkey,
    pub pauser: solana_pubkey::Pubkey,
    pub paused: bool,
    pub bump: u8,
}
