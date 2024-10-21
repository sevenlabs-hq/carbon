//! Provides traits and structures for managing and consuming data updates from various sources.
//!
//! The `datasource` module defines the `Datasource` trait and associated data types for
//! handling updates related to accounts, transactions, and account deletions. This module
//! allows for flexible data ingestion from various Solana data sources, enabling integration
//! with the `carbon-core` processing pipeline.
//!
//! # Overview
//!
//! The core component of this module is the `Datasource` trait, which represents an interface
//! for consuming data updates asynchronously. Implementations of `Datasource` provide the
//! logic for fetching data updates and delivering them via a channel to the pipeline.
//! Different types of updates are represented by the `Update` enum, including:
//! - `AccountUpdate`: Represents updates to accounts, including the account's public key, slot,
//!   and other account data.
//! - `TransactionUpdate`: Represents transaction updates, including transaction details,
//!   signature, and status metadata.
//! - `AccountDeletion`: Represents account deletion events, indicating when an account is removed
//!   from the blockchain state.
//!
//! The module also includes the `UpdateType` enum to categorize the kinds of updates that
//! a data source can provide.
//!
//! # Notes
//!
//! - The `Datasource` trait is asynchronous and should be used within a Tokio runtime.
//! - Use the `Update` enum to encapsulate data updates of different types. This helps
//!   centralize handling of all update kinds in the pipeline.
//! - Ensure implementations handle errors gracefully, especially when fetching data
//!   and sending updates to the pipeline.

use crate::{error::CarbonResult, metrics::MetricsCollection};
use async_trait::async_trait;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Defines the interface for data sources that produce updates for accounts, transactions,
/// and account deletions.
///
/// The `Datasource` trait represents a data source that can be consumed asynchronously
/// within a pipeline. Implementations of this trait are responsible for fetching updates
/// and sending them through a channel to be processed further. Each datasource specifies
/// the types of updates it supports by implementing the `update_types` method.
///
/// # Required Methods
///
/// - `consume`: Initiates the asynchronous consumption of updates. This method should send
///   updates through the provided `sender` channel.
/// - `update_types`: Returns a list of `UpdateType` variants indicating the types of updates
///   the datasource can provide.
///
/// # Example
///
/// ```rust
/// #[async_trait]
/// impl Datasource for MyDatasource {
///     async fn consume(
///         &self,
///         sender: &tokio::sync::mpsc::UnboundedSender<Update>,
///         cancellation_token: CancellationToken,
///         metrics: Arc<MetricsCollection>,
///     ) -> CarbonResult<tokio::task::AbortHandle> {
///         // Implement update fetching logic
///     }
///
///     fn update_types(&self) -> Vec<UpdateType> {
///         vec![UpdateType::AccountUpdate, UpdateType::Transaction]
///     }
/// }
/// ```
///
/// # Notes
///
/// - This trait is marked with `async_trait`, so implementations must be asynchronous.
/// - The `consume` method should handle errors and retries to ensure robust update delivery.
///
#[async_trait]
pub trait Datasource: Send + Sync {
    async fn consume(
        &self,
        sender: &tokio::sync::mpsc::UnboundedSender<Update>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;

    fn update_types(&self) -> Vec<UpdateType>;
}

/// Represents a data update in the `carbon-core` pipeline, encompassing different update types.
///
/// - `Account`: Represents an update to an account's data.
/// - `Transaction`: Represents a transaction-related update, including transaction metadata.
/// - `AccountDeletion`: Represents an event where an account has been deleted.
///
#[derive(Debug, Clone)]
pub enum Update {
    Account(AccountUpdate),
    Transaction(TransactionUpdate),
    AccountDeletion(AccountDeletion),
}

/// Enumerates the types of updates a datasource can provide.
///
/// The `UpdateType` enum categorizes updates into three types:
/// - `AccountUpdate`: Indicates that the datasource provides account updates.
/// - `Transaction`: Indicates that the datasource provides transaction updates.
/// - `AccountDeletion`: Indicates that the datasource provides account deletion events.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateType {
    AccountUpdate,
    Transaction,
    AccountDeletion,
}

/// Represents an update to a Solana account, including its public key, data, and slot information.
///
/// The `AccountUpdate` struct encapsulates the essential information for an account update,
/// containing the account's `pubkey`, `account` data, and the `slot` at which the update occurred.
///
/// - `pubkey`: The public key of the account being updated.
/// - `account`: The new state of the account.
/// - `slot`: The slot number in which this account update was recorded.
///
#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub pubkey: Pubkey,
    pub account: solana_sdk::account::Account,
    pub slot: u64,
}

/// Represents the deletion of a Solana account, containing the account's public key and slot information.
///
/// The `AccountDeletion` struct indicates that an account has been removed from the blockchain state,
/// providing the `pubkey` of the deleted account and the `slot` in which the deletion occurred.
///
/// - `pubkey`: The public key of the deleted account.
/// - `slot`: The slot number in which the account was deleted.
///
#[derive(Debug, Clone)]
pub struct AccountDeletion {
    pub pubkey: Pubkey,
    pub slot: u64,
}

/// Represents a transaction update in the Solana network, including transaction metadata, status, and slot information.
///
/// The `TransactionUpdate` struct provides detailed information about a transaction, including its
/// `signature`, `transaction` data, `meta` status, and the `slot` where it was recorded. Additionally,
/// it includes a `is_vote` flag to indicate whether the transaction is a voting transaction.
///
/// - `signature`: The unique signature of the transaction.
/// - `transaction`: The complete `VersionedTransaction` data of the transaction.
/// - `meta`: Metadata about the transaction's status, such as fee information and logs.
/// - `is_vote`: A boolean indicating whether the transaction is a vote.
/// - `slot`: The slot number in which the transaction was recorded.
///
#[derive(Debug, Clone)]
pub struct TransactionUpdate {
    pub signature: Signature,
    pub transaction: solana_sdk::transaction::VersionedTransaction,
    pub meta: solana_transaction_status::TransactionStatusMeta,
    pub is_vote: bool,
    pub slot: u64,
}
