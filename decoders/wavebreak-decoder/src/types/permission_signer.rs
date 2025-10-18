use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PermissionSigner {
    #[serde(with = "serde_big_array::BigArray")]
    pub bytes: [u8; 33],
}
