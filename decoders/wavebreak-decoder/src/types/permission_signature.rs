use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PermissionSignature {
    pub recovery_id: u8,
    #[serde(with = "serde_big_array::BigArray")]
    pub bytes: [u8; 64],
}
