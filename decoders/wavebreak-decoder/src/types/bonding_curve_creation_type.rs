use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum BondingCurveCreationType {
    Manual,
    Launch,
    Lockedlaunch,
    Presale,
}
