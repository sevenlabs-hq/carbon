use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InnerNode {
    pub tag: u8,
    pub padding: [u8; 3],
    pub prefix_len: u32,
    pub key: u128,
    pub children: [u32; 2],
    pub child_earliest_expiry: [u64; 2],
    #[serde(with = "BigArray")]
    pub reserved: [u8; 40],
}
