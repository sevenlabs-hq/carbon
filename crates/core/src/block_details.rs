use crate::datasource::BlockDetails;
use crate::error::CarbonResult;
use crate::filter::Filter;
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
/// - `filters`: A collection of filters that determine which block details
///   updates should be processed. Each filter in this collection is applied to
///   incoming block details updates, and only updates that pass all filters
///   (return `true`) will be processed. If this collection is empty, all
///   updates are processed.
pub struct BlockDetailsPipe {
    pub processor: Box<dyn Processor<InputType = BlockDetails> + Send + Sync>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

/// An async trait for processing block details.
///
/// The `BlockDetailsPipes` trait allows for processing of block details.
///
/// # Required Methods
///
/// - `run`: Processes a block details event and tracks the operation with
///   metrics.
/// - `filters`: Returns a reference to the filters associated with this pipe,
///   which are used by the pipeline to determine which block details events
///   should be processed.
#[async_trait]
pub trait BlockDetailsPipes: Send + Sync {
    async fn run(
        &mut self,
        block_details: BlockDetails,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
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

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
