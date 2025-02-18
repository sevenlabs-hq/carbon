use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AnyNode {
    pub tag: u8,
    #[serde(with = "BigArray")]
    pub data: [u8; 87],
}
