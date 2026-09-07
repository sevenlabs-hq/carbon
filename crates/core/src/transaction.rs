//! Transaction-shaped pipe wiring + per-transaction metadata.
//!
//! # Components
//!
//! - [`TransactionMetadata`] — context shared across every instruction in one
//!   transaction (signature, fee payer, slot, full message and meta).
//! - [`TransactionProcessorInputType<'a, T>`] — borrowed bundle handed to
//!   processors registered via `Pipeline::transaction(...)`.
//! - [`TransactionPipe`] / [`TransactionPipes`] — internal pipe that parses
//!   each instruction through an `InstructionDecoderCollection` and routes the
//!   whole transaction to one processor.
//! - [`parse_instructions_flat`] — helper that maps a flat list of instructions
//!   through a collection.

pub mod rpc;

use {
    crate::{
        collection::InstructionDecoderCollection,
        error::{BoxError, CarbonResult, Error},
        filter::Filter,
        instruction::InstructionMetadata,
        processor::Processor,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
    solana_hash::Hash,
    solana_instruction::Instruction,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::sync::Arc,
};
/// Per-transaction context shared across all of its instructions.
///
/// Built from a `TransactionUpdate` via `TryFrom`. Wrapped in `Arc`
/// inside the pipeline so child instructions cheaply reference it.
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

impl TryFrom<crate::update::TransactionUpdate> for TransactionMetadata {
    type Error = crate::error::Error;

    fn try_from(value: crate::update::TransactionUpdate) -> Result<Self, Self::Error> {
        let accounts = value.transaction().message.static_account_keys();

        Ok(TransactionMetadata {
            slot: value.slot(),
            signature: *value.signature(),
            fee_payer: *accounts
                .first()
                .ok_or(crate::error::Error::MissingFeePayer)?,
            meta: value.meta().clone(),
            message: value.transaction().message.clone(),
            index: value.index(),
            block_time: value.block_time(),
            block_hash: value.block_hash().copied(),
        })
    }
}

/// Borrowed bundle delivered to processors registered via
/// `Pipeline::transaction(...)`: shared transaction metadata plus the
/// flat list of decoded instructions.
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
    T: InstructionDecoderCollection + Send + Sync + 'static,
    P: for<'a> Processor<TransactionProcessorInputType<'a, T>> + Send + Sync + 'static,
{
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[(InstructionMetadata, Instruction)],
    ) -> CarbonResult<()> {
        let parsed_instructions =
            parse_instructions_flat::<T>(instructions).map_err(Error::Decode)?;

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
) -> Result<Vec<(InstructionMetadata, T)>, BoxError> {
    let mut decoded = Vec::new();
    for (metadata, instruction) in instructions {
        if let Some(value) = T::decode_instruction(instruction)? {
            decoded.push((metadata.clone(), value));
        }
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::instruction::InstructionsWithMetadata,
        std::{io, sync::Mutex},
    };

    struct Decoded(u8);

    impl InstructionDecoderCollection for Decoded {
        fn decode_instruction(instruction: &Instruction) -> Result<Option<Self>, BoxError> {
            match instruction.data.first() {
                Some(0) => Ok(None),
                Some(value) => Ok(Some(Self(*value))),
                None => Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "missing instruction data",
                )
                .into()),
            }
        }
    }

    fn instructions(data: &[&[u8]]) -> InstructionsWithMetadata {
        data.iter()
            .enumerate()
            .map(|(index, data)| {
                (
                    InstructionMetadata {
                        transaction_metadata: Arc::new(TransactionMetadata::default()),
                        stack_height: 1,
                        index: index as u32,
                        absolute_path: vec![index as u8],
                    },
                    Instruction {
                        program_id: Pubkey::new_unique(),
                        accounts: vec![],
                        data: data.to_vec(),
                    },
                )
            })
            .collect()
    }

    #[test]
    fn collection_skips_non_matches_and_preserves_order() {
        let input = instructions(&[&[7], &[0], &[9]]);
        let decoded = parse_instructions_flat::<Decoded>(&input).unwrap();
        let values: Vec<_> = decoded
            .iter()
            .map(|(meta, value)| (meta.index, value.0))
            .collect();
        assert_eq!(values, vec![(0, 7), (2, 9)]);
    }

    struct Collector(Arc<Mutex<Vec<Vec<u8>>>>);

    impl Processor<TransactionProcessorInputType<'_, Decoded>> for Collector {
        async fn process(
            &mut self,
            input: &TransactionProcessorInputType<'_, Decoded>,
        ) -> CarbonResult<()> {
            self.0.lock().unwrap().push(
                input
                    .instructions
                    .iter()
                    .map(|(_, value)| value.0)
                    .collect(),
            );
            Ok(())
        }
    }

    #[tokio::test]
    async fn transaction_pipe_never_delivers_partial_collections() {
        let received = Arc::new(Mutex::new(Vec::new()));
        let mut pipe = TransactionPipe::<Decoded, _>::new(Collector(received.clone()), vec![]);
        let metadata = Arc::new(TransactionMetadata::default());
        for input in [
            instructions(&[]),
            instructions(&[&[0]]),
            instructions(&[&[7], &[0], &[9]]),
        ] {
            pipe.run(metadata.clone(), &input).await.unwrap();
        }
        let error = pipe
            .run(metadata, &instructions(&[&[7], &[], &[9]]))
            .await
            .unwrap_err();
        let Error::Decode(error) = error else {
            panic!("expected decoder error");
        };
        assert_eq!(
            error.downcast_ref::<io::Error>().unwrap().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert_eq!(*received.lock().unwrap(), vec![vec![], vec![], vec![7, 9]]);
    }
}
