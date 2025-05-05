use crate::datasource::BlockDetails;
use crate::error::CarbonResult;
use crate::metrics::MetricsCollection;
use crate::processor::Processor;
use async_trait::async_trait;
use std::sync::Arc;

/// A pipe for processing block details using a defined processor.
///
/// The `BlockDetailsPipe` processes updates related to block metadata, such as
/// slot, block hash, and rewards. It uses a `Processor` to handle the block
/// details and perform the necessary operations.
///
/// ## Fields
///
/// - `processor`: A `Processor` that processes block details updates.
pub struct BlockDetailsPipe {
    pub processor: Box<dyn Processor<InputType = BlockDetails> + Send + Sync>,
}

/// A trait for handling block details updates in the pipeline.
///
/// The `BlockDetailsPipes` trait defines an asynchronous `run` method, which
/// is responsible for processing a `BlockDetails` event. Implementing this
/// trait allows you to create custom block details handling within the
/// pipeline, which can be extended to include metrics tracking or other
/// custom behaviors.
///
/// ## Functionality
///
/// The `run` method takes a `BlockDetails` event and a list of `Metrics` to
/// track the processing operation. The method is asynchronous, allowing for
/// non-blocking operations, and is expected to return a `CarbonResult<()>`,
/// which indicates success or failure.
///
/// # Syntax
///
/// Implementations of `BlockDetailsPipes` should focus on handling block
/// details updates and integrating with metrics as needed.
///
/// # Example
///
/// ```ignore
/// use carbon_core::metrics::MetricsCollection;
/// use std::sync::Arc;
/// use carbon_core::error::CarbonResult;
/// use carbon_core::datasource::BlockDetails;
/// use carbon_core::block_details::BlockDetailsPipe;
/// use async_trait::async_trait;
///
/// struct MyBlockDetailsPipe;
///
/// #[async_trait]
/// impl BlockDetailsPipes for BlockDetailsPipe {
///     async fn run(
///         &mut self,
///         block_details: BlockDetails,
///         metrics: Arc<MetricsCollection>,
///     ) -> CarbonResult<()> {
///         // Custom processing logic for the block details event
///         Ok(())
///     }
/// }
/// ```
///
/// # Parameters
///
/// - `block_details`: A `BlockDetails` instance representing the block details
///   update event.
/// - `metrics`: A vector of `Metrics` objects for monitoring and reporting
///   purposes.
///
/// # Returns
///
/// Returns a `CarbonResult<()>`, which will be `Ok(())` if processing is
/// successful, or an error if there was an issue with the processing logic.
///
/// # Notes
///
/// - This trait is asynchronous and requires the `async_trait` crate for
///   `async` methods.
/// - Ensure that `BlockDetailsPipe` is configured with a processor capable
///   of handling block details updates, as this is its primary responsibility
///   within the pipeline.
#[async_trait]
pub trait BlockDetailsPipes: Send + Sync {
    /// Processes a block details event and tracks the operation with
    /// metrics.
    ///
    /// This asynchronous method allows for non-blocking processing of block
    /// details events, enabling integration with custom logic and metrics
    /// tracking.
    ///
    /// # Parameters
    ///
    /// - `block_details`: The block details event to process.
    /// - `metrics`: A list of `Metrics` implementations for tracking and
    ///   reporting metrics.
    ///
    /// # Returns
    ///
    /// Returns a `CarbonResult<()>`, which is `Ok` on success, or an error if
    /// processing fails.
    async fn run(
        &mut self,
        block_details: BlockDetails,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl BlockDetailsPipes for BlockDetailsPipe {
    async fn run(
        &mut self,
        block_details: BlockDetails,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!(
            "Block details::run(block_details: {:?}, metrics)",
            block_details,
        );

        self.processor.process(block_details, metrics).await?;

        Ok(())
    }
}
