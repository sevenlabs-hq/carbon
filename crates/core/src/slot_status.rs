//! Slot-status pipe wiring.
//!
//! # Components
//!
//! - [`SlotStatusPipe`] — internal pipe wrapping the user processor and
//!   filters for `Update::SlotStatus`. Constructed by
//!   `PipelineBuilder::slot_status(...)` and
//!   `slot_status_with_filters(...)`.
//! - [`SlotStatusPipes`] — dyn-dispatch trait the pipeline holds as `Box<dyn
//!   SlotStatusPipes>`.

use {
    crate::{
        datasource::SlotStatusUpdate, error::CarbonResult, filter::Filter, processor::Processor,
    },
    async_trait::async_trait,
};

pub struct SlotStatusPipe<P> {
    processor: P,
    filters: Vec<Box<dyn Filter + 'static>>,
}

impl<P> SlotStatusPipe<P> {
    pub fn new(processor: P, filters: Vec<Box<dyn Filter + 'static>>) -> Self {
        Self { processor, filters }
    }
}

#[async_trait]
pub trait SlotStatusPipes: Send + Sync {
    async fn run(&mut self, slot_status: SlotStatusUpdate) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<P> SlotStatusPipes for SlotStatusPipe<P>
where
    P: Processor<SlotStatusUpdate> + Send + Sync,
{
    async fn run(&mut self, slot_status: SlotStatusUpdate) -> CarbonResult<()> {
        self.processor.process(&slot_status).await?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}
