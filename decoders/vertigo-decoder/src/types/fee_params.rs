use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct FeeParams {
    pub normalization_period: u64,
    pub decay: f64,
    pub reference: u64,
    pub royalties_bps: u16,
    pub privileged_swapper: Option<solana_pubkey::Pubkey>,
}
