
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct AddLiquidity {
    pub token_amount_in: u64,
    pub min_lp_amount_out: u64,
    pub token_amount_pre_swap: Option<u64>,
}
