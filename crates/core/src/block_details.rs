//! Block-details pipe wiring.
//!
//! # Components
//!
//! - [`BlockDetailsPipe`] — internal pipe wrapping the user processor and
//!   filters for `Update::Block`. Constructed by
//!   `PipelineBuilder::block_details(...)` and
//!   `block_details_with_filters(...)`.
//! - [`BlockDetailsPipes`] — dyn-dispatch trait the pipeline holds as `Box<dyn
//!   BlockDetailsPipes>`.

use {
    crate::{error::CarbonResult, filter::Filter, processor::Processor, update::BlockUpdate},
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
    async fn run(&mut self, block_details: BlockUpdate) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<P> BlockDetailsPipes for BlockDetailsPipe<P>
where
    P: Processor<BlockUpdate> + Send + Sync,
{
    async fn run(&mut self, block_details: BlockUpdate) -> CarbonResult<()> {
        self.processor.process(&block_details).await?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}
