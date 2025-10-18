use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MsolTicketSolSpent {
    pub cost_basis: u64,
    pub msol_unstaked: u64,
}
