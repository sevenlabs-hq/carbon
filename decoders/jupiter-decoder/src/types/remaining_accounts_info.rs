
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
