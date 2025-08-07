//! Provides structures and traits for processing and decoding Solana accounts
//! within the pipeline.
//!
//! This module includes the necessary components for handling account data
//! updates in the `carbon-core` pipeline. It provides abstractions for decoding
//! accounts, processing account metadata, and implementing account-specific
//! pipes, which are integral to the pipeline's account update processing.
//!
//! # Overview
//!
//! The `account` module supports various tasks related to Solana account
//! processing:
//! - **Account Metadata**: Metadata about accounts, including slot and public
//!   key information.
//! - **Decoded Account**: Holds detailed account data after decoding, such as
//!   lamports, owner, and rent epoch.
//! - **Account Decoders**: A trait-based mechanism to decode raw Solana account
//!   data into structured formats for processing.
//! - **Account Pipes**: Encapsulates account processing logic, allowing custom
//!   processing of decoded account data in the pipeline.
//!
//! # Example
//!
//! ```ignore
//! struct MyAccountDecoder;
//!
//! impl<'a> AccountDecoder<'a> for MyAccountDecoder {
//!     type AccountType = (AccountMetadata, DecodedAccount<PumpAccount>);
//!
//!     fn decode_account(
//!         &self,
//!         account: &'a solana_account::Account,
//!     ) -> Option<DecodedAccount<Self::AccountType>> {
//!         // Custom decoding logic here
//!         Some(DecodedAccount {
//!             lamports: account.lamports,
//!             data: String::from_utf8(account.data.clone()).ok()?,
//!             owner: account.owner,
//!             executable: account.executable,
//!             rent_epoch: account.rent_epoch,
//!         })
//!     }
//! }
//! ```
//!
//! # Notes
//!
//! - This module requires access to Solana SDK structures, specifically
//!   `Account` and `Pubkey`.
//! - All components support asynchronous processing to enable concurrent data
//!   handling in the pipeline.

use {
    crate::{
        error::CarbonResult, filter::Filter, metrics::MetricsCollection, processor::Processor,
    },
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    std::sync::Arc,
};

/// Holds metadata for an account update, including the slot and public key.
///
/// `AccountMetadata` provides essential information about an account update,
/// such as the slot number where the account was updated and the account's
/// public key. This metadata is used within the pipeline to identify and
/// process account updates.
///
/// # Fields
///
/// - `slot`: The Solana slot number where the account was updated.
/// - `pubkey`: The public key of the account.
#[derive(Debug, Clone)]
pub struct AccountMetadata {
    pub slot: u64,
    pub pubkey: Pubkey,
}

/// Represents the decoded data of a Solana account, including account-specific
/// details.
///
/// `DecodedAccount` holds the detailed data of a Solana account after it has
/// been decoded. It includes the account's lamports, owner, executable status,
/// and rent epoch, as well as any decoded data specific to the account.
///
/// # Type Parameters
///
/// - `T`: The type of data specific to the account, which is determined by the
///   decoder used.
///
/// # Fields
///
/// - `lamports`: The number of lamports in the account.
/// - `data`: The decoded data specific to the account.
/// - `owner`: The public key of the account's owner.
/// - `executable`: Whether the account is executable.
/// - `rent_epoch`: The rent epoch of the account.
#[derive(Debug, Clone)]
pub struct DecodedAccount<T> {
    pub lamports: u64,
    pub data: T,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Defines a trait for decoding Solana accounts into structured data types.
///
/// `AccountDecoder` provides a way to convert raw Solana `Account` data into
/// structured `DecodedAccount` instances. By implementing this trait, you can
/// define custom decoding logic to interpret account data in a way that suits
/// your application's requirements.
///
/// # Associated Types
///
/// - `AccountType`: The data type resulting from decoding the account, specific
///   to the application.
pub trait AccountDecoder<'a> {
    type AccountType;

    fn decode_account(
        &self,
        account: &'a solana_account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>>;
}

/// The input type for the account processor.
///
/// - `T`: The account type, as determined by the decoder.
pub type AccountProcessorInputType<T> =
    (AccountMetadata, DecodedAccount<T>, solana_account::Account);

/// A processing pipe that decodes and processes Solana account updates.
///
/// `AccountPipe` combines an `AccountDecoder` and a `Processor` to manage
/// account updates in the pipeline. This struct decodes the raw account data
/// and then processes the resulting `DecodedAccount` with the specified
/// processing logic.
///
/// # Type Parameters
///
/// - `T`: The data type of the decoded account information, as determined by
///   the decoder.
///
/// # Fields
///
/// - `decoder`: An `AccountDecoder` that decodes raw account data into
///   structured form.
/// - `processor`: A `Processor` that handles the processing logic for decoded
///   accounts.
/// - `filters`: A collection of filters that determine which account updates
///   should be processed. Each filter in this collection is applied to incoming
///   account updates, and only updates that pass all filters (return `true`)
///   will be processed. If this collection is empty, all updates are processed.
pub struct AccountPipe<T: Send> {
    pub decoder: Box<dyn for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static>,
    pub processor: Box<dyn Processor<InputType = AccountProcessorInputType<T>> + Send + Sync>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

/// An async trait for processing account updates.
///
/// The `AccountPipes` trait allows for processing of account updates.
///
/// # Required Methods
///
/// - `run`: Processes an account update and tracks the operation with metrics.
/// - `filters`: Returns a reference to the filters associated with this pipe,
///   which are used by the pipeline to determine which account updates should
///   be processed.
#[async_trait]
pub trait AccountPipes: Send + Sync {
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_account::Account),
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
}

#[async_trait]
impl<T: Send> AccountPipes for AccountPipe<T> {
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_account::Account),
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!(
            "AccountPipe::run(account_with_metadata: {:?}, metrics)",
            account_with_metadata,
        );

        if let Some(decoded_account) = self.decoder.decode_account(&account_with_metadata.1) {
            self.processor
                .process(
                    (
                        account_with_metadata.0.clone(),
                        decoded_account,
                        account_with_metadata.1,
                    ),
                    metrics.clone(),
                )
                .await?;
        }
        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
