use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Bootstrapping {
    pub activation_point: u64,
    pub whitelisted_vault: solana_sdk::pubkey::Pubkey,
    pub pool_creator: solana_sdk::pubkey::Pubkey,
    pub activation_type: u8,
}
