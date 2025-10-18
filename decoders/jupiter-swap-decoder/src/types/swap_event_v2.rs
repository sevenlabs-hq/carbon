use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapEventV2 {
    pub input_mint: solana_pubkey::Pubkey,
    pub input_amount: u64,
    pub output_mint: solana_pubkey::Pubkey,
    pub output_amount: u64,
}
