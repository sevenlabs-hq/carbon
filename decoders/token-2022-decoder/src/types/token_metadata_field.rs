use alloc::{format, string::String};
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum TokenMetadataField {
    Name,
    Symbol,
    Uri,
    Key(String),
}
