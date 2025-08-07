//! Provides types and traits for handling transaction processing in the
//! `carbon-core` framework. It also provides utilities for matching
//! transactions to schemas and executing custom processing logic on matched
//! data.
//!
//! ## Key Components
//!
//! - **TransactionPipe**: Represents a processing pipe for transactions, with
//!   functionality to parse and match instructions against a schema and handle
//!   matched data with a specified processor.
//! - **TransactionMetadata**: Metadata associated with a transaction, including
//!   slot, signature, and fee payer information.
//! - **ParsedTransaction**: Represents a transaction with its metadata and
//!   parsed instructions.
//!
//! ## Usage
//!
//! To use this module, create a `TransactionPipe` with a transaction schema and
//! a processor. Then, run the transaction pipe with a set of instructions and
//! metrics to parse, match, and process transaction data. The `run` method will
//! asynchronously handle these steps.
//!
//! ## Notes
//!
//! - **Nested Instructions**: The `TransactionPipe` supports nested
//!   instructions within transactions, allowing for multi-level transaction
//!   processing.
//! - **Schema Matching**: The `TransactionPipe` will match transaction
//!   instructions against the provided schema, only processing the data if it
//!   conforms to the schema.

use crate::filter::Filter;
use solana_program::hash::Hash;

use {
    crate::{
        collection::InstructionDecoderCollection,
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
        metrics::MetricsCollection,
        processor::Processor,
        schema::{ParsedInstruction, TransactionSchema},
        transformers,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
    serde::de::DeserializeOwned,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::sync::Arc,
};
/// Contains metadata about a transaction, including its slot, signature, fee
/// payer, transaction status metadata, the version transaction message and its
/// block time.
///
/// # Fields
/// - `slot`: The slot number in which this transaction was processed
/// - `signature`: The unique signature of this transaction
/// - `fee_payer`: The public key of the fee payer account that paid for this
///   transaction
/// - `meta`: Transaction status metadata containing execution status, fees,
///   balances, and other metadata
/// - `message`: The versioned message containing the transaction instructions
///   and account keys
/// - `block_time`: The Unix timestamp of when the transaction was processed.
///
/// Note: The `block_time` field may not be returned in all scenarios.
#[derive(Debug, Clone, Default)]
pub struct TransactionMetadata {
    pub slot: u64,
    pub signature: Signature,
    pub fee_payer: Pubkey,
    pub meta: solana_transaction_status::TransactionStatusMeta,
    pub message: solana_program::message::VersionedMessage,
    pub block_time: Option<i64>,
    pub block_hash: Option<Hash>,
}

/// Tries convert transaction update into the metadata.
///
/// This function retrieves core metadata such as the transaction's slot,
/// signature, and fee payer from the transaction's message. It ensures that
/// these details are available and ready for further processing.
///
/// # Parameters
///
/// - `transaction_update`: The `TransactionUpdate` containing the transaction
///   details.
///
/// # Returns
///
/// A `CarbonResult<TransactionMetadata>` which includes the slot, signature,
/// fee payer, transaction status metadata and the version transaction message.
///
/// # Errors
///
/// Returns an error if the fee payer cannot be extracted from the transaction's
/// account keys.
impl TryFrom<crate::datasource::TransactionUpdate> for TransactionMetadata {
    type Error = crate::error::Error;

    fn try_from(value: crate::datasource::TransactionUpdate) -> Result<Self, Self::Error> {
        log::trace!("try_from(transaction_update: {:?})", value);
        let accounts = value.transaction.message.static_account_keys();

        Ok(TransactionMetadata {
            slot: value.slot,
            signature: value.signature,
            fee_payer: *accounts
                .first()
                .ok_or(crate::error::Error::MissingFeePayer)?,
            meta: value.meta.clone(),
            message: value.transaction.message.clone(),
            block_time: value.block_time,
            block_hash: value.block_hash,
        })
    }
}

/// The input type for the transaction processor.
///
/// - `T`: The instruction type, implementing `InstructionDecoderCollection`.
/// - `U`: The output type for the matched data, if schema-matching,
///   implementing `DeserializeOwned`.
pub type TransactionProcessorInputType<T, U = ()> = (
    Arc<TransactionMetadata>,
    Vec<(InstructionMetadata, DecodedInstruction<T>)>,
    Option<U>,
);

/// A pipe for processing transactions based on a defined schema and processor.
///
/// The `TransactionPipe` parses a transaction's instructions, optionally checks
/// them against the schema, and runs the processor if the instructions match
/// the schema. It provides methods for parsing nested instructions and matching
/// transaction data to the schema.
///
/// ## Generics
///
/// - `T`: The instruction type, implementing `InstructionDecoderCollection`.
/// - `U`: The output type for the matched data, if schema-matching,
///   implementing `DeserializeOwned`.
///
/// ## Fields
///
/// - `schema`: The schema against which to match transaction instructions.
/// - `processor`: The processor that will handle matched transaction data.
/// - `filters`: A collection of filters that determine which transaction
///   updates should be processed. Each filter in this collection is applied to
///   incoming transaction updates, and only updates that pass all filters
///   (return `true`) will be processed. If this collection is empty, all
///   updates are processed.
pub struct TransactionPipe<T: InstructionDecoderCollection, U> {
    schema: Option<TransactionSchema<T>>,
    processor: Box<dyn Processor<InputType = TransactionProcessorInputType<T, U>> + Send + Sync>,
    filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

/// Represents a parsed transaction, including its metadata and parsed
/// instructions.
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
    /// - `filters`: A collection of filters for selective processing of
    ///   transaction updates. Filters can be used to selectively process
    ///   transactions based on criteria such as datasource ID, transaction
    ///   type, or other custom logic.
    ///
    /// # Returns
    ///
    /// A `TransactionPipe` instance configured with the specified schema and
    /// processor.
    pub fn new(
        schema: Option<TransactionSchema<T>>,
        processor: impl Processor<InputType = TransactionProcessorInputType<T, U>>
            + Send
            + Sync
            + 'static,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self {
        log::trace!(
            "TransactionPipe::new(schema: {:?}, processor: {:?})",
            schema,
            stringify!(processor)
        );
        Self {
            schema,
            processor: Box::new(processor),
            filters,
        }
    }

    /// Matches parsed instructions against the schema and returns the data as
    /// type `U`.
    ///
    /// The method only returns data if the parsed instructions conform to the
    /// schema.
    ///
    /// # Parameters
    ///
    /// - `instructions`: A slice of `ParsedInstruction` to be matched against
    ///   the schema.
    ///
    /// # Returns
    ///
    /// An `Option<U>` containing the deserialized matched data if the
    /// instructions match the schema.
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

/// Parses nested instructions into a list of `ParsedInstruction`.
///
/// This method recursively traverses the nested instructions and parses
/// each one, creating a structured representation of the instructions.
///
/// # Parameters
///
/// - `nested_ixs`: A slice of `NestedInstruction` representing the instructions
///   to be parsed.
///
/// # Returns
///
/// A `Box<Vec<ParsedInstruction<T>>>` containing the parsed instructions.
pub fn parse_instructions<T: InstructionDecoderCollection>(
    nested_ixs: &[NestedInstruction],
) -> Vec<ParsedInstruction<T>> {
    log::trace!("parse_instructions(nested_ixs: {:?})", nested_ixs);

    let mut parsed_instructions: Vec<ParsedInstruction<T>> = Vec::new();

    for nested_ix in nested_ixs {
        if let Some(instruction) = T::parse_instruction(&nested_ix.instruction) {
            parsed_instructions.push(ParsedInstruction {
                program_id: nested_ix.instruction.program_id,
                instruction,
                inner_instructions: parse_instructions(&nested_ix.inner_instructions),
            });
        } else {
            for inner_ix in nested_ix.inner_instructions.iter() {
                parsed_instructions.extend(parse_instructions(&[inner_ix.clone()]));
            }
        }
    }

    parsed_instructions
}

/// An async trait for processing transactions.
///
/// The `TransactionPipes` trait allows for processing of transactions.
///
/// # Required Methods
///
/// - `run`: Processes a transaction and tracks the operation with metrics.
/// - `filters`: Returns a reference to the filters associated with this pipe,
///   which are used by the pipeline to determine which transaction updates
///   should be processed.
#[async_trait]
pub trait TransactionPipes<'a>: Send + Sync {
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[NestedInstruction],
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
}

#[async_trait]
impl<T, U> TransactionPipes<'_> for TransactionPipe<T, U>
where
    T: InstructionDecoderCollection + Sync + 'static,
    U: DeserializeOwned + Send + Sync + 'static,
{
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[NestedInstruction],
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!(
            "TransactionPipe::run(instructions: {:?}, metrics)",
            instructions,
        );

        let parsed_instructions = parse_instructions(instructions);

        let matched_data = self.matches_schema(&parsed_instructions);

        let unnested_instructions = transformers::unnest_parsed_instructions(
            transaction_metadata.clone(),
            parsed_instructions,
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

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
