use crate::PROGRAM_ID;

use super::StableSwapDecoder;
pub mod accept_owner;
pub mod approve_strategy;
pub mod change_amp_factor;
pub mod change_max_supply;
pub mod change_swap_fee;
pub mod create_strategy;
pub mod deposit;
pub mod exec_strategy;
pub mod initialize;
pub mod pause;
pub mod pool_balance_updated_event;
pub mod pool_updated_event;
pub mod reject_owner;
pub mod shutdown;
pub mod swap;
pub mod swap_v2;
pub mod transfer_owner;
pub mod unpause;
pub mod withdraw;

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
pub enum StableSwapInstruction {
    AcceptOwner(accept_owner::AcceptOwner),
    ApproveStrategy(approve_strategy::ApproveStrategy),
    ChangeAmpFactor(change_amp_factor::ChangeAmpFactor),
    ChangeMaxSupply(change_max_supply::ChangeMaxSupply),
    ChangeSwapFee(change_swap_fee::ChangeSwapFee),
    CreateStrategy(create_strategy::CreateStrategy),
    Deposit(deposit::Deposit),
    ExecStrategy(exec_strategy::ExecStrategy),
    Initialize(initialize::Initialize),
    Pause(pause::Pause),
    RejectOwner(reject_owner::RejectOwner),
    Shutdown(shutdown::Shutdown),
    Swap(swap::Swap),
    SwapV2(swap_v2::SwapV2),
    TransferOwner(transfer_owner::TransferOwner),
    Unpause(unpause::Unpause),
    Withdraw(withdraw::Withdraw),
    PoolBalanceUpdatedEvent(pool_balance_updated_event::PoolBalanceUpdatedEvent),
    PoolUpdatedEvent(pool_updated_event::PoolUpdatedEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for StableSwapDecoder {
    type InstructionType = StableSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            StableSwapInstruction::AcceptOwner => accept_owner::AcceptOwner,
            StableSwapInstruction::ApproveStrategy => approve_strategy::ApproveStrategy,
            StableSwapInstruction::ChangeAmpFactor => change_amp_factor::ChangeAmpFactor,
            StableSwapInstruction::ChangeMaxSupply => change_max_supply::ChangeMaxSupply,
            StableSwapInstruction::ChangeSwapFee => change_swap_fee::ChangeSwapFee,
            StableSwapInstruction::CreateStrategy => create_strategy::CreateStrategy,
            StableSwapInstruction::Deposit => deposit::Deposit,
            StableSwapInstruction::ExecStrategy => exec_strategy::ExecStrategy,
            StableSwapInstruction::Initialize => initialize::Initialize,
            StableSwapInstruction::Pause => pause::Pause,
            StableSwapInstruction::RejectOwner => reject_owner::RejectOwner,
            StableSwapInstruction::Shutdown => shutdown::Shutdown,
            StableSwapInstruction::Swap => swap::Swap,
            StableSwapInstruction::SwapV2 => swap_v2::SwapV2,
            StableSwapInstruction::TransferOwner => transfer_owner::TransferOwner,
            StableSwapInstruction::Unpause => unpause::Unpause,
            StableSwapInstruction::Withdraw => withdraw::Withdraw,
            StableSwapInstruction::PoolBalanceUpdatedEvent => pool_balance_updated_event::PoolBalanceUpdatedEvent,
            StableSwapInstruction::PoolUpdatedEvent => pool_updated_event::PoolUpdatedEvent,
        )
    }
}
