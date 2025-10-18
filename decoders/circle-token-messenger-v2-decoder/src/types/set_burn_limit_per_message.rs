use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetBurnLimitPerMessage {
    pub token: solana_pubkey::Pubkey,
    pub burn_limit_per_message: u64,
}
