use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3944691a77c6d559")]
pub struct SettlePnlRecordEvent {
    pub ts: i64,
    pub user: solana_pubkey::Pubkey,
    pub market_index: u16,
    pub pnl: i128,
    pub base_asset_amount: i64,
    pub quote_asset_amount_after: i64,
    pub quote_entry_amount: i64,
    pub settle_price: i64,
    pub explanation: SettlePnlExplanation,
}
