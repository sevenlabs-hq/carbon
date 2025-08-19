use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de0344347c2ed6d01")]
pub struct OrderActionRecordEvent {
    pub ts: i64,
    pub action: OrderAction,
    pub action_explanation: OrderActionExplanation,
    pub market_index: u16,
    pub market_type: MarketType,
    pub filler: Option<solana_pubkey::Pubkey>,
    pub filler_reward: Option<u64>,
    pub fill_record_id: Option<u64>,
    pub base_asset_amount_filled: Option<u64>,
    pub quote_asset_amount_filled: Option<u64>,
    pub taker_fee: Option<u64>,
    pub maker_fee: Option<i64>,
    pub referrer_reward: Option<u32>,
    pub quote_asset_amount_surplus: Option<i64>,
    pub spot_fulfillment_method_fee: Option<u64>,
    pub taker: Option<solana_pubkey::Pubkey>,
    pub taker_order_id: Option<u32>,
    pub taker_order_direction: Option<PositionDirection>,
    pub taker_order_base_asset_amount: Option<u64>,
    pub taker_order_cumulative_base_asset_amount_filled: Option<u64>,
    pub taker_order_cumulative_quote_asset_amount_filled: Option<u64>,
    pub maker: Option<solana_pubkey::Pubkey>,
    pub maker_order_id: Option<u32>,
    pub maker_order_direction: Option<PositionDirection>,
    pub maker_order_base_asset_amount: Option<u64>,
    pub maker_order_cumulative_base_asset_amount_filled: Option<u64>,
    pub maker_order_cumulative_quote_asset_amount_filled: Option<u64>,
    pub oracle_price: i64,
}
