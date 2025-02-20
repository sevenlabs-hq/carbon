use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TestInitParams {
    pub allow_swap: bool,
    pub allow_add_liquidity: bool,
    pub allow_remove_liquidity: bool,
    pub allow_increase_position: bool,
    pub allow_decrease_position: bool,
    pub allow_collateral_withdrawal: bool,
    pub allow_liquidate_position: bool,
}
