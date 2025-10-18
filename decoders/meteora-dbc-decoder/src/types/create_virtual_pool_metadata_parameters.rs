use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateVirtualPoolMetadataParameters {
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 96],
    pub name: String,
    pub website: String,
    pub logo: String,
}
