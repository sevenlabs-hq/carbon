use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9fa4f5725efd0309")]
pub struct ConfigMarinadeEvent {
    pub state: solana_pubkey::Pubkey,
    pub rewards_fee_change: Option<FeeValueChange>,
    pub slots_for_stake_delta_change: Option<U64ValueChange>,
    pub min_stake_change: Option<U64ValueChange>,
    pub min_deposit_change: Option<U64ValueChange>,
    pub min_withdraw_change: Option<U64ValueChange>,
    pub staking_sol_cap_change: Option<U64ValueChange>,
    pub liquidity_sol_cap_change: Option<U64ValueChange>,
    pub withdraw_stake_account_enabled_change: Option<BoolValueChange>,
    pub delayed_unstake_fee_change: Option<FeeCentsValueChange>,
    pub withdraw_stake_account_fee_change: Option<FeeCentsValueChange>,
    pub max_stake_moved_per_epoch_change: Option<FeeValueChange>,
}
