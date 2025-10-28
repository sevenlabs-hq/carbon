use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapParameters2 {
    pub amount0: u64,
    pub amount1: u64,
    pub swap_mode: u8,
}
