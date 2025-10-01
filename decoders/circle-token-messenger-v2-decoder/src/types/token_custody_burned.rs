use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenCustodyBurned {
    pub custody_token_account: solana_pubkey::Pubkey,
    pub amount: u64,
}
