use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d44b90d941ce3655f")]
pub struct PositionMovementEvent {
    pub net_balance_transfer: i64,
    pub margin_account_balance: u64,
    pub spread_account_balance: u64,
    pub movement_fees: u64,
}
