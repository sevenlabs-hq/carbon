use crate::datasource::SlotUpdate;
use crate::error::CarbonResult;
use crate::processor::Processor;
use async_trait::async_trait;

pub struct SlotPipe {
    pub processor: Box<dyn Processor<InputType = SlotUpdate> + Send + Sync>,
}

#[async_trait]
pub trait SlotPipes: Send + Sync {
    async fn run(&self, slot_update: SlotUpdate) -> CarbonResult<()>;
}

#[async_trait]
impl SlotPipes for SlotPipe {
    async fn run(&self, slot_update: SlotUpdate) -> CarbonResult<()> {
        self.processor.process(slot_update).await?;
        Ok(())
    }
}
