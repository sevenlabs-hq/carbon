use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum NodeTag {
    Uninitialized,
    InnerNode,
    LeafNode,
    FreeNode,
    LastFreeNode,
}
