use carbon_core::{borsh, CarbonDeserialize};

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
