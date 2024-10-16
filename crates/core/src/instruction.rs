//! Provides structures and traits for decoding and processing instructions within transactions.
//!
//! The module includes the following main components:
//! - **`InstructionMetadata`**: Metadata associated with an instruction, capturing transaction context.
//! - **`DecodedInstruction`**: Represents an instruction that has been decoded, with associated program ID, data, and accounts.
//! - **`InstructionDecoder`**: A trait for decoding instructions into specific types.
//! - **`InstructionPipe`**: A structure that processes instructions using a decoder and a processor.
//! - **`InstructionPipes`**: An async trait for processing instructions within nested contexts.
//! - **`NestedInstruction`**: Represents instructions with potential nested inner instructions, allowing for recursive processing.
//!
//! These components enable the `carbon-core` framework to handle Solana transaction instructions efficiently,
//! decoding them into structured types and facilitating hierarchical processing.

use crate::{
    error::CarbonResult, metrics::Metrics, processor::Processor, transaction::TransactionMetadata,
};
use async_trait::async_trait;
use serde::Deserialize;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
use std::sync::Arc;

/// Metadata associated with a specific instruction, including transaction-level details.
///
/// `InstructionMetadata` is utilized within the pipeline to associate each instruction
/// with the broader context of its transaction, as well as its position within the
/// instruction stack.
///
/// # Fields
///
/// - `transaction_metadata`: Metadata providing details of the entire transaction.
/// - `stack_height`: Represents the instruction’s depth within the stack, where 0 is the root level.

#[derive(Debug, Clone)]
pub struct InstructionMetadata {
    pub transaction_metadata: TransactionMetadata,
    pub stack_height: u32,
}

/// A decoded instruction containing program ID, data, and associated accounts.
///
/// The `DecodedInstruction` struct represents the outcome of decoding a raw instruction,
/// encapsulating its program ID, parsed data, and the accounts involved.
///
/// # Type Parameters
///
/// - `T`: The type representing the decoded data for the instruction.
///
/// # Fields
///
/// - `program_id`: The program ID that owns the instruction.
/// - `data`: The decoded data payload for the instruction, of type `T`.
/// - `accounts`: A vector of `AccountMeta`, representing the accounts involved in the instruction.

#[derive(Debug, Clone, Deserialize)]
pub struct DecodedInstruction<T> {
    pub program_id: Pubkey,
    pub data: T,
    pub accounts: Vec<AccountMeta>,
}

/// A trait for decoding Solana instructions into a structured type.
///
/// Implement the `InstructionDecoder` trait for types that can decode raw instructions
/// into a more meaningful structure, providing application-specific logic.
///
/// # Type Parameters
///
/// - `InstructionType`: The type into which the instruction data will be decoded.
///
/// # Required Methods
///
/// - `decode_instruction`: Decodes a raw Solana `Instruction` into a `DecodedInstruction`.
pub trait InstructionDecoder<'a> {
    type InstructionType;

    fn decode_instruction(
        &self,
        instruction: &'a solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>>;
}

/// A processing pipeline for instructions, using a decoder and processor.
///
/// The `InstructionPipe` structure enables the processing of decoded instructions,
/// pairing an `InstructionDecoder` with a `Processor`. It supports generic instruction types.
///
/// # Type Parameters
///
/// - `T`: The type representing the decoded instruction data.
///
/// # Fields
///
/// - `decoder`: The decoder used for parsing instructions.
/// - `processor`: The processor that handles decoded instructions.
pub struct InstructionPipe<T: Send> {
    pub decoder:
        Box<dyn for<'a> InstructionDecoder<'a, InstructionType = T> + Send + Sync + 'static>,
    pub processor: Box<
        dyn Processor<
                InputType = (
                    InstructionMetadata,
                    DecodedInstruction<T>,
                    Vec<NestedInstruction>,
                ),
            > + Send
            + Sync
            + 'static,
    >,
}

/// An async trait for processing instructions within nested contexts.
///
/// The `InstructionPipes` trait allows for recursive processing of instructions that may contain
/// nested instructions. This enables complex, hierarchical instruction handling for transactions.
///
/// # Required Methods
///
/// - `run`: Processes a `NestedInstruction`, recursively processing any inner instructions.
///
#[async_trait]
pub trait InstructionPipes<'a>: Send + Sync {
    async fn run(
        &mut self,
        nested_instruction: &NestedInstruction,
        metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<T: Send + 'static> InstructionPipes<'_> for InstructionPipe<T> {
    async fn run(
        &mut self,
        nested_instruction: &NestedInstruction,
        metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()> {
        log::trace!(
            "InstructionPipe::run(nested_instruction: {:?}, metrics)",
            nested_instruction,
        );

        if let Some(decoded_instruction) = self
            .decoder
            .decode_instruction(&nested_instruction.instruction)
        {
            self.processor
                .process(
                    (
                        nested_instruction.metadata.clone(),
                        decoded_instruction,
                        nested_instruction.inner_instructions.clone(),
                    ),
                    metrics.clone(),
                )
                .await?;
        }

        for nested_inner_instruction in nested_instruction.inner_instructions.iter() {
            self.run(nested_inner_instruction, metrics.clone()).await?;
        }

        Ok(())
    }
}

/// Represents a nested instruction with metadata, including potential inner instructions.
///
/// The `NestedInstruction` struct allows for recursive instruction handling, where
/// each instruction may have associated metadata and a list of nested instructions.
///
/// # Fields
///
/// - `metadata`: The metadata associated with the instruction.
/// - `instruction`: The Solana instruction being processed.
/// - `inner_instructions`: A vector of `NestedInstruction`, representing any nested instructions.
///
#[derive(Debug, Clone)]
pub struct NestedInstruction {
    pub metadata: InstructionMetadata,
    pub instruction: solana_sdk::instruction::Instruction,
    pub inner_instructions: Vec<NestedInstruction>,
}
