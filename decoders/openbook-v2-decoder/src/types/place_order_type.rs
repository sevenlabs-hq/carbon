use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PlaceOrderType {
    Limit,
    ImmediateOrCancel,
    PostOnly,
    Market,
    PostOnlySlide,
    FillOrKill,
}
