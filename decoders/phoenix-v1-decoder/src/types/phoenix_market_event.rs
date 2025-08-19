use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PhoenixMarketEvent {
    Uninitialized,
    Header(AuditLogHeader),
    Fill(FillEvent),
    Place(PlaceEvent),
    Reduce(ReduceEvent),
    Evict(EvictEvent),
    FillSummary(FillSummaryEvent),
    Fee(FeeEvent),
    TimeInForce(TimeInForceEvent),
    ExpiredOrder(ExpiredOrderEvent),
}
