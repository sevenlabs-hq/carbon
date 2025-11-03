use {crate::MemoProgramDecoder, carbon_core::instruction::DecodedInstruction};

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
pub enum MemoProgramInstruction {
    Memo(Vec<u8>),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MemoProgramDecoder {
    type InstructionType = MemoProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&spl_memo_interface::v3::ID) {
            return None;
        }

        Some(DecodedInstruction {
            data: MemoProgramInstruction::Memo(instruction.data.clone()),
            program_id: instruction.program_id,
            accounts: instruction.accounts.clone(),
        })
    }
}
