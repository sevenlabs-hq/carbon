use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d40c6cde8260871e2")]
pub struct SwapEvent {
    pub pool_id: solana_pubkey::Pubkey,
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub trade_fee: u64,
    pub creator_fee: u64,
    pub creator_fee_on_input: bool,
}
