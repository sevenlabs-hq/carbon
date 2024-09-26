
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub enum RewardState {
    Uninitialized,
    Initialized,
    Opening,
    Ended,
}


