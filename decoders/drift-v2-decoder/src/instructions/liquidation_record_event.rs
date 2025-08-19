use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d7f11006cb60de735")]
pub struct LiquidationRecordEvent {
    pub ts: i64,
    pub liquidation_type: LiquidationType,
    pub user: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub margin_requirement: u128,
    pub total_collateral: i128,
    pub margin_freed: u64,
    pub liquidation_id: u16,
    pub bankrupt: bool,
    pub canceled_order_ids: Vec<u32>,
    pub liquidate_perp: LiquidatePerpRecord,
    pub liquidate_spot: LiquidateSpotRecord,
    pub liquidate_borrow_for_perp_pnl: LiquidateBorrowForPerpPnlRecord,
    pub liquidate_perp_pnl_for_deposit: LiquidatePerpPnlForDepositRecord,
    pub perp_bankruptcy: PerpBankruptcyRecord,
    pub spot_bankruptcy: SpotBankruptcyRecord,
}
