use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7c8627b40b49e869")]
pub struct NftList {
    pub version: u8,
    pub collection_name: String,
}
