use crate::instruction::NestedInstruction;

/// An instruction and its decoded data.
#[derive(Debug)]
pub struct InstructionProcessorInput<'a, T> {
    pub(crate) instruction: &'a NestedInstruction,
    pub(crate) decoded: T,
}

impl<'a, T> InstructionProcessorInput<'a, T> {
    pub fn instruction(&self) -> &'a NestedInstruction {
        self.instruction
    }

    pub fn decoded(&self) -> &T {
        &self.decoded
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            instruction::{InstructionMetadata, NestedInstructions},
            transaction::TransactionMetadata,
        },
        solana_instruction::Instruction,
        solana_pubkey::Pubkey,
        std::sync::Arc,
    };

    #[test]
    fn input_borrows_instruction_and_owns_decoded_data() {
        let instruction = NestedInstruction {
            metadata: InstructionMetadata {
                transaction_metadata: Arc::new(TransactionMetadata::default()),
                stack_height: 1,
                index: 0,
                absolute_path: vec![0],
            },
            instruction: Instruction {
                program_id: Pubkey::new_unique(),
                accounts: vec![],
                data: vec![7],
            },
            inner_instructions: NestedInstructions::default(),
        };
        let decoded = String::from("decoded instruction");
        let decoded_ptr = decoded.as_ptr();
        let input = InstructionProcessorInput {
            instruction: &instruction,
            decoded,
        };

        assert!(std::ptr::eq(input.instruction(), &instruction));
        assert_eq!(input.decoded(), "decoded instruction");
        assert_eq!(input.decoded().as_ptr(), decoded_ptr);
        assert_eq!(input.instruction().metadata.absolute_path, vec![0]);
    }
}
