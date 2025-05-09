use {super::AddressLookupTableDecoder, crate::PROGRAM_ID};
pub mod close_lookup_table;
pub mod create_lookup_table;
pub mod deactivate_lookup_table;
pub mod extend_lookup_table;
pub mod freeze_lookup_table;

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

pub enum AddressLookupTableInstruction {
    CreateLookupTable(create_lookup_table::CreateLookupTable),
    FreezeLookupTable(freeze_lookup_table::FreezeLookupTable),
    ExtendLookupTable(extend_lookup_table::ExtendLookupTable),
    DeactivateLookupTable(deactivate_lookup_table::DeactivateLookupTable),
    CloseLookupTable(close_lookup_table::CloseLookupTable),
}

impl carbon_core::instruction::InstructionDecoder<'_> for AddressLookupTableDecoder {
    type InstructionType = AddressLookupTableInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            AddressLookupTableInstruction::CreateLookupTable => create_lookup_table::CreateLookupTable,
            AddressLookupTableInstruction::FreezeLookupTable => freeze_lookup_table::FreezeLookupTable,
            AddressLookupTableInstruction::ExtendLookupTable => extend_lookup_table::ExtendLookupTable,
            AddressLookupTableInstruction::DeactivateLookupTable => deactivate_lookup_table::DeactivateLookupTable,
            AddressLookupTableInstruction::CloseLookupTable => close_lookup_table::CloseLookupTable,
        )
    }
}
