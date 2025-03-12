



use super::MarinadeFinanceDecoder;
pub mod initialize;
pub mod change_authority;
pub mod add_validator;
pub mod remove_validator;
pub mod set_validator_score;
pub mod config_validator_system;
pub mod deposit;
pub mod deposit_stake_account;
pub mod liquid_unstake;
pub mod add_liquidity;
pub mod remove_liquidity;
pub mod config_lp;
pub mod config_marinade;
pub mod order_unstake;
pub mod claim;
pub mod stake_reserve;
pub mod update_active;
pub mod update_deactivated;
pub mod deactivate_stake;
pub mod emergency_unstake;
pub mod partial_unstake;
pub mod merge_stakes;
pub mod redelegate;
pub mod pause;
pub mod resume;
pub mod withdraw_stake_account;
pub mod realloc_validator_list;
pub mod realloc_stake_list;
pub mod change_authority_event;
pub mod config_lp_event;
pub mod config_marinade_event;
pub mod initialize_event;
pub mod emergency_pause_event;
pub mod resume_event;
pub mod realloc_validator_list_event;
pub mod realloc_stake_list_event;
pub mod deactivate_stake_event;
pub mod merge_stakes_event;
pub mod redelegate_event;
pub mod stake_reserve_event;
pub mod update_active_event;
pub mod update_deactivated_event;
pub mod claim_event;
pub mod order_unstake_event;
pub mod add_liquidity_event;
pub mod liquid_unstake_event;
pub mod remove_liquidity_event;
pub mod add_validator_event;
pub mod remove_validator_event;
pub mod set_validator_score_event;
pub mod deposit_stake_account_event;
pub mod deposit_event;
pub mod withdraw_stake_account_event;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum MarinadeFinanceInstruction {
    Initialize(initialize::Initialize),
    ChangeAuthority(change_authority::ChangeAuthority),
    AddValidator(add_validator::AddValidator),
    RemoveValidator(remove_validator::RemoveValidator),
    SetValidatorScore(set_validator_score::SetValidatorScore),
    ConfigValidatorSystem(config_validator_system::ConfigValidatorSystem),
    Deposit(deposit::Deposit),
    DepositStakeAccount(deposit_stake_account::DepositStakeAccount),
    LiquidUnstake(liquid_unstake::LiquidUnstake),
    AddLiquidity(add_liquidity::AddLiquidity),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    ConfigLp(config_lp::ConfigLp),
    ConfigMarinade(config_marinade::ConfigMarinade),
    OrderUnstake(order_unstake::OrderUnstake),
    Claim(claim::Claim),
    StakeReserve(stake_reserve::StakeReserve),
    UpdateActive(update_active::UpdateActive),
    UpdateDeactivated(update_deactivated::UpdateDeactivated),
    DeactivateStake(deactivate_stake::DeactivateStake),
    EmergencyUnstake(emergency_unstake::EmergencyUnstake),
    PartialUnstake(partial_unstake::PartialUnstake),
    MergeStakes(merge_stakes::MergeStakes),
    Redelegate(redelegate::Redelegate),
    Pause(pause::Pause),
    Resume(resume::Resume),
    WithdrawStakeAccount(withdraw_stake_account::WithdrawStakeAccount),
    ReallocValidatorList(realloc_validator_list::ReallocValidatorList),
    ReallocStakeList(realloc_stake_list::ReallocStakeList),
    ChangeAuthorityEvent(change_authority_event::ChangeAuthorityEvent),
    ConfigLpEvent(config_lp_event::ConfigLpEvent),
    ConfigMarinadeEvent(config_marinade_event::ConfigMarinadeEvent),
    InitializeEvent(initialize_event::InitializeEvent),
    EmergencyPauseEvent(emergency_pause_event::EmergencyPauseEvent),
    ResumeEvent(resume_event::ResumeEvent),
    ReallocValidatorListEvent(realloc_validator_list_event::ReallocValidatorListEvent),
    ReallocStakeListEvent(realloc_stake_list_event::ReallocStakeListEvent),
    DeactivateStakeEvent(deactivate_stake_event::DeactivateStakeEvent),
    MergeStakesEvent(merge_stakes_event::MergeStakesEvent),
    RedelegateEvent(redelegate_event::RedelegateEvent),
    StakeReserveEvent(stake_reserve_event::StakeReserveEvent),
    UpdateActiveEvent(update_active_event::UpdateActiveEvent),
    UpdateDeactivatedEvent(update_deactivated_event::UpdateDeactivatedEvent),
    ClaimEvent(claim_event::ClaimEvent),
    OrderUnstakeEvent(order_unstake_event::OrderUnstakeEvent),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    LiquidUnstakeEvent(liquid_unstake_event::LiquidUnstakeEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    AddValidatorEvent(add_validator_event::AddValidatorEvent),
    RemoveValidatorEvent(remove_validator_event::RemoveValidatorEvent),
    SetValidatorScoreEvent(set_validator_score_event::SetValidatorScoreEvent),
    DepositStakeAccountEvent(deposit_stake_account_event::DepositStakeAccountEvent),
    DepositEvent(deposit_event::DepositEvent),
    WithdrawStakeAccountEvent(withdraw_stake_account_event::WithdrawStakeAccountEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for MarinadeFinanceDecoder {
    type InstructionType = MarinadeFinanceInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MarinadeFinanceInstruction::Initialize => initialize::Initialize,
            MarinadeFinanceInstruction::ChangeAuthority => change_authority::ChangeAuthority,
            MarinadeFinanceInstruction::AddValidator => add_validator::AddValidator,
            MarinadeFinanceInstruction::RemoveValidator => remove_validator::RemoveValidator,
            MarinadeFinanceInstruction::SetValidatorScore => set_validator_score::SetValidatorScore,
            MarinadeFinanceInstruction::ConfigValidatorSystem => config_validator_system::ConfigValidatorSystem,
            MarinadeFinanceInstruction::Deposit => deposit::Deposit,
            MarinadeFinanceInstruction::DepositStakeAccount => deposit_stake_account::DepositStakeAccount,
            MarinadeFinanceInstruction::LiquidUnstake => liquid_unstake::LiquidUnstake,
            MarinadeFinanceInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            MarinadeFinanceInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            MarinadeFinanceInstruction::ConfigLp => config_lp::ConfigLp,
            MarinadeFinanceInstruction::ConfigMarinade => config_marinade::ConfigMarinade,
            MarinadeFinanceInstruction::OrderUnstake => order_unstake::OrderUnstake,
            MarinadeFinanceInstruction::Claim => claim::Claim,
            MarinadeFinanceInstruction::StakeReserve => stake_reserve::StakeReserve,
            MarinadeFinanceInstruction::UpdateActive => update_active::UpdateActive,
            MarinadeFinanceInstruction::UpdateDeactivated => update_deactivated::UpdateDeactivated,
            MarinadeFinanceInstruction::DeactivateStake => deactivate_stake::DeactivateStake,
            MarinadeFinanceInstruction::EmergencyUnstake => emergency_unstake::EmergencyUnstake,
            MarinadeFinanceInstruction::PartialUnstake => partial_unstake::PartialUnstake,
            MarinadeFinanceInstruction::MergeStakes => merge_stakes::MergeStakes,
            MarinadeFinanceInstruction::Redelegate => redelegate::Redelegate,
            MarinadeFinanceInstruction::Pause => pause::Pause,
            MarinadeFinanceInstruction::Resume => resume::Resume,
            MarinadeFinanceInstruction::WithdrawStakeAccount => withdraw_stake_account::WithdrawStakeAccount,
            MarinadeFinanceInstruction::ReallocValidatorList => realloc_validator_list::ReallocValidatorList,
            MarinadeFinanceInstruction::ReallocStakeList => realloc_stake_list::ReallocStakeList,
            MarinadeFinanceInstruction::ChangeAuthorityEvent => change_authority_event::ChangeAuthorityEvent,
            MarinadeFinanceInstruction::ConfigLpEvent => config_lp_event::ConfigLpEvent,
            MarinadeFinanceInstruction::ConfigMarinadeEvent => config_marinade_event::ConfigMarinadeEvent,
            MarinadeFinanceInstruction::InitializeEvent => initialize_event::InitializeEvent,
            MarinadeFinanceInstruction::EmergencyPauseEvent => emergency_pause_event::EmergencyPauseEvent,
            MarinadeFinanceInstruction::ResumeEvent => resume_event::ResumeEvent,
            MarinadeFinanceInstruction::ReallocValidatorListEvent => realloc_validator_list_event::ReallocValidatorListEvent,
            MarinadeFinanceInstruction::ReallocStakeListEvent => realloc_stake_list_event::ReallocStakeListEvent,
            MarinadeFinanceInstruction::DeactivateStakeEvent => deactivate_stake_event::DeactivateStakeEvent,
            MarinadeFinanceInstruction::MergeStakesEvent => merge_stakes_event::MergeStakesEvent,
            MarinadeFinanceInstruction::RedelegateEvent => redelegate_event::RedelegateEvent,
            MarinadeFinanceInstruction::StakeReserveEvent => stake_reserve_event::StakeReserveEvent,
            MarinadeFinanceInstruction::UpdateActiveEvent => update_active_event::UpdateActiveEvent,
            MarinadeFinanceInstruction::UpdateDeactivatedEvent => update_deactivated_event::UpdateDeactivatedEvent,
            MarinadeFinanceInstruction::ClaimEvent => claim_event::ClaimEvent,
            MarinadeFinanceInstruction::OrderUnstakeEvent => order_unstake_event::OrderUnstakeEvent,
            MarinadeFinanceInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            MarinadeFinanceInstruction::LiquidUnstakeEvent => liquid_unstake_event::LiquidUnstakeEvent,
            MarinadeFinanceInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            MarinadeFinanceInstruction::AddValidatorEvent => add_validator_event::AddValidatorEvent,
            MarinadeFinanceInstruction::RemoveValidatorEvent => remove_validator_event::RemoveValidatorEvent,
            MarinadeFinanceInstruction::SetValidatorScoreEvent => set_validator_score_event::SetValidatorScoreEvent,
            MarinadeFinanceInstruction::DepositStakeAccountEvent => deposit_stake_account_event::DepositStakeAccountEvent,
            MarinadeFinanceInstruction::DepositEvent => deposit_event::DepositEvent,
            MarinadeFinanceInstruction::WithdrawStakeAccountEvent => withdraw_stake_account_event::WithdrawStakeAccountEvent,
        )
    }
}