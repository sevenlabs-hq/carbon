use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d0a32f075ed43e6e9")]
pub struct SettleFundsLogEvent {
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub base_native: u64,
    pub quote_native: u64,
    pub referrer_rebate: u64,
    pub referrer: Option<solana_sdk::pubkey::Pubkey>,
}
