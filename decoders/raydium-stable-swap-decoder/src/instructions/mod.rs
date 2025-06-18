use crate::PROGRAM_ID;

use super::RaydiumStableSwapAmmDecoder;
pub mod deposit;
pub mod initialize;
pub mod pre_initialize;
pub mod swap_base_in;
pub mod swap_base_out;
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
pub enum RaydiumStableSwapAmmInstruction {
    Initialize(initialize::Initialize),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    SwapBaseIn(swap_base_in::SwapBaseIn),
    PreInitialize(pre_initialize::PreInitialize),
    SwapBaseOut(swap_base_out::SwapBaseOut),
}

impl carbon_core::instruction::InstructionDecoder<'_> for RaydiumStableSwapAmmDecoder {
    type InstructionType = RaydiumStableSwapAmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            RaydiumStableSwapAmmInstruction::Initialize => initialize::Initialize,
            RaydiumStableSwapAmmInstruction::Deposit => deposit::Deposit,
            RaydiumStableSwapAmmInstruction::Withdraw => withdraw::Withdraw,
            RaydiumStableSwapAmmInstruction::SwapBaseIn => swap_base_in::SwapBaseIn,
            RaydiumStableSwapAmmInstruction::PreInitialize => pre_initialize::PreInitialize,
            RaydiumStableSwapAmmInstruction::SwapBaseOut => swap_base_out::SwapBaseOut,
        )
    }
}
