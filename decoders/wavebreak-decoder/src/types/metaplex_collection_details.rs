use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum MetaplexCollectionDetails {
    V1 { size: u64 },
    V2 { padding: [u8; 8] },
}
