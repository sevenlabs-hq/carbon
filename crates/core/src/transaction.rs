use crate::filter::Filter;
use solana_program::hash::Hash;

use {
    crate::{
        collection::InstructionDecoderCollection,
        error::CarbonResult,
        instruction::{InstructionMetadata, NestedInstruction},
        processor::Processor,
        schema::{ParsedInstruction, TransactionSchema},
        transformers,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
    serde::de::DeserializeOwned,
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
pub struct TransactionProcessorInputType<'a, T, U = ()> {
    pub metadata: &'a Arc<TransactionMetadata>,
    pub instructions: &'a Vec<(InstructionMetadata, T)>,
    pub matched_data: &'a Option<U>,
}

pub struct TransactionPipe<T: InstructionDecoderCollection, U, P> {
    schema: Option<TransactionSchema<T>>,
    processor: P,
    filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    _phantom: std::marker::PhantomData<U>,
}

pub struct ParsedTransaction<I: InstructionDecoderCollection> {
    pub metadata: TransactionMetadata,
    pub instructions: Vec<ParsedInstruction<I>>,
}

impl<T: InstructionDecoderCollection, U, P> TransactionPipe<T, U, P> {
    pub fn new(
        schema: Option<TransactionSchema<T>>,
        processor: P,
        filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    ) -> Self {
        log::trace!(
            "TransactionPipe::new(schema: {:?}, processor: {:?})",
            schema,
            stringify!(processor)
        );
        Self {
            schema,
            processor,
            filters,
            _phantom: std::marker::PhantomData,
        }
    }

    fn matches_schema(&self, instructions: &[ParsedInstruction<T>]) -> Option<U>
    where
        U: DeserializeOwned,
    {
        match self.schema {
            Some(ref schema) => schema.match_schema(instructions),
            None => None,
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
impl<T, U, P> TransactionPipes<'_> for TransactionPipe<T, U, P>
where
    T: InstructionDecoderCollection + Sync + 'static,
    U: DeserializeOwned + Send + Sync + 'static,
    P: for<'a> Processor<TransactionProcessorInputType<'a, T, U>> + Send + Sync + 'static,
{
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[NestedInstruction],
    ) -> CarbonResult<()> {
        log::trace!("TransactionPipe::run(instructions: {instructions:?})");

        let parsed_instructions = parse_instructions(instructions);

        let matched_data = self.matches_schema(&parsed_instructions);

        let unnested_instructions = transformers::unnest_parsed_instructions(
            transaction_metadata.clone(),
            parsed_instructions,
            0,
        );

        let data = TransactionProcessorInputType {
            metadata: &transaction_metadata,
            instructions: &unnested_instructions,
            matched_data: &matched_data,
        };

        self.processor.process(&data).await?;

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
