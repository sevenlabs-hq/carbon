use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GetRemoveLiquidityAmountAndFee2Params {
    pub lp_amount_in: u64,
}
