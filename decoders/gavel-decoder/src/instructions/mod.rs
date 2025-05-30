use crate::PROGRAM_ID;

use super::GavelDecoder;
pub mod add_liquidity;
pub mod initialize_lp_position;
pub mod initialize_pool;
pub mod log;
pub mod remove_liquidity;
pub mod renounce_liquidity;
pub mod swap;
pub mod transfer_liquidity;
pub mod withdraw_lp_fees;
pub mod withdraw_protocol_fees;

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
pub enum GavelInstruction {
    Swap(swap::Swap),
    AddLiquidity(add_liquidity::AddLiquidity),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    RenounceLiquidity(renounce_liquidity::RenounceLiquidity),
    WithdrawLpFees(withdraw_lp_fees::WithdrawLpFees),
    InitializeLpPosition(initialize_lp_position::InitializeLpPosition),
    InitializePool(initialize_pool::InitializePool),
    WithdrawProtocolFees(withdraw_protocol_fees::WithdrawProtocolFees),
    Log(log::Log),
    TransferLiquidity(transfer_liquidity::TransferLiquidity),
}

impl carbon_core::instruction::InstructionDecoder<'_> for GavelDecoder {
    type InstructionType = GavelInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            GavelInstruction::Swap => swap::Swap,
            GavelInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            GavelInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            GavelInstruction::RenounceLiquidity => renounce_liquidity::RenounceLiquidity,
            GavelInstruction::WithdrawLpFees => withdraw_lp_fees::WithdrawLpFees,
            GavelInstruction::InitializeLpPosition => initialize_lp_position::InitializeLpPosition,
            GavelInstruction::InitializePool => initialize_pool::InitializePool,
            GavelInstruction::WithdrawProtocolFees => withdraw_protocol_fees::WithdrawProtocolFees,
            GavelInstruction::Log => log::Log,
            GavelInstruction::TransferLiquidity => transfer_liquidity::TransferLiquidity,
        )
    }
}
