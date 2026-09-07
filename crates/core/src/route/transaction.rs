use {
    super::InstructionProcessorInput,
    crate::{instruction::NestedInstructions, update::TransactionUpdate},
};

/// A transaction update and its decoded instructions in depth-first preorder.
#[derive(Debug)]
pub struct TransactionProcessorInput<'a, T> {
    pub(crate) update: &'a TransactionUpdate,
    pub(crate) instructions: &'a [InstructionProcessorInput<'a, T>],
}

impl<'a, T> TransactionProcessorInput<'a, T> {
    pub fn update(&self) -> &'a TransactionUpdate {
        self.update
    }

    pub fn instructions(&self) -> &[InstructionProcessorInput<'a, T>] {
        self.instructions
    }
}

/// A transaction update and its complete instruction tree.
#[derive(Debug)]
pub struct TransactionFilterInput {
    pub(crate) update: TransactionUpdate,
    pub(crate) instructions: NestedInstructions,
}

impl TransactionFilterInput {
    pub fn update(&self) -> &TransactionUpdate {
        &self.update
    }

    pub fn instructions(&self) -> &NestedInstructions {
        &self.instructions
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            instruction::{InstructionMetadata, NestedInstruction},
            transaction::TransactionMetadata,
        },
        solana_instruction::Instruction,
        solana_pubkey::Pubkey,
        solana_signature::Signature,
        solana_transaction::versioned::VersionedTransaction,
        solana_transaction_status::TransactionStatusMeta,
        std::sync::Arc,
    };

    #[test]
    fn processor_input_borrows_update_and_original_tree_nodes() {
        let update = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default()],
                ..Default::default()
            },
            TransactionStatusMeta::default(),
            7,
        )
        .unwrap();
        let metadata = Arc::new(TransactionMetadata::default());
        let node = |path: Vec<u8>| NestedInstruction {
            metadata: InstructionMetadata {
                transaction_metadata: metadata.clone(),
                stack_height: path.len() as u32,
                index: 0,
                absolute_path: path,
            },
            instruction: Instruction {
                program_id: Pubkey::new_unique(),
                accounts: vec![],
                data: vec![],
            },
            inner_instructions: NestedInstructions::default(),
        };
        let mut root = node(vec![0]);
        root.inner_instructions.push(node(vec![0, 0]));
        let filter_input = TransactionFilterInput {
            update,
            instructions: NestedInstructions(vec![root]),
        };
        let child = &filter_input.instructions()[0].inner_instructions[0];
        let decoded = vec![InstructionProcessorInput {
            instruction: child,
            decoded: String::from("decoded child"),
        }];
        let input = TransactionProcessorInput {
            update: filter_input.update(),
            instructions: &decoded,
        };

        assert!(std::ptr::eq(input.update(), filter_input.update()));
        assert!(std::ptr::eq(input.instructions(), decoded.as_slice()));
        assert!(std::ptr::eq(input.instructions()[0].instruction(), child));
        assert_eq!(input.instructions()[0].decoded(), "decoded child");
        assert_eq!(filter_input.instructions().len(), 1);
        assert_eq!(filter_input.instructions()[0].inner_instructions.len(), 1);

        let empty = TransactionProcessorInput::<String> {
            update: filter_input.update(),
            instructions: &[],
        };
        assert!(empty.instructions().is_empty());
    }
}
