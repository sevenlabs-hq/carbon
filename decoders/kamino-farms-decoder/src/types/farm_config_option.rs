use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum FarmConfigOption {
    UpdateRewardRps,
    UpdateRewardMinClaimDuration,
    WithdrawAuthority,
    DepositWarmupPeriod,
    WithdrawCooldownPeriod,
    RewardType,
    RpsDecimals,
    LockingMode,
    LockingStartTimestamp,
    LockingDuration,
    LockingEarlyWithdrawalPenaltyBps,
    DepositCapAmount,
    SlashedAmountSpillAddress,
    ScopePricesAccount,
    ScopeOraclePriceId,
    ScopeOracleMaxAge,
    UpdateRewardScheduleCurvePoints,
    UpdatePendingFarmAdmin,
    UpdateStrategyId,
    UpdateDelegatedRpsAdmin,
    UpdateVaultId,
}
