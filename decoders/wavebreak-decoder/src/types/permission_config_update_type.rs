use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PermissionConfigUpdateType {
    AllowedSigner1(PermissionSigner),
    AllowedSigner2(PermissionSigner),
    AllowedSigner3(PermissionSigner),
}
