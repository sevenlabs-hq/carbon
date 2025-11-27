//! Defines the structures and functions for constructing and matching
//! transaction schemas in `carbon-core`.
//!
//! This module provides the `TransactionSchema`, `SchemaNode`, and
//! `InstructionSchemaNode` types, enabling users to define and validate
//! transactions against a specific schema. Transaction schemas can be nested,
//! allowing for complex, multi-layered transaction structures that represent
//! various instructions and sub-instructions.
//!
//! ## Key Components
//!
//! - **TransactionSchema**: Represents the overall schema for a transaction,
//!   consisting of a collection of schema nodes at its root.
//! - **SchemaNode**: A node in the schema that can either be an instruction
//!   node or an `Any` node, allowing flexibility in matching instructions at
//!   that level.
//! - **InstructionSchemaNode**: Represents an instruction with its type, name,
//!   and any nested inner instructions.
//!
//! ## Usage
//!
//! The `TransactionSchema` type provides methods to match a given transaction’s
//! instructions against the schema and return a mapped representation of the
//! data if it conforms to the schema. The `match_schema` and `match_nodes`
//! methods allow for hierarchical matching and data extraction from
//! transactions.
//!
//! ## Notes
//!
//! - **Schema Matching**: Schema matching is sequential, with `Any` nodes
//!   providing flexibility in handling unknown instructions within
//!   transactions. Each `InstructionSchemaNode` defines specific instructions
//!   to be matched, allowing for strict validation where needed.
//! - **Nested Instructions**: Instruction schemas can contain nested
//!   instructions, enabling validation of complex transactions with inner
//!   instructions.
//! - **Data Conversion**: The `match_schema` method returns data as a
//!   deserialized type using `serde_json`. Ensure that your expected output
//!   type implements `DeserializeOwned`.

use {
    crate::{collection::InstructionDecoderCollection, instruction::DecodedInstruction},
    serde::de::DeserializeOwned,
    solana_instruction::AccountMeta,
    solana_pubkey::Pubkey,
    std::collections::HashMap,
};

/// Represents a node within a transaction schema, which can be either an
/// `Instruction` node or an `Any` node to allow for flexible matching.
#[derive(Debug, Clone)]
pub enum SchemaNode<T: InstructionDecoderCollection> {
    /// Represents a specific instruction type and its nested structure.
    Instruction(InstructionSchemaNode<T>),
    /// Matches any instruction type, providing flexibility within the schema.
    Any,
}

/// Represents an instruction node within a schema, containing the instruction
/// type, name, and optional nested instructions for further validation.
#[derive(Debug, Clone)]
pub struct InstructionSchemaNode<T: InstructionDecoderCollection> {
    /// The type of the instruction, as defined by the associated collection.
    pub ix_type: T::InstructionType,
    /// A unique name identifier for the instruction node within the schema.
    pub name: String,
    /// A vector of nested schema nodes for matching nested instructions.
    pub inner_instructions: Vec<SchemaNode<T>>,
}

/// Represents a parsed instruction, containing its program ID, decoded
/// instruction data, and any nested instructions within the transaction.
#[derive(Debug)]
pub struct ParsedInstruction<T: InstructionDecoderCollection> {
    /// The program ID associated with this instruction.
    pub program_id: Pubkey,
    /// The decoded instruction data.
    pub instruction: DecodedInstruction<T>,
    /// A vector of parsed nested instructions.
    pub inner_instructions: Vec<ParsedInstruction<T>>,
}

/// Represents the schema for a transaction, defining the structure and expected
/// instructions.
///
/// `TransactionSchema` allows you to define the structure of a transaction by
/// specifying a list of `SchemaNode` elements at the root level. These nodes
/// can represent specific instruction types or allow for flexibility with `Any`
/// nodes. Nested instructions are supported to enable complex hierarchical
/// schemas.
///
/// ## Methods
///
/// - `match_schema`: Attempts to match the transaction’s instructions against
///   the schema, returning a deserialized representation of the data.
/// - `match_nodes`: Matches the instructions against the schema nodes,
///   returning a mapping of instruction names to data, if successful.
#[derive(Debug, Clone)]
pub struct TransactionSchema<T: InstructionDecoderCollection> {
    pub root: Vec<SchemaNode<T>>,
}

impl<T: InstructionDecoderCollection> TransactionSchema<T> {
    /// Matches the transaction's instructions against the schema and returns a
    /// deserialized result.
    ///
    /// # Parameters
    ///
    /// - `instructions`: A slice of `ParsedInstruction` representing the
    ///   instructions to be matched.
    ///
    /// # Returns
    ///
    /// An `Option<U>` containing the deserialized data if matching and
    /// deserialization are successful. The U represents the expected output
    /// type, manually made by the developer.
    pub fn match_schema<U>(&self, instructions: &[ParsedInstruction<T>]) -> Option<U>
    where
        U: DeserializeOwned,
    {
        log::trace!("Schema::match_schema(self: {self:?}, instructions: {instructions:?})");
        let value = serde_json::to_value(self.match_nodes(instructions)).ok()?;

        log::trace!("Schema::match_schema: deserializing value: {value:?}");
        serde_json::from_value::<U>(value).ok()
    }

    /// Matches the instructions against the schema nodes and returns a mapping
    /// of instruction names to data.
    ///
    /// This method processes the instructions and checks them against the
    /// schema nodes sequentially. If the instructions match, a `HashMap` of
    /// instruction names to decoded data and associated accounts is returned.
    ///
    /// # Parameters
    ///
    /// - `instructions`: A slice of `ParsedInstruction` representing the
    ///   instructions to be matched.
    ///
    /// # Returns
    ///
    /// An `Option<HashMap<String, (T, Vec<AccountMeta>)>>` containing the
    /// matched instruction data, or `None` if the instructions do not match
    /// the schema.
    pub fn match_nodes(
        &self,
        instructions: &[ParsedInstruction<T>],
    ) -> Option<HashMap<String, (T, Vec<AccountMeta>)>> {
        log::trace!("Schema::match_nodes(self: {self:?}, instructions: {instructions:?})");
        let mut output = HashMap::<String, (T, Vec<AccountMeta>)>::new();

        let mut node_index = 0;
        let mut instruction_index = 0;

        let mut any = false;

        while let Some(node) = self.root.get(node_index) {
            log::trace!("Schema::match_nodes: current node ({node_index}): {node:?}");

            if let SchemaNode::Any = node {
                log::trace!("Schema::match_nodes: Any node detected, skipping");
                any = true;
                node_index += 1;
                continue;
            }

            let mut matched = false;

            while let Some(current_instruction) = instructions.get(instruction_index) {
                log::trace!(
                    "Schema::match_nodes: current instruction ({instruction_index}): {current_instruction:?}"
                );

                let SchemaNode::Instruction(instruction_node) = node else {
                    return None;
                };

                if current_instruction.instruction.data.get_type() != instruction_node.ix_type
                    && !any
                {
                    log::trace!(
                        "Schema::match_nodes: instruction type mismatch, returning (any = false)"
                    );
                    return None;
                }

                if current_instruction.instruction.data.get_type() != instruction_node.ix_type
                    && any
                {
                    log::trace!(
                        "Schema::match_nodes: instruction type mismatch, skipping (any = true)"
                    );
                    instruction_index += 1;
                    continue;
                }

                output.insert(
                    instruction_node.name.clone(),
                    (
                        current_instruction.instruction.data.clone(),
                        current_instruction.instruction.accounts.clone(),
                    ),
                );

                if !instruction_node.inner_instructions.is_empty() {
                    let inner_output = TransactionSchema {
                        root: instruction_node.inner_instructions.clone(),
                    }
                    .match_nodes(&current_instruction.inner_instructions)?;
                    output = merge_hashmaps(output, inner_output);
                }

                log::trace!("Schema::match_nodes: instruction matched, output: {output:?}");

                instruction_index += 1;
                node_index += 1;
                any = false;
                matched = true;
                break;
            }

            if !matched {
                log::trace!("Schema::match_nodes: node not matched, returning");
                return None;
            }
        }

        log::trace!("Schema::match_nodes: final output: {output:?}");

        Some(output)
    }
}

/// Merges two hash maps containing instruction data and account information.
///
/// # Parameters
///
/// - `a`: The first `HashMap` to be merged.
/// - `b`: The second `HashMap` to be merged.
///
/// # Returns
///
/// A new `HashMap` containing all elements from `a` and `b`. In the case of
/// duplicate keys, values from `b` will overwrite those from `a`.
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
