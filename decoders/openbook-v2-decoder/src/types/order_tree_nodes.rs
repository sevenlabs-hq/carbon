use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OrderTreeNodes {
    pub order_tree_type: u8,
    pub padding: [u8; 3],
    pub bump_index: u32,
    pub free_list_len: u32,
    pub free_list_head: u32,
    #[serde(with = "BigArray")]
    pub reserved: [u8; 512],
    #[serde(with = "BigArray")]
    pub nodes: [AnyNode; 1024],
}
