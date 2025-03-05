use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Authorized {
    pub staker: solana_sdk::pubkey::Pubkey,
    pub withdrawer: solana_sdk::pubkey::Pubkey,
}
