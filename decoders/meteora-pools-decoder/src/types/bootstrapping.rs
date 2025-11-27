use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Bootstrapping {
    pub activation_point: u64,
    pub whitelisted_vault: solana_pubkey::Pubkey,
    pub pool_creator: solana_pubkey::Pubkey,
    pub activation_type: u8,
}
