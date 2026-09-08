//! Transaction route execution and processor input.

use {
    super::{DecodedRouteOptions, ErrorPolicy, InstructionProcessorInput, RouteContext},
    crate::{
        collection::InstructionDecoderCollection,
        error::{BoxError, CarbonResult, Error},
        instruction::NestedInstruction,
        processor::Processor,
        update::TransactionUpdate,
    },
    std::{future::Future, marker::PhantomData, pin::Pin},
};

/// A transaction update and its decoded instructions in depth-first preorder.
#[derive(Debug)]
pub struct TransactionProcessorInput<'a, T> {
    pub(crate) update: &'a TransactionUpdate,
    pub(crate) instructions: &'a [InstructionProcessorInput<'a, T>],
}

impl<'a, T> TransactionProcessorInput<'a, T> {
    pub fn update(&self) -> &'a TransactionUpdate {
        self.update
    }

    pub fn instructions(&self) -> &[InstructionProcessorInput<'a, T>] {
        self.instructions
    }
}

pub(crate) struct TransactionRoute<C, P> {
    processor: P,
    options: DecodedRouteOptions<TransactionUpdate>,
    collection: PhantomData<C>,
}

impl<C, P> TransactionRoute<C, P> {
    pub(crate) fn new(processor: P, options: DecodedRouteOptions<TransactionUpdate>) -> Self {
        Self {
            processor,
            options,
            collection: PhantomData,
        }
    }
}

impl<C, P> TransactionRoute<C, P>
where
    C: InstructionDecoderCollection + Send + Sync,
    P: for<'a> Processor<TransactionProcessorInput<'a, C>>,
{
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        update: &TransactionUpdate,
        instructions: &[&NestedInstruction],
    ) -> CarbonResult<()> {
        if !self
            .options
            .filters
            .filter(context, update)
            .await
            .map_err(Error::Filter)?
        {
            return Ok(());
        }

        let decoded = match decode_instructions::<C>(instructions) {
            Ok(decoded) => decoded,
            Err(error) => {
                if self.options.decode_error_policy == ErrorPolicy::Exit {
                    return Err(Error::Decode(error));
                }
                log::error!(
                    "transaction decoding failed in pipeline {}, datasource {}, route {}: {error}",
                    context.pipeline_id(),
                    context.datasource_id(),
                    context.route_id(),
                );
                return Ok(());
            }
        };

        let input = TransactionProcessorInput {
            update,
            instructions: &decoded,
        };
        let result = self.processor.process(context, &input).await;
        if let Err(error) = &result {
            if self.options.processor_error_policy == ErrorPolicy::Exit {
                return result.map_err(Error::Processor);
            }
            log::error!(
                "transaction processing failed in pipeline {}, datasource {}, route {}: {error}",
                context.pipeline_id(),
                context.datasource_id(),
                context.route_id(),
            );
        }

        self.options
            .filters
            .commit(context, update, &result)
            .await
            .map_err(Error::FilterCommit)
    }
}

pub(crate) trait DynTransactionRoute: Send {
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        update: &'a TransactionUpdate,
        instructions: &'a [&NestedInstruction],
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>>;
}

impl<C, P> DynTransactionRoute for TransactionRoute<C, P>
where
    C: InstructionDecoderCollection + Send + Sync,
    P: for<'a> Processor<TransactionProcessorInput<'a, C>>,
{
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        update: &'a TransactionUpdate,
        instructions: &'a [&NestedInstruction],
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>> {
        Box::pin(TransactionRoute::run(self, context, update, instructions))
    }
}

fn decode_instructions<'a, T: InstructionDecoderCollection>(
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
            transaction::TransactionMetadata,
        },
        solana_instruction::Instruction,
        solana_pubkey::Pubkey,
        solana_signature::Signature,
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
        let decoded = decode_instructions::<Decoded>(&input.iter().collect::<Vec<_>>()).unwrap();
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
    async fn transaction_route_never_delivers_partial_collections() {
        let received = Arc::new(Mutex::new(Vec::new()));
        let options = DecodedRouteOptions::default()
            .filter(crate::filter::SlotRangeFilter::from(1, None))
            .decode_error_policy(ErrorPolicy::Exit);
        let mut route = TransactionRoute::<Decoded, _>::new(Collector(received.clone()), options);
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
            route
                .run(&context, &update, &input.iter().collect::<Vec<_>>())
                .await
                .unwrap();
        }
        let input = instructions(&[&[7], &[], &[9]]);
        let error = route
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
        let mut route = TransactionRoute::<Decoded, _>::new(
            Collector(received.clone()),
            DecodedRouteOptions::default(),
        );
        route
            .run(&context, &update, &input.iter().collect::<Vec<_>>())
            .await
            .unwrap();
        assert_eq!(*received.lock().unwrap(), vec![vec![], vec![], vec![7, 9]]);
    }

    #[derive(Clone)]
    struct FailureObserver {
        commits: Arc<Mutex<usize>>,
        fail_commit: bool,
    }

    impl Processor<TransactionProcessorInput<'_, Decoded>> for FailureObserver {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            _input: &TransactionProcessorInput<'_, Decoded>,
        ) -> ProcessorResult {
            Err(io::Error::other("processor").into())
        }
    }

    impl crate::filter::Filter<TransactionUpdate> for FailureObserver {
        async fn filter(
            &mut self,
            _context: &RouteContext<'_>,
            _update: &TransactionUpdate,
        ) -> Result<bool, BoxError> {
            Ok(true)
        }

        async fn commit(
            &mut self,
            _context: &RouteContext<'_>,
            _update: &TransactionUpdate,
            result: &ProcessorResult,
        ) -> Result<(), BoxError> {
            assert_eq!(
                result
                    .as_ref()
                    .unwrap_err()
                    .downcast_ref::<io::Error>()
                    .unwrap()
                    .to_string(),
                "processor"
            );
            *self.commits.lock().unwrap() += 1;
            if self.fail_commit {
                return Err(io::Error::other("commit").into());
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn processor_policy_controls_commits_and_commit_errors_remain_fatal() {
        let ids = ["pipeline", "source", "route"].map(|id| Id::new(id).unwrap());
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let update = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default()],
                ..Default::default()
            },
            TransactionStatusMeta::default(),
            1,
        )
        .unwrap();
        let input = instructions(&[&[7]]);
        let nodes = input.iter().collect::<Vec<_>>();

        for policy in [ErrorPolicy::Exit, ErrorPolicy::Continue] {
            for fail_commit in [false, true] {
                let commits = Arc::new(Mutex::new(0));
                let observer = FailureObserver {
                    commits: commits.clone(),
                    fail_commit,
                };
                let options = DecodedRouteOptions::default()
                    .filter(observer.clone())
                    .filter(observer.clone())
                    .processor_error_policy(policy);
                let mut route = TransactionRoute::<Decoded, _>::new(observer, options);
                let result = route.run(&context, &update, &nodes).await;
                if policy == ErrorPolicy::Exit {
                    assert!(matches!(result, Err(Error::Processor(_))));
                    assert_eq!(*commits.lock().unwrap(), 0);
                    continue;
                }
                assert_eq!(*commits.lock().unwrap(), 2);
                if fail_commit {
                    assert!(matches!(result, Err(Error::FilterCommit(_))));
                } else {
                    result.unwrap();
                }
            }
        }
    }

    #[test]
    fn processor_input_borrows_update_and_original_tree_nodes() {
        let update = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default()],
                ..Default::default()
            },
            TransactionStatusMeta::default(),
            7,
        )
        .unwrap();
        let metadata = Arc::new(TransactionMetadata::default());
        let node = |path: Vec<u8>| NestedInstruction {
            metadata: InstructionMetadata {
                transaction_metadata: metadata.clone(),
                stack_height: path.len() as u32,
                index: 0,
                absolute_path: path,
            },
            instruction: Instruction {
                program_id: Pubkey::new_unique(),
                accounts: vec![],
                data: vec![],
            },
            inner_instructions: NestedInstructions::default(),
        };
        let mut root = node(vec![0]);
        root.inner_instructions.push(node(vec![0, 0]));
        let instructions = NestedInstructions(vec![root]);
        let child = &instructions[0].inner_instructions[0];
        let decoded = vec![InstructionProcessorInput {
            instruction: child,
            decoded: String::from("decoded child"),
        }];
        let input = TransactionProcessorInput {
            update: &update,
            instructions: &decoded,
        };

        assert!(std::ptr::eq(input.update(), &update));
        assert!(std::ptr::eq(input.instructions(), decoded.as_slice()));
        assert!(std::ptr::eq(input.instructions()[0].instruction(), child));
        assert_eq!(input.instructions()[0].decoded(), "decoded child");
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0].inner_instructions.len(), 1);

        let empty = TransactionProcessorInput::<String> {
            update: &update,
            instructions: &[],
        };
        assert!(empty.instructions().is_empty());
    }
}
