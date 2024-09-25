
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}
