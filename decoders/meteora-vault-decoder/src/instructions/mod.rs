use crate::PROGRAM_ID;

use super::MeteoraVaultDecoder;
pub mod add_liquidity_event;
pub mod add_strategy;
pub mod claim_reward_event;
pub mod collect_dust;
pub mod deposit;
pub mod deposit_strategy;
pub mod enable_vault;
pub mod initialize;
pub mod initialize_strategy;
pub mod performance_fee_event;
pub mod remove_liquidity_event;
pub mod remove_strategy;
pub mod remove_strategy2;
pub mod report_loss_event;
pub mod set_operator;
pub mod strategy_deposit_event;
pub mod strategy_withdraw_event;
pub mod total_amount_event;
pub mod withdraw;
pub mod withdraw2;
pub mod withdraw_directly_from_strategy;
pub mod withdraw_strategy;

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
pub enum MeteoraVaultInstruction {
    Initialize(initialize::Initialize),
    EnableVault(enable_vault::EnableVault),
    SetOperator(set_operator::SetOperator),
    InitializeStrategy(initialize_strategy::InitializeStrategy),
    RemoveStrategy(remove_strategy::RemoveStrategy),
    RemoveStrategy2(remove_strategy2::RemoveStrategy2),
    CollectDust(collect_dust::CollectDust),
    AddStrategy(add_strategy::AddStrategy),
    DepositStrategy(deposit_strategy::DepositStrategy),
    WithdrawStrategy(withdraw_strategy::WithdrawStrategy),
    Withdraw2(withdraw2::Withdraw2),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    WithdrawDirectlyFromStrategy(withdraw_directly_from_strategy::WithdrawDirectlyFromStrategy),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    StrategyDepositEvent(strategy_deposit_event::StrategyDepositEvent),
    StrategyWithdrawEvent(strategy_withdraw_event::StrategyWithdrawEvent),
    ClaimRewardEvent(claim_reward_event::ClaimRewardEvent),
    PerformanceFeeEvent(performance_fee_event::PerformanceFeeEvent),
    ReportLossEvent(report_loss_event::ReportLossEvent),
    TotalAmountEvent(total_amount_event::TotalAmountEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MeteoraVaultDecoder {
    type InstructionType = MeteoraVaultInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            MeteoraVaultInstruction::Initialize => initialize::Initialize,
            MeteoraVaultInstruction::EnableVault => enable_vault::EnableVault,
            MeteoraVaultInstruction::SetOperator => set_operator::SetOperator,
            MeteoraVaultInstruction::InitializeStrategy => initialize_strategy::InitializeStrategy,
            MeteoraVaultInstruction::RemoveStrategy => remove_strategy::RemoveStrategy,
            MeteoraVaultInstruction::RemoveStrategy2 => remove_strategy2::RemoveStrategy2,
            MeteoraVaultInstruction::CollectDust => collect_dust::CollectDust,
            MeteoraVaultInstruction::AddStrategy => add_strategy::AddStrategy,
            MeteoraVaultInstruction::DepositStrategy => deposit_strategy::DepositStrategy,
            MeteoraVaultInstruction::WithdrawStrategy => withdraw_strategy::WithdrawStrategy,
            MeteoraVaultInstruction::Withdraw2 => withdraw2::Withdraw2,
            MeteoraVaultInstruction::Deposit => deposit::Deposit,
            MeteoraVaultInstruction::Withdraw => withdraw::Withdraw,
            MeteoraVaultInstruction::WithdrawDirectlyFromStrategy => withdraw_directly_from_strategy::WithdrawDirectlyFromStrategy,
            MeteoraVaultInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            MeteoraVaultInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            MeteoraVaultInstruction::StrategyDepositEvent => strategy_deposit_event::StrategyDepositEvent,
            MeteoraVaultInstruction::StrategyWithdrawEvent => strategy_withdraw_event::StrategyWithdrawEvent,
            MeteoraVaultInstruction::ClaimRewardEvent => claim_reward_event::ClaimRewardEvent,
            MeteoraVaultInstruction::PerformanceFeeEvent => performance_fee_event::PerformanceFeeEvent,
            MeteoraVaultInstruction::ReportLossEvent => report_loss_event::ReportLossEvent,
            MeteoraVaultInstruction::TotalAmountEvent => total_amount_event::TotalAmountEvent,
        )
    }
}
