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
    async fn run(&mut self, block_details: BlockDetails) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<P> BlockDetailsPipes for BlockDetailsPipe<P>
where
    P: Processor<BlockDetails> + Send + Sync,
{
    async fn run(&mut self, block_details: BlockDetails) -> CarbonResult<()> {
        self.processor.process(&block_details).await?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}
