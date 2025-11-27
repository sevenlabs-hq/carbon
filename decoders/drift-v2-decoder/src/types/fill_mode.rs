use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum FillMode {
    Fill,
    PlaceAndMake,
    PlaceAndTake(bool, u8),
    Liquidation,
}
