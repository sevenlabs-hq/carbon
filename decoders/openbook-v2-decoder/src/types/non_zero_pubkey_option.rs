use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct NonZeroPubkeyOption {
    pub key: solana_sdk::pubkey::Pubkey,
}
