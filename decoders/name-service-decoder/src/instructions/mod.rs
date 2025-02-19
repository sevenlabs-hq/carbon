use super::NameDecoder;
pub mod create;
pub mod delete;
pub mod realloc;
pub mod transfer;
pub mod update;

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
pub enum NameInstruction {
    Create(create::Create),
    Update(update::Update),
    Transfer(transfer::Transfer),
    Delete(delete::Delete),
    Realloc(realloc::Realloc),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for NameDecoder {
    type InstructionType = NameInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            NameInstruction::Create => create::Create,
            NameInstruction::Update => update::Update,
            NameInstruction::Transfer => transfer::Transfer,
            NameInstruction::Delete => delete::Delete,
            NameInstruction::Realloc => realloc::Realloc,
        )
    }
}
