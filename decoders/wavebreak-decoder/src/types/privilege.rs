use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum Privilege {
    AuthorityConfigUpdate,
    ReceiveProtocolFees,
    MintConfigInitialize,
    MintConfigUpdate,
    MintConfigClose,
    PermissionConfigInitialize,
    PermissionConfigUpdate,
    PermissionConfigClose,
    BondingCurveInitialize,
    BondingCurveUpdate,
    BondingCurveClose,
}
