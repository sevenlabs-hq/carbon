//! Provides types and traits for handling transaction processing in the `carbon-core` framework.
//! It also provides utilities for matching transactions to schemas and executing custom processing
//! logic on matched data.
//!
//! ## Key Components
//!
//! - **TransactionPipe**: Represents a processing pipe for transactions, with functionality
//!   to parse and match instructions against a schema and handle matched data with a specified
//!   processor.
//! - **TransactionMetadata**: Metadata associated with a transaction, including slot, signature,
//!   and fee payer information.
//! - **ParsedTransaction**: Represents a transaction with its metadata and parsed instructions.
//!
//! ## Usage
//!
//! To use this module, create a `TransactionPipe` with a transaction schema and a processor.
//! Then, run the transaction pipe with a set of instructions and metrics to parse, match,
//! and process transaction data. The `run` method will asynchronously handle these steps.
//!
//! ## Notes
//!
//! - **Nested Instructions**: The `TransactionPipe` supports nested instructions within transactions,
//!   allowing for multi-level transaction processing.
//! - **Schema Matching**: The `TransactionPipe` will match transaction instructions against
//!   the provided schema, only processing the data if it conforms to the schema.

use crate::{
    collection::InstructionDecoderCollection,
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    metrics::MetricsCollection,
    processor::Processor,
    schema::{ParsedInstruction, TransactionSchema},
    transformers,
};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::convert::TryFrom;
use std::sync::Arc;
/// Contains metadata about a transaction, including its slot, signature, fee payer, transaction status metadata and the version transaction message.

#[derive(Debug, Clone)]
pub struct TransactionMetadata {
    pub slot: u64,
    pub signature: Signature,
    pub fee_payer: Pubkey,
    pub meta: solana_transaction_status::TransactionStatusMeta,
    pub message: solana_sdk::message::VersionedMessage,
}

/// Tries convert transaction update into the metadata.
///
/// This function retrieves core metadata such as the transaction's slot, signature, and
/// fee payer from the transaction's message. It ensures that these details are available
/// and ready for further processing.
///
/// # Parameters
///
/// - `transaction_update`: The `TransactionUpdate` containing the transaction details.
///
/// # Returns
///
/// A `CarbonResult<TransactionMetadata>` which includes the slot, signature, fee payer, transaction status metadata and the version transaction message.
///
/// # Errors
///
/// Returns an error if the fee payer cannot be extracted from the transaction's account keys.
impl TryFrom<crate::datasource::TransactionUpdate> for TransactionMetadata {
    type Error = crate::error::Error;

    fn try_from(value: crate::datasource::TransactionUpdate) -> Result<Self, Self::Error> {
        log::trace!(
            "try_from(transaction_update: {:?})",
            value
        );
        let accounts = value.transaction.message.static_account_keys();

        Ok(TransactionMetadata {
            slot: value.slot,
            signature: value.signature,
            fee_payer: *accounts
                .first()
                .ok_or(crate::error::Error::MissingFeePayer)?,
            meta: value.meta.clone(),
            message: value.transaction.message.clone(),
        })
    }
}

/// The input type for the transaction processor.
///
/// - `T`: The instruction type, implementing `InstructionDecoderCollection`.
/// - `U`: The output type for the matched data, if schema-matching, implementing `DeserializeOwned`.
pub type TransactionProcessorInputType<T, U = ()> = (
    TransactionMetadata,
    Vec<(InstructionMetadata, DecodedInstruction<T>)>,
    Option<U>,
);

/// A pipe for processing transactions based on a defined schema and processor.
///
/// The `TransactionPipe` parses a transaction's instructions, optionally checks them against the schema,
/// and runs the processor if the instructions match the schema. It provides methods for parsing
/// nested instructions and matching transaction data to the schema.
///
/// ## Generics
///
/// - `T`: The instruction type, implementing `InstructionDecoderCollection`.
/// - `U`: The output type for the matched data, if schema-matching, implementing `DeserializeOwned`.
pub struct TransactionPipe<T: InstructionDecoderCollection, U> {
    schema: Option<TransactionSchema<T>>,
    processor: Box<dyn Processor<InputType = TransactionProcessorInputType<T, U>> + Send + Sync>,
}

/// Represents a parsed transaction, including its metadata and parsed instructions.
pub struct ParsedTransaction<I: InstructionDecoderCollection> {
    pub metadata: TransactionMetadata,
    pub instructions: Vec<ParsedInstruction<I>>,
}

impl<T: InstructionDecoderCollection, U> TransactionPipe<T, U> {
    /// Creates a new `TransactionPipe` with the specified schema and processor.
    ///
    /// # Parameters
    ///
    /// - `schema`: The schema against which to match transaction instructions.
    /// - `processor`: The processor that will handle matched transaction data.
    ///
    /// # Returns
    ///
    /// A `TransactionPipe` instance configured with the specified schema and processor.

    pub fn new(
        schema: Option<TransactionSchema<T>>,
        processor: impl Processor<InputType = TransactionProcessorInputType<T, U>>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        log::trace!(
            "TransactionPipe::new(schema: {:?}, processor: {:?})",
            schema,
            stringify!(processor)
        );
        Self {
            schema,
            processor: Box::new(processor),
        }
    }

    /// Parses nested instructions into a list of `ParsedInstruction`.
    ///
    /// This method recursively traverses the nested instructions and parses each one, creating
    /// a structured representation of the instructions.
    ///
    /// # Parameters
    ///
    /// - `instructions`: A slice of `NestedInstruction` representing the instructions to be parsed.
    ///
    /// # Returns
    ///
    /// A `Box<Vec<ParsedInstruction<T>>>` containing the parsed instructions.
    fn parse_instructions(
        &self,
        instructions: &[NestedInstruction],
    ) -> Box<Vec<ParsedInstruction<T>>> {
        log::trace!(
            "TransactionPipe::parse_instructions(instructions: {:?})",
            instructions
        );

        let mut parsed_instructions: Box<Vec<ParsedInstruction<T>>> = Box::new(vec![]);

        instructions
            .iter()
            .enumerate()
            .for_each(|(_i, nested_instr)| {
                if let Some(parsed) = T::parse_instruction(&nested_instr.instruction) {
                    parsed_instructions.push(ParsedInstruction {
                        program_id: nested_instr.instruction.program_id,
                        instruction: parsed,
                        inner_instructions: self
                            .parse_instructions(&nested_instr.inner_instructions),
                    });
                }
            });

        parsed_instructions
    }

    /// Matches parsed instructions against the schema and returns the data as type `U`.
    ///
    /// The method only returns data if the parsed instructions conform to the schema.
    ///
    /// # Parameters
    ///
    /// - `instructions`: A slice of `ParsedInstruction` to be matched against the schema.
    ///
    /// # Returns
    ///
    /// An `Option<U>` containing the deserialized matched data if the instructions match the schema.
    fn matches_schema(&self, instructions: &[ParsedInstruction<T>]) -> Option<U>
    where
        U: DeserializeOwned,
    {
        match self.schema {
            Some(ref schema) => schema.match_schema(instructions),
            None => None,
        }
    }
}

/// Defines an asynchronous trait for processing transactions.
///
/// This trait provides a method for running transaction pipes, handling parsed instructions
/// with associated metrics, and leveraging `TransactionPipe` implementations.
#[async_trait]
pub trait TransactionPipes<'a>: Send + Sync {
    /// Runs the transaction pipe with the provided instructions and metrics.
    ///
    /// The method parses the instructions, matches them against the schema, and processes
    /// the matched data asynchronously.
    ///
    /// # Parameters
    ///
    /// - `instructions`: A slice of `NestedInstruction` containing the transaction instructions.
    /// - `metrics`: A vector of metrics instances for performance tracking.
    ///
    /// # Returns
    ///
    /// A `CarbonResult<()>` indicating success or failure.
    async fn run(
        &mut self,
        transaction_metadata: TransactionMetadata,
        instructions: &[NestedInstruction],
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<T, U> TransactionPipes<'_> for TransactionPipe<T, U>
where
    T: InstructionDecoderCollection + Sync + 'static,
    U: DeserializeOwned + Send + Sync + 'static,
{
    async fn run(
        &mut self,
        transaction_metadata: TransactionMetadata,
        instructions: &[NestedInstruction],
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!(
            "TransactionPipe::run(instructions: {:?}, metrics)",
            instructions,
        );

        let parsed_instructions = self.parse_instructions(&instructions);

        let matched_data = self.matches_schema(&parsed_instructions);

        let unnested_instructions = transformers::unnest_parsed_instructions(
            transaction_metadata.clone(),
            *parsed_instructions,
            0,
        );

        self.processor
            .process(
                (transaction_metadata, unnested_instructions, matched_data),
                metrics,
            )
            .await?;

        Ok(())
    }
}
