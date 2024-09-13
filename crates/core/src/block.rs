use crate::datasource::BlockUpdate;
use crate::error::CarbonResult;
use crate::processor::Processor;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct DecodedBlock {
    pub slot: u64,
    pub block_hash: String,
    pub block_timestamp: i64,
    pub previous_block_hash: String,
    pub transaction_count: u64,
    pub transactions: Option<Vec<solana_sdk::transaction::VersionedTransaction>>,
}

pub trait BlockDecoder {
    type BlockType;

    fn decode(&self, block: &BlockUpdate) -> Option<DecodedBlock>;
}

pub struct BlockPipe<T: Send> {
    pub processor: Box<dyn Processor<InputType = DecodedBlock> + Send + Sync>,
    pub decoder: Box<dyn BlockDecoder<BlockType = T> + Send + Sync>,
}

#[async_trait]
pub trait BlockPipes {
    async fn run(&self, block: BlockUpdate) -> CarbonResult<()>;
}

#[async_trait]
impl<T: Send> BlockPipes for BlockPipe<T> {
    async fn run(&self, block_update: BlockUpdate) -> CarbonResult<()> {
        if let Some(block) = self.decoder.decode(&block_update) {
            self.processor.process(block).await?;
        }
        Ok(())
    }
}
