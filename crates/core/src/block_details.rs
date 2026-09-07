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
    crate::{
        error::{CarbonResult, Error},
        filter::Filters,
        processor::Processor,
        route::RouteContext,
        update::BlockUpdate,
    },
    async_trait::async_trait,
};

pub struct BlockDetailsPipe<P> {
    processor: P,
    filters: Filters<BlockUpdate>,
}

impl<P> BlockDetailsPipe<P> {
    pub fn new(processor: P, filters: Filters<BlockUpdate>) -> Self {
        Self { processor, filters }
    }
}

#[async_trait]
pub trait BlockDetailsPipes: Send {
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        block_details: &BlockUpdate,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<P> BlockDetailsPipes for BlockDetailsPipe<P>
where
    P: Processor<BlockUpdate>,
{
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        block_details: &BlockUpdate,
    ) -> CarbonResult<()> {
        if !self
            .filters
            .filter(context, block_details)
            .await
            .map_err(Error::Filter)?
        {
            return Ok(());
        }

        let result = self.processor.process(context, block_details).await;

        if result.is_ok() {
            self.filters
                .commit(context, block_details, &result)
                .await
                .map_err(Error::FilterCommit)?;
        }

        result.map_err(Error::Processor)?;

        Ok(())
    }
}
