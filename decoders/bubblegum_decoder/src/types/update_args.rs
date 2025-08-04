use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateArgs {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub creators: Option<Vec<Creator>>,
    pub seller_fee_basis_points: Option<u16>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}
