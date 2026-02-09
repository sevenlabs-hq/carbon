use crate::filter::Filter;
use solana_program::hash::Hash;

use {
    crate::{
        collection::InstructionDecoderCollection,
        error::CarbonResult,
        instruction::{InstructionMetadata, NestedInstruction, ParsedInstruction},
        processor::Processor,
        transformers,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
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
        log::trace!("try_from(transaction_update: {value:?})");
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
    pub instructions: &'a Vec<(InstructionMetadata, T)>,
}

pub struct TransactionPipe<T: InstructionDecoderCollection, P> {
    processor: P,
    filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    _phantom: std::marker::PhantomData<T>,
}

pub struct ParsedTransaction<I: InstructionDecoderCollection> {
    pub metadata: TransactionMetadata,
    pub instructions: Vec<ParsedInstruction<I>>,
}

impl<T: InstructionDecoderCollection, P> TransactionPipe<T, P> {
    pub fn new(processor: P, filters: Vec<Box<dyn Filter + Send + Sync + 'static>>) -> Self {
        log::trace!(
            "TransactionPipe::new(processor: {:?})",
            stringify!(processor)
        );
        Self {
            processor,
            filters,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub fn parse_instructions<T: InstructionDecoderCollection>(
    nested_ixs: &[NestedInstruction],
) -> Vec<ParsedInstruction<T>> {
    log::trace!("parse_instructions(nested_ixs: {nested_ixs:?})");

    let mut parsed_instructions: Vec<ParsedInstruction<T>> = Vec::new();

    for nested_ix in nested_ixs {
        if let Some(instruction) = T::parse_instruction(&nested_ix.instruction) {
            parsed_instructions.push(ParsedInstruction {
                metadata: nested_ix.metadata.clone(),
                instruction,
                inner_instructions: parse_instructions(&nested_ix.inner_instructions),
            });
        } else {
            for inner_ix in nested_ix.inner_instructions.iter() {
                parsed_instructions.extend(parse_instructions(std::slice::from_ref(inner_ix)));
            }
        }
    }

    parsed_instructions
}

#[async_trait]
pub trait TransactionPipes<'a>: Send + Sync {
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[NestedInstruction],
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
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
        instructions: &[NestedInstruction],
    ) -> CarbonResult<()> {
        log::trace!("TransactionPipe::run(instructions: {instructions:?})");

        let parsed_instructions = parse_instructions(instructions);

        let unnested_instructions = transformers::unnest_parsed_instructions(parsed_instructions);

        let data = TransactionProcessorInputType {
            metadata: &transaction_metadata,
            instructions: &unnested_instructions,
        };

        self.processor.process(&data).await?;

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
