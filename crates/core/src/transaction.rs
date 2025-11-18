//! Provides types and traits for handling transaction processing in the
//! `carbon-core` framework.
//!
//! ## Key Components
//!
//! - **TransactionPipe**: Represents a processing pipe for transactions that
//!   processes full transaction context including metadata and all nested
//!   instructions.
//! - **TransactionMetadata**: Metadata associated with a transaction, including
//!   slot, signature, and fee payer information.
//!
//! ## Usage
//!
//! To use this module, create a `TransactionPipe` with a processor. The pipe
//! will process full transactions with their metadata and all nested instructions,
//! providing complete transaction context to the processor.
//!
//! ## Notes
//!
//! - **Nested Instructions**: The `TransactionPipe` supports nested
//!   instructions within transactions, allowing for multi-level transaction
//!   processing.

use crate::filter::Filter;
use solana_program::hash::Hash;

use {
    crate::{
        error::CarbonResult, instruction::NestedInstruction, metrics::MetricsCollection,
        processor::Processor,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
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
    pub message: solana_message::VersionedMessage,
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
pub type TransactionProcessorInputType = (Arc<TransactionMetadata>, Vec<NestedInstruction>);

/// A pipe for processing full transaction context.
///
/// The `TransactionPipe` processes complete transactions with their metadata and
/// all nested instructions, providing full transaction context to the processor.
///
/// ## Fields
///
/// - `processor`: The processor that will handle full transaction data including
///   metadata and all nested instructions.
/// - `filters`: A collection of filters that determine which transaction
///   updates should be processed. Each filter in this collection is applied to
///   incoming transaction updates, and only updates that pass all filters
///   (return `true`) will be processed. If this collection is empty, all
///   updates are processed.
pub struct TransactionPipe {
    processor: Box<dyn Processor<InputType = TransactionProcessorInputType> + Send + Sync>,
    filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

impl TransactionPipe {
    /// Creates a new `TransactionPipe` with the specified processor.
    ///
    /// # Parameters
    ///
    /// - `processor`: The processor that will handle full transaction data
    ///   including metadata and all nested instructions.
    /// - `filters`: A collection of filters for selective processing of
    ///   transaction updates. Filters can be used to selectively process
    ///   transactions based on criteria such as datasource ID, transaction
    ///   type, or other custom logic.
    ///
    /// # Returns
    ///
    /// A `TransactionPipe` instance configured with the specified processor.
    pub fn new(
        processor: impl Processor<InputType = TransactionProcessorInputType> + Send + Sync + 'static,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self {
        log::trace!(
            "TransactionPipe::new(processor: {:?})",
            stringify!(processor)
        );
        Self {
            processor: Box::new(processor),
            filters,
        }
    }
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
impl TransactionPipes<'_> for TransactionPipe {
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

        self.processor
            .process((transaction_metadata, instructions.to_vec()), metrics)
            .await?;

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
