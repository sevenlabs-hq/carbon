use serde::Serialize;

use crate::instruction::DecodedInstruction;

pub trait InstructionDecoderCollection:
    Clone + std::fmt::Debug + Send + Sync + Eq + std::hash::Hash + Serialize + 'static
{
    type InstructionType: Clone + std::fmt::Debug + PartialEq + Eq + Send + Sync + 'static;

    fn parse_instruction(
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self>>;
    fn get_type(&self) -> Self::InstructionType;
}
