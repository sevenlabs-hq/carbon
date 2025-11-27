use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum AccountDiscriminator {
    Uninitialized,
    Closed,
    BondingCurve,
    AuthorityConfig,
    PermissionConfig,
    ConsumedPermission,
    MintConfig,
}
