use crate::datasource::BlockDetails;
use crate::error::CarbonResult;
use crate::filter::Filter;
use crate::processor::Processor;
use async_trait::async_trait;

pub struct BlockDetailsPipe {
    pub processor: Box<dyn Processor<InputType = BlockDetails> + Send + Sync>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

#[async_trait]
pub trait BlockDetailsPipes: Send + Sync {
    async fn run(&mut self, block_details: BlockDetails) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
}

#[async_trait]
impl BlockDetailsPipes for BlockDetailsPipe {
    async fn run(&mut self, block_details: BlockDetails) -> CarbonResult<()> {
        log::trace!("Block details::run(block_details: {block_details:?})");

        self.processor.process(block_details).await?;

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
