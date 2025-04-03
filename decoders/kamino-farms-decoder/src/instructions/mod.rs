use {super::KaminoFarmsDecoder, crate::PROGRAM_ID};
pub mod add_rewards;
pub mod deposit_to_farm_vault;
pub mod harvest_reward;
pub mod idl_missing_types;
pub mod initialize_farm;
pub mod initialize_farm_delegated;
pub mod initialize_global_config;
pub mod initialize_reward;
pub mod initialize_user;
pub mod refresh_farm;
pub mod refresh_user_state;
pub mod reward_user_once;
pub mod set_stake_delegated;
pub mod stake;
pub mod transfer_ownership;
pub mod unstake;
pub mod update_farm_admin;
pub mod update_farm_config;
pub mod update_global_config;
pub mod update_global_config_admin;
pub mod withdraw_from_farm_vault;
pub mod withdraw_reward;
pub mod withdraw_slashed_amount;
pub mod withdraw_treasury;
pub mod withdraw_unstaked_deposits;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum KaminoFarmsInstruction {
    InitializeGlobalConfig(initialize_global_config::InitializeGlobalConfig),
    UpdateGlobalConfig(update_global_config::UpdateGlobalConfig),
    InitializeFarm(initialize_farm::InitializeFarm),
    InitializeFarmDelegated(initialize_farm_delegated::InitializeFarmDelegated),
    InitializeReward(initialize_reward::InitializeReward),
    AddRewards(add_rewards::AddRewards),
    UpdateFarmConfig(update_farm_config::UpdateFarmConfig),
    InitializeUser(initialize_user::InitializeUser),
    TransferOwnership(transfer_ownership::TransferOwnership),
    RewardUserOnce(reward_user_once::RewardUserOnce),
    RefreshFarm(refresh_farm::RefreshFarm),
    Stake(stake::Stake),
    SetStakeDelegated(set_stake_delegated::SetStakeDelegated),
    HarvestReward(harvest_reward::HarvestReward),
    Unstake(unstake::Unstake),
    RefreshUserState(refresh_user_state::RefreshUserState),
    WithdrawUnstakedDeposits(withdraw_unstaked_deposits::WithdrawUnstakedDeposits),
    WithdrawTreasury(withdraw_treasury::WithdrawTreasury),
    DepositToFarmVault(deposit_to_farm_vault::DepositToFarmVault),
    WithdrawFromFarmVault(withdraw_from_farm_vault::WithdrawFromFarmVault),
    WithdrawSlashedAmount(withdraw_slashed_amount::WithdrawSlashedAmount),
    UpdateFarmAdmin(update_farm_admin::UpdateFarmAdmin),
    UpdateGlobalConfigAdmin(update_global_config_admin::UpdateGlobalConfigAdmin),
    WithdrawReward(withdraw_reward::WithdrawReward),
    IdlMissingTypes(idl_missing_types::IdlMissingTypes),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for KaminoFarmsDecoder {
    type InstructionType = KaminoFarmsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        carbon_core::try_decode_instructions!(instruction,
            KaminoFarmsInstruction::InitializeGlobalConfig => initialize_global_config::InitializeGlobalConfig,
            KaminoFarmsInstruction::UpdateGlobalConfig => update_global_config::UpdateGlobalConfig,
            KaminoFarmsInstruction::InitializeFarm => initialize_farm::InitializeFarm,
            KaminoFarmsInstruction::InitializeFarmDelegated => initialize_farm_delegated::InitializeFarmDelegated,
            KaminoFarmsInstruction::InitializeReward => initialize_reward::InitializeReward,
            KaminoFarmsInstruction::AddRewards => add_rewards::AddRewards,
            KaminoFarmsInstruction::UpdateFarmConfig => update_farm_config::UpdateFarmConfig,
            KaminoFarmsInstruction::InitializeUser => initialize_user::InitializeUser,
            KaminoFarmsInstruction::TransferOwnership => transfer_ownership::TransferOwnership,
            KaminoFarmsInstruction::RewardUserOnce => reward_user_once::RewardUserOnce,
            KaminoFarmsInstruction::RefreshFarm => refresh_farm::RefreshFarm,
            KaminoFarmsInstruction::Stake => stake::Stake,
            KaminoFarmsInstruction::SetStakeDelegated => set_stake_delegated::SetStakeDelegated,
            KaminoFarmsInstruction::HarvestReward => harvest_reward::HarvestReward,
            KaminoFarmsInstruction::Unstake => unstake::Unstake,
            KaminoFarmsInstruction::RefreshUserState => refresh_user_state::RefreshUserState,
            KaminoFarmsInstruction::WithdrawUnstakedDeposits => withdraw_unstaked_deposits::WithdrawUnstakedDeposits,
            KaminoFarmsInstruction::WithdrawTreasury => withdraw_treasury::WithdrawTreasury,
            KaminoFarmsInstruction::DepositToFarmVault => deposit_to_farm_vault::DepositToFarmVault,
            KaminoFarmsInstruction::WithdrawFromFarmVault => withdraw_from_farm_vault::WithdrawFromFarmVault,
            KaminoFarmsInstruction::WithdrawSlashedAmount => withdraw_slashed_amount::WithdrawSlashedAmount,
            KaminoFarmsInstruction::UpdateFarmAdmin => update_farm_admin::UpdateFarmAdmin,
            KaminoFarmsInstruction::UpdateGlobalConfigAdmin => update_global_config_admin::UpdateGlobalConfigAdmin,
            KaminoFarmsInstruction::WithdrawReward => withdraw_reward::WithdrawReward,
            KaminoFarmsInstruction::IdlMissingTypes => idl_missing_types::IdlMissingTypes,
        )
    }
}
