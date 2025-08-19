use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum MarginCalculationMode {
    Standard {
        track_open_orders_fraction: bool,
    },
    Liquidation {
        market_to_track_margin_requirement: Option<MarketIdentifier>,
    },
}
