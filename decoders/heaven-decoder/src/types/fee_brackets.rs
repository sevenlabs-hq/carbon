use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FeeBrackets {
    pub brackets: [FeeBracket; 4],
    pub count: u8,
    pub padding: [u8; 7],
}
