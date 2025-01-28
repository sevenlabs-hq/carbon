//! Provides structures and traits for handling account deletion events within the pipeline.
//!
//! This module defines the `AccountDeletionPipe` struct and `AccountDeletionPipes` trait,
//! which allow for the processing of account deletion events as they occur in the pipeline.
//! By implementing `AccountDeletionPipes`, you can create custom behavior for managing
//! the deletion of accounts and associated resources, integrating with metrics for monitoring.

use crate::{
    datasource::AccountDeletion, error::CarbonResult, metrics::MetricsCollection,
    processor::Processor,
};
use async_trait::async_trait;
use std::sync::Arc;

/// A processing pipe for handling account deletions.
///
/// The `AccountDeletionPipe` struct encapsulates the logic needed to process account deletion events
/// within the pipeline. It uses a `Processor` to define how account deletions are managed, allowing for
/// customizable handling based on the needs of the application.
///
/// ## Functionality
///
/// This struct processes an `AccountDeletion` event by passing it through a user-defined `Processor`.
/// The processor is responsible for managing the specific logic of the account deletion event, such as
/// cleaning up resources or updating other parts of the system. The `AccountDeletionPipe` also integrates
/// with `Metrics`, enabling the tracking and monitoring of deletion events.
///
/// # Example
///
/// ```rust
///
/// struct MyAccountDeletionProcessor;
///
/// #[async_trait]
/// impl Processor for MyAccountDeletionProcessor {
///     async fn process(
///         &self,
///         account_deletion: AccountDeletion,
///         metrics: Arc<MetricsCollection>,
///     ) -> CarbonResult<()> {
///         // Custom deletion logic
///         Ok(())
///     }
/// }
///
/// ```
///
/// ## Fields
///
/// - `processor`: A boxed `Processor` that handles the specific logic of processing
///   an account deletion event.
///
/// # Notes
///
/// - Ensure that the `Processor` implementation provided is designed to handle account deletions.
/// - This struct is typically used within the broader pipeline structure for managing updates.
pub struct AccountDeletionPipe {
    pub processor: Box<dyn Processor<InputType = AccountDeletion> + Send + Sync>,
}

/// A trait for handling account deletion events in the pipeline.
///
/// The `AccountDeletionPipes` trait defines an asynchronous `run` method, which is responsible for
/// processing an `AccountDeletion` event. Implementing this trait allows you to create custom account
/// deletion handling within the pipeline, which can be extended to include metrics tracking or other
/// custom behaviors.
///
/// ## Functionality
///
/// The `run` method takes an `AccountDeletion` event and a list of `Metrics` to track the processing
/// operation. The method is asynchronous, allowing for non-blocking operations, and is expected to
/// return a `CarbonResult<()>`, which indicates success or failure.
///
/// # Syntax
///
/// Implementations of `AccountDeletionPipes` should focus on handling account deletions and integrating
/// with metrics as needed.
///
/// # Example
///
/// ```rust
///
/// struct MyAccountDeletionPipe;
///
/// #[async_trait]
/// impl AccountDeletionPipes for AccountDeletionPipe {
///     async fn run(
///         &mut self,
///         account_deletion: AccountDeletion,
///         metrics: Arc<MetricsCollection>,
///     ) -> CarbonResult<()> {
///         // Custom processing logic for the deletion event
///         Ok(())
///     }
/// }
/// ```
///
/// # Parameters
///
/// - `account_deletion`: An `AccountDeletion` instance representing the account deletion event.
/// - `metrics`: A vector of `Metrics` objects for monitoring and reporting purposes.
///
/// # Returns
///
/// Returns a `CarbonResult<()>`, which will be `Ok(())` if processing is successful, or an error
/// if there was an issue with the processing logic.
///
/// # Notes
///
/// - This trait is asynchronous and requires the `async_trait` crate for `async` methods.
/// - Ensure that `AccountDeletionPipe` is configured with a processor capable of handling account
///   deletions, as this is its primary responsibility within the pipeline.
#[async_trait]
pub trait AccountDeletionPipes: Send + Sync {
    /// Processes an account deletion event and tracks the operation with metrics.
    ///
    /// This asynchronous method allows for non-blocking processing of account deletion events,
    /// enabling integration with custom deletion logic and metrics tracking.
    ///
    /// # Parameters
    ///
    /// - `account_deletion`: The account deletion event to process.
    /// - `metrics`: A list of `Metrics` implementations for tracking and reporting metrics.
    ///
    /// # Returns
    ///
    /// Returns a `CarbonResult<()>`, which is `Ok` on success, or an error if processing fails.
    async fn run(
        &mut self,
        account_deletion: AccountDeletion,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl AccountDeletionPipes for AccountDeletionPipe {
    async fn run(
        &mut self,
        account_deletion: AccountDeletion,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!(
            "AccountDeletionPipe::run(account_deletion: {:?}, metrics)",
            account_deletion,
        );

        self.processor.process(account_deletion, metrics).await?;

        Ok(())
    }
}
