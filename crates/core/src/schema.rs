use crate::collection::InstructionDecoderCollection;
use crate::instruction::DecodedInstruction;
use serde::de::DeserializeOwned;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SchemaNode<T: InstructionDecoderCollection> {
    Instruction(InstructionSchemaNode<T>),
    Any,
}

#[derive(Debug, Clone)]
pub struct InstructionSchemaNode<T: InstructionDecoderCollection> {
    pub ix_type: T::InstructionType,
    pub name: String,
    pub inner_instructions: Vec<SchemaNode<T>>,
}

#[derive(Debug)]
pub struct ParsedInstruction<T: InstructionDecoderCollection> {
    pub program_id: Pubkey,
    pub instruction: DecodedInstruction<T>,
    pub inner_instructions: Box<Vec<ParsedInstruction<T>>>,
}

#[derive(Debug, Clone)]
pub struct TransactionSchema<T: InstructionDecoderCollection> {
    pub root: Vec<SchemaNode<T>>,
}

impl<T: InstructionDecoderCollection> TransactionSchema<T> {
    pub fn match_schema<U>(&self, instructions: &[ParsedInstruction<T>]) -> Option<U>
    where
        U: DeserializeOwned,
    {
        serde_json::from_value::<U>(serde_json::to_value(self.match_nodes(instructions)).ok()?).ok()
    }

    pub fn match_nodes(
        &self,
        instructions: &[ParsedInstruction<T>],
    ) -> Option<HashMap<String, (T, Vec<AccountMeta>)>> {
        log::trace!(
            "Schema::match_nodes(self: {:?}, instructions: {:?})",
            self,
            instructions
        );
        let mut output = HashMap::<String, (T, Vec<AccountMeta>)>::new();

        let current_index = 0;
        let current_instruction = instructions.get(current_index)?;

        let mut any = false;

        for node in self.root.iter() {
            match node {
                SchemaNode::Any => {
                    any = true;
                }
                SchemaNode::Instruction(ix) => {
                    if ix.ix_type != current_instruction.instruction.data.get_type() {
                        if !any {
                            return None;
                        } else {
                            continue;
                        }
                    } else {
                        output.insert(
                            ix.name.clone(),
                            (
                                current_instruction.instruction.data.clone(),
                                current_instruction.instruction.accounts.clone(),
                            ),
                        );
                    }

                    if !ix.inner_instructions.is_empty() {
                        match self.match_nodes(&current_instruction.inner_instructions) {
                            Some(inner_output) => {
                                output = merge_hashmaps(output, inner_output);
                            }
                            None => {
                                if !any {
                                    return None;
                                } else {
                                    continue;
                                }
                            }
                        }
                    }

                    any = false;
                }
            }
        }

        Some(output)
    }
}

pub fn merge_hashmaps<K, V>(
    a: HashMap<K, (V, Vec<AccountMeta>)>,
    b: HashMap<K, (V, Vec<AccountMeta>)>,
) -> HashMap<K, (V, Vec<AccountMeta>)>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    log::trace!("merge_hashmaps(a, b)");
    let mut output = a;
    for (key, value) in b {
        output.insert(key, value);
    }
    output
}
