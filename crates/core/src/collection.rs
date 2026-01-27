use {crate::instruction::DecodedInstruction, serde::Serialize};

pub trait InstructionDecoderCollection:
    Clone + std::fmt::Debug + Send + Sync + Eq + std::hash::Hash + Serialize + 'static
{
    type InstructionType: Clone + std::fmt::Debug + PartialEq + Eq + Send + Sync + 'static;
    type ArrangedAccounts: Clone + std::fmt::Debug + Send + Sync + Serialize + 'static;

    fn parse_instruction(
        instruction: &solana_instruction::Instruction,
    ) -> Option<DecodedInstruction<Self, Self::ArrangedAccounts>>;
    fn get_type(&self) -> Self::InstructionType;
}
