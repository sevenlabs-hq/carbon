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
        if instruction.data.len() < 4 {
            return None;
        }
        let (discriminator, data) = instruction.data.split_at(4);
        let entry = match discriminator {
            [0, 0, 0, 0] => {
                let args: create_lookup_table::CreateLookupTable =
                    bincode::deserialize(data).ok()?;
                AddressLookupTableInstruction::CreateLookupTable(args)
            }
            [1, 0, 0, 0] => {
                let args: freeze_lookup_table::FreezeLookupTable =
                    bincode::deserialize(data).ok()?;
                AddressLookupTableInstruction::FreezeLookupTable(args)
            }
            [2, 0, 0, 0] => {
                let args: extend_lookup_table::ExtendLookupTable =
                    bincode::deserialize(data).ok()?;
                AddressLookupTableInstruction::ExtendLookupTable(args)
            }
            [3, 0, 0, 0] => {
                let args: deactivate_lookup_table::DeactivateLookupTable =
                    bincode::deserialize(data).ok()?;

                AddressLookupTableInstruction::DeactivateLookupTable(args)
            }
            [4, 0, 0, 0] => {
                let args: close_lookup_table::CloseLookupTable = bincode::deserialize(data).ok()?;
                AddressLookupTableInstruction::CloseLookupTable(args)
            }
            _ => return None,
        };

        Some(carbon_core::instruction::DecodedInstruction {
            program_id: instruction.program_id,
            accounts: instruction.accounts.clone(),
            data: entry,
        })
    }
}
