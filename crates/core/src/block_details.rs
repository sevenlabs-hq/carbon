//! Block-details pipe wiring.
//!
//! # Components
//!
//! - [`BlockDetailsPipe`] — internal pipe wrapping the user processor and
//!   filters for `Update::BlockDetails`. Constructed by
//!   `PipelineBuilder::block_details(...)` and
//!   `block_details_with_filters(...)`.
//! - [`BlockDetailsPipes`] — dyn-dispatch trait the pipeline holds as `Box<dyn
//!   BlockDetailsPipes>`.
#[cfg(feature = "batch")]
use crate::datasource::BatchUpdateId;
use {
    crate::{datasource::BlockDetails, error::CarbonResult, filter::Filter, processor::Processor},
    async_trait::async_trait,
};

pub struct BlockDetailsPipe<P> {
    processor: P,
    filters: Vec<Box<dyn Filter + 'static>>,
}

impl<P> BlockDetailsPipe<P> {
    pub fn new(processor: P, filters: Vec<Box<dyn Filter + 'static>>) -> Self {
        Self { processor, filters }
    }
}

#[async_trait]
pub trait BlockDetailsPipes: Send + Sync {
    async fn run(
        &mut self,
        #[cfg(feature = "batch")] update_id: &BatchUpdateId,
        block_details: BlockDetails,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<P> BlockDetailsPipes for BlockDetailsPipe<P>
where
    P: Processor<BlockDetails> + Send + Sync,
{
    async fn run(
        &mut self,
        #[cfg(feature = "batch")] update_id: &BatchUpdateId,
        block_details: BlockDetails,
    ) -> CarbonResult<()> {
        self.processor
            .process(
                #[cfg(feature = "batch")]
                update_id,
                &block_details,
            )
            .await?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}
