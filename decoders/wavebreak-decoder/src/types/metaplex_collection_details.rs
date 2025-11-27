use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum MetaplexCollectionDetails {
    V1 { size: u64 },
    V2 { padding: [u8; 8] },
}
