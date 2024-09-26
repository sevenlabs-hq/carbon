
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct Observation {
    pub block_timestamp: u32,
    pub tick_cumulative: i64,
    pub padding: [u64; 4],
}
