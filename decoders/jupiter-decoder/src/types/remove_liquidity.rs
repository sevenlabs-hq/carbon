
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct RemoveLiquidity {
    pub lp_amount_in: u64,
    pub min_amount_out: u64,
}
