use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SplitStakeAccountInfo {
    pub account: solana_pubkey::Pubkey,
    pub index: u32,
}
