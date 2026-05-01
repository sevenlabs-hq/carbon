use {
    crate::{
        collection::InstructionDecoderCollection, error::CarbonResult, filter::Filter,
        instruction::InstructionMetadata, processor::Processor,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
    solana_hash::Hash,
    solana_instruction::Instruction,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::sync::Arc,
};
#[derive(Debug, Clone, Default)]
pub struct TransactionMetadata {
    pub slot: u64,
    pub signature: Signature,
    pub fee_payer: Pubkey,
    pub meta: solana_transaction_status::TransactionStatusMeta,
    pub message: solana_message::VersionedMessage,
    pub index: Option<u64>,
    pub block_time: Option<i64>,
    pub block_hash: Option<Hash>,
}

impl TryFrom<crate::datasource::TransactionUpdate> for TransactionMetadata {
    type Error = crate::error::Error;

    fn try_from(value: crate::datasource::TransactionUpdate) -> Result<Self, Self::Error> {
        let accounts = value.transaction.message.static_account_keys();

        Ok(TransactionMetadata {
            slot: value.slot,
            signature: value.signature,
            fee_payer: *accounts
                .first()
                .ok_or(crate::error::Error::MissingFeePayer)?,
            meta: value.meta.clone(),
            message: value.transaction.message.clone(),
            index: value.index,
            block_time: value.block_time,
            block_hash: value.block_hash,
        })
    }
}

#[derive(Debug)]
pub struct TransactionProcessorInputType<'a, T> {
    pub metadata: &'a Arc<TransactionMetadata>,
    pub instructions: &'a [(InstructionMetadata, T)],
}

pub struct TransactionPipe<T: InstructionDecoderCollection, P> {
    processor: P,
    filters: Vec<Box<dyn Filter + 'static>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: InstructionDecoderCollection, P> TransactionPipe<T, P> {
    pub fn new(processor: P, filters: Vec<Box<dyn Filter + 'static>>) -> Self {
        Self {
            processor,
            filters,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
pub trait TransactionPipes<'a>: Send + Sync {
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[(InstructionMetadata, Instruction)],
    ) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<T, P> TransactionPipes<'_> for TransactionPipe<T, P>
where
    T: InstructionDecoderCollection + Sync + 'static,
    P: for<'a> Processor<TransactionProcessorInputType<'a, T>> + Send + Sync + 'static,
{
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[(InstructionMetadata, Instruction)],
    ) -> CarbonResult<()> {
        let parsed_instructions = parse_instructions_flat::<T>(instructions);

        let data = TransactionProcessorInputType {
            metadata: &transaction_metadata,
            instructions: &parsed_instructions,
        };

        self.processor.process(&data).await?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}

pub fn parse_instructions_flat<T: InstructionDecoderCollection>(
    instructions: &[(InstructionMetadata, Instruction)],
) -> Vec<(InstructionMetadata, T)> {
    instructions
        .iter()
        .filter_map(|(metadata, instruction)| {
            T::parse_instruction(instruction).map(|parsed| (metadata.clone(), parsed))
        })
        .collect()
}
