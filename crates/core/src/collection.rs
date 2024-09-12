use crate::instruction::DecodedInstruction;

pub trait InstructionDecoderCollection: Clone + std::fmt::Debug + Send + Sync + 'static {
    type InstructionType: Clone + std::fmt::Debug + PartialEq + Eq + Send + Sync + 'static;

    fn parse_instruction<'a>(
        instruction: &'a solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self>>;
    fn get_type(&self) -> Self::InstructionType;
}
