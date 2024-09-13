use crate::collection::InstructionDecoderCollection;
use crate::instruction::DecodedInstruction;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct SchemaNode<T: InstructionDecoderCollection> {
    pub ix_type: T::InstructionType,
    pub name: String,
    pub inner_instructions: Vec<SchemaNode<T>>,
}

#[derive(Debug)]
pub struct ParsedInstruction<T: InstructionDecoderCollection> {
    pub program_id: Pubkey,
    pub instruction: DecodedInstruction<T>,
    pub inner_instructions: Vec<ParsedInstruction<T>>,
}

impl<T: InstructionDecoderCollection> SchemaNode<T> {
    pub fn matches(&self, instruction: &ParsedInstruction<T>) -> bool {
        if self.ix_type != instruction.instruction.data.get_type() {
            return false;
        }

        for (schema_inner, parsed_inner) in self
            .inner_instructions
            .iter()
            .zip(&instruction.inner_instructions)
        {
            if !schema_inner.matches(parsed_inner) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct TransactionSchema<T: InstructionDecoderCollection> {
    pub root: Vec<SchemaNode<T>>,
}

impl<T: InstructionDecoderCollection> TransactionSchema<T> {
    pub fn matches(&self, instructions: &[ParsedInstruction<T>]) -> bool {
        self.match_instructions(&self.root, instructions).is_some()
    }

    fn match_instructions(
        &self,
        nodes: &[SchemaNode<T>],
        instructions: &[ParsedInstruction<T>],
    ) -> Option<usize> {
        let mut total_matched = 0;

        for node in nodes {
            if let Some(matched_count) =
                self.match_single_node(node, &instructions[total_matched..])
            {
                total_matched += matched_count;
            } else {
                return None;
            }
        }

        if total_matched == instructions.len() {
            Some(total_matched)
        } else {
            None
        }
    }

    fn match_single_node(
        &self,
        node: &SchemaNode<T>,
        instructions: &[ParsedInstruction<T>],
    ) -> Option<usize> {
        if instructions.is_empty() {
            return None;
        }

        if !self.instruction_matches_node(&instructions[0], node) {
            return None;
        }

        let mut matched_count = 1;

        if !node.inner_instructions.is_empty() {
            if let Some(inner_matched) = self.match_instructions(
                &node.inner_instructions,
                &instructions[0].inner_instructions,
            ) {
                if inner_matched != instructions[0].inner_instructions.len() {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(matched_count)
    }

    fn instruction_matches_node(
        &self,
        instruction: &ParsedInstruction<T>,
        node: &SchemaNode<T>,
    ) -> bool {
        instruction.instruction.data.get_type() == node.ix_type
    }
}
