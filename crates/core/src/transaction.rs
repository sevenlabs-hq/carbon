//! Transaction-shaped pipe wiring + per-transaction metadata.
//!
//! # Components
//!
//! - [`TransactionMetadata`] — context shared across every instruction in one
//!   transaction (signature, fee payer, slot, full message and meta).
//! - [`TransactionProcessorInput<'a, T>`] — borrowed bundle handed to
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
        filter::Filters,
        instruction::NestedInstruction,
        processor::Processor,
        route::{InstructionProcessorInput, RouteContext, TransactionProcessorInput},
        update::TransactionUpdate,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
    solana_hash::Hash,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
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

pub struct TransactionPipe<T: InstructionDecoderCollection, P> {
    processor: P,
    filters: Filters<TransactionUpdate>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: InstructionDecoderCollection, P> TransactionPipe<T, P> {
    pub fn new(processor: P, filters: Filters<TransactionUpdate>) -> Self {
        Self {
            processor,
            filters,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
pub trait TransactionPipes: Send {
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        update: &TransactionUpdate,
        instructions: &[&NestedInstruction],
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<T, P> TransactionPipes for TransactionPipe<T, P>
where
    T: InstructionDecoderCollection + Send + Sync + 'static,
    P: for<'a> Processor<TransactionProcessorInput<'a, T>> + 'static,
{
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        update: &TransactionUpdate,
        instructions: &[&NestedInstruction],
    ) -> CarbonResult<()> {
        if !self
            .filters
            .filter(context, update)
            .await
            .map_err(Error::Filter)?
        {
            return Ok(());
        }

        let parsed_instructions =
            parse_instructions_flat::<T>(instructions).map_err(Error::Decode)?;

        let data = TransactionProcessorInput {
            update,
            instructions: &parsed_instructions,
        };

        let result = self.processor.process(context, &data).await;

        if result.is_ok() {
            self.filters
                .commit(context, update, &result)
                .await
                .map_err(Error::FilterCommit)?;
        }

        result.map_err(Error::Processor)?;

        Ok(())
    }
}

pub fn parse_instructions_flat<'a, T: InstructionDecoderCollection>(
    instructions: &[&'a NestedInstruction],
) -> Result<Vec<InstructionProcessorInput<'a, T>>, BoxError> {
    let mut decoded = Vec::new();
    for &instruction in instructions {
        if let Some(value) = T::decode_instruction(&instruction.instruction)? {
            decoded.push(InstructionProcessorInput {
                instruction,
                decoded: value,
            });
        }
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            id::Id,
            instruction::{InstructionMetadata, NestedInstructions},
            processor::ProcessorResult,
        },
        solana_instruction::Instruction,
        solana_transaction::versioned::VersionedTransaction,
        solana_transaction_status::TransactionStatusMeta,
        std::{
            io,
            sync::{Arc, Mutex},
        },
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

    fn instructions(data: &[&[u8]]) -> Vec<NestedInstruction> {
        data.iter()
            .enumerate()
            .map(|(index, data)| NestedInstruction {
                metadata: InstructionMetadata {
                    transaction_metadata: Arc::new(TransactionMetadata::default()),
                    stack_height: 1,
                    index: index as u32,
                    absolute_path: vec![index as u8],
                },
                instruction: Instruction {
                    program_id: Pubkey::new_unique(),
                    accounts: vec![],
                    data: data.to_vec(),
                },
                inner_instructions: NestedInstructions::default(),
            })
            .collect()
    }

    #[test]
    fn collection_skips_non_matches_and_preserves_order() {
        let input = instructions(&[&[7], &[0], &[9]]);
        let decoded =
            parse_instructions_flat::<Decoded>(&input.iter().collect::<Vec<_>>()).unwrap();
        assert!(std::ptr::eq(decoded[0].instruction(), &input[0]));
        assert!(std::ptr::eq(decoded[1].instruction(), &input[2]));
        let values: Vec<_> = decoded
            .iter()
            .map(|input| (input.instruction().metadata.index, input.decoded().0))
            .collect();
        assert_eq!(values, vec![(0, 7), (2, 9)]);
    }

    struct Collector(Arc<Mutex<Vec<Vec<u8>>>>);

    impl Processor<TransactionProcessorInput<'_, Decoded>> for Collector {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            input: &TransactionProcessorInput<'_, Decoded>,
        ) -> ProcessorResult {
            self.0.lock().unwrap().push(
                input
                    .instructions()
                    .iter()
                    .map(|input| input.decoded().0)
                    .collect(),
            );
            Ok(())
        }
    }

    #[tokio::test]
    async fn transaction_pipe_never_delivers_partial_collections() {
        let received = Arc::new(Mutex::new(Vec::new()));
        let mut filters = Filters::new();
        filters.push(crate::filter::SlotRangeFilter::from(1, None));
        let mut pipe = TransactionPipe::<Decoded, _>::new(Collector(received.clone()), filters);
        let pipeline_id = Id::new("pipeline").unwrap();
        let datasource_id = Id::new("source").unwrap();
        let route_id = Id::new("transactions").unwrap();
        let context = RouteContext::new(&pipeline_id, &datasource_id, &route_id);
        let update = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default()],
                ..Default::default()
            },
            TransactionStatusMeta::default(),
            1,
        )
        .unwrap();
        for input in [
            instructions(&[]),
            instructions(&[&[0]]),
            instructions(&[&[7], &[0], &[9]]),
        ] {
            pipe.run(&context, &update, &input.iter().collect::<Vec<_>>())
                .await
                .unwrap();
        }
        let input = instructions(&[&[7], &[], &[9]]);
        let error = pipe
            .run(&context, &update, &input.iter().collect::<Vec<_>>())
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
