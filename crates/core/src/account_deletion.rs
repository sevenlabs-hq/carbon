//! Provides structures and traits for handling account deletion events within
//! the pipeline.
//!
//! This module defines the `AccountDeletionPipe` struct and
//! `AccountDeletionPipes` trait, which allow for the processing of account
//! deletion events as they occur in the pipeline. By implementing
//! `AccountDeletionPipes`, you can create custom behavior for managing
//! the deletion of accounts and associated resources, integrating with metrics
//! for monitoring.

use {
    crate::{
        datasource::AccountDeletion, error::CarbonResult, filter::Filter,
        metrics::MetricsCollection, processor::Processor,
    },
    async_trait::async_trait,
    std::sync::Arc,
};

/// A processing pipe for handling account deletions.
///
/// The `AccountDeletionPipe` struct encapsulates the logic needed to process
/// account deletion events within the pipeline. It uses a `Processor` to define
/// how account deletions are managed, allowing for customizable handling based
/// on the needs of the application.
///
/// ## Functionality
///
/// This struct processes an `AccountDeletion` event by passing it through a
/// user-defined `Processor`. The processor is responsible for managing the
/// specific logic of the account deletion event, such as cleaning up resources
/// or updating other parts of the system. The `AccountDeletionPipe` also
/// integrates with `Metrics`, enabling the tracking and monitoring of deletion
/// events.
///
/// # Example
///
/// ```ignore
/// use carbon_core::error::CarbonResult;
/// use carbon_core::metrics::MetricsCollection;
/// use carbon_core::datasource::AccountDeletion;
/// use carbon_core::processor::Processor;
/// use async_trait::async_trait;
/// use std::sync::Arc;
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
/// ```
///
/// ## Fields
///
/// - `processor`: A boxed `Processor` that handles the specific logic of
///   processing an account deletion event.
/// - `filters`: A collection of filters that determine which account deletion
///   events should be processed. Each filter in this collection is applied to
///   incoming account deletion events, and only events that pass all filters
///   (return `true`) will be processed. If this collection is empty, all
///   events are processed.
///
/// # Notes
///
/// - Ensure that the `Processor` implementation provided is designed to handle
///   account deletions.
/// - This struct is typically used within the broader pipeline structure for
///   managing updates.
pub struct AccountDeletionPipe {
    pub processor: Box<dyn Processor<InputType = AccountDeletion> + Send + Sync>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

/// An async trait for processing account deletions.
///
/// The `AccountDeletionPipes` trait allows for processing of account deletions.
///
/// # Required Methods
///
/// - `run`: Processes an account deletion event and tracks the operation with
///   metrics.
/// - `filters`: Returns a reference to the filters associated with this pipe,
///   which are used by the pipeline to determine which account deletion events
///   should be processed.
#[async_trait]
pub trait AccountDeletionPipes: Send + Sync {
    async fn run(
        &mut self,
        account_deletion: AccountDeletion,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
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

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
