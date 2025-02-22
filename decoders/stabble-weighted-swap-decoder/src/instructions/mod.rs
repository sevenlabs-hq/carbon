use crate::PROGRAM_ID;

use super::WeightedSwapDecoder;
pub mod accept_owner;
pub mod change_max_supply;
pub mod change_swap_fee;
pub mod deposit;
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
pub enum WeightedSwapInstruction {
    AcceptOwner(accept_owner::AcceptOwner),
    ChangeMaxSupply(change_max_supply::ChangeMaxSupply),
    ChangeSwapFee(change_swap_fee::ChangeSwapFee),
    Deposit(deposit::Deposit),
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

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for WeightedSwapDecoder {
    type InstructionType = WeightedSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            WeightedSwapInstruction::AcceptOwner => accept_owner::AcceptOwner,
            WeightedSwapInstruction::ChangeMaxSupply => change_max_supply::ChangeMaxSupply,
            WeightedSwapInstruction::ChangeSwapFee => change_swap_fee::ChangeSwapFee,
            WeightedSwapInstruction::Deposit => deposit::Deposit,
            WeightedSwapInstruction::Initialize => initialize::Initialize,
            WeightedSwapInstruction::Pause => pause::Pause,
            WeightedSwapInstruction::RejectOwner => reject_owner::RejectOwner,
            WeightedSwapInstruction::Shutdown => shutdown::Shutdown,
            WeightedSwapInstruction::Swap => swap::Swap,
            WeightedSwapInstruction::SwapV2 => swap_v2::SwapV2,
            WeightedSwapInstruction::TransferOwner => transfer_owner::TransferOwner,
            WeightedSwapInstruction::Unpause => unpause::Unpause,
            WeightedSwapInstruction::Withdraw => withdraw::Withdraw,
            WeightedSwapInstruction::PoolBalanceUpdatedEvent => pool_balance_updated_event::PoolBalanceUpdatedEvent,
            WeightedSwapInstruction::PoolUpdatedEvent => pool_updated_event::PoolUpdatedEvent,
        )
    }
}
