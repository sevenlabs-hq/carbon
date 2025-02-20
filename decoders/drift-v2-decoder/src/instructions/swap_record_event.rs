use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da2bb7bc28a38faf1")]
pub struct SwapRecordEvent {
    pub ts: i64,
    pub user: solana_sdk::pubkey::Pubkey,
    pub amount_out: u64,
    pub amount_in: u64,
    pub out_market_index: u16,
    pub in_market_index: u16,
    pub out_oracle_price: i64,
    pub in_oracle_price: i64,
    pub fee: u64,
}
