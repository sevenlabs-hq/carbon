//! Instruction route execution and processor input.

use {
    super::{DecodedRouteOptions, ErrorPolicy, RouteContext},
    crate::{
        error::{CarbonResult, Error},
        instruction::{InstructionDecoder, NestedInstruction},
        processor::Processor,
    },
    std::{future::Future, pin::Pin},
};

/// An instruction and its decoded data.
#[derive(Debug)]
pub struct InstructionProcessorInput<'a, T> {
    pub(crate) instruction: &'a NestedInstruction,
    pub(crate) decoded: T,
}

impl<'a, T> InstructionProcessorInput<'a, T> {
    pub fn instruction(&self) -> &'a NestedInstruction {
        self.instruction
    }

    pub fn decoded(&self) -> &T {
        &self.decoded
    }
}

pub(crate) struct InstructionRoute<D, P> {
    decoder: D,
    processor: P,
    options: DecodedRouteOptions<NestedInstruction>,
}

impl<D, P> InstructionRoute<D, P> {
    pub(crate) fn new(
        decoder: D,
        processor: P,
        options: DecodedRouteOptions<NestedInstruction>,
    ) -> Self {
        Self {
            decoder,
            processor,
            options,
        }
    }
}

impl<D, P> InstructionRoute<D, P>
where
    D: InstructionDecoder + Send,
    D::InstructionType: Send + Sync,
    P: for<'a> Processor<InstructionProcessorInput<'a, D::InstructionType>>,
{
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        instruction: &NestedInstruction,
    ) -> CarbonResult<()> {
        if !self
            .options
            .filters
            .filter(context, instruction)
            .await
            .map_err(Error::Filter)?
        {
            return Ok(());
        }

        let decoded = match self.decoder.decode_instruction(&instruction.instruction) {
            Ok(Some(decoded)) => decoded,
            Ok(None) => return Ok(()),
            Err(error) => {
                if self.options.decode_error_policy == ErrorPolicy::Exit {
                    return Err(Error::Decode(error));
                }
                log::error!(
                    "instruction decoding failed in pipeline {}, datasource {}, route {}: {error}",
                    context.pipeline_id(),
                    context.datasource_id(),
                    context.route_id(),
                );
                return Ok(());
            }
        };

        let input = InstructionProcessorInput {
            instruction,
            decoded,
        };
        let result = self.processor.process(context, &input).await;
        if let Err(error) = &result {
            if self.options.processor_error_policy == ErrorPolicy::Exit {
                return result.map_err(Error::Processor);
            }
            log::error!(
                "instruction processing failed in pipeline {}, datasource {}, route {}: {error}",
                context.pipeline_id(),
                context.datasource_id(),
                context.route_id(),
            );
        }

        self.options
            .filters
            .commit(context, instruction, &result)
            .await
            .map_err(Error::FilterCommit)
    }
}

pub(crate) trait DynInstructionRoute: Send {
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        instruction: &'a NestedInstruction,
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>>;
}

impl<D, P> DynInstructionRoute for InstructionRoute<D, P>
where
    D: InstructionDecoder + Send,
    D::InstructionType: Send + Sync,
    P: for<'a> Processor<InstructionProcessorInput<'a, D::InstructionType>>,
{
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        instruction: &'a NestedInstruction,
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>> {
        Box::pin(InstructionRoute::run(self, context, instruction))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            error::BoxError,
            instruction::{InstructionMetadata, NestedInstructions},
            transaction::TransactionMetadata,
        },
        solana_instruction::Instruction,
        solana_pubkey::Pubkey,
        std::sync::Arc,
    };

    fn instruction(data: Vec<u8>) -> NestedInstruction {
        NestedInstruction {
            metadata: InstructionMetadata {
                transaction_metadata: Arc::new(TransactionMetadata::default()),
                stack_height: 1,
                index: 0,
                absolute_path: vec![0],
            },
            instruction: Instruction {
                program_id: Pubkey::new_unique(),
                accounts: vec![],
                data,
            },
            inner_instructions: NestedInstructions::default(),
        }
    }

    struct Decoder(std::cell::Cell<usize>);

    impl InstructionDecoder for Decoder {
        type InstructionType = u8;

        fn decode_instruction(&self, instruction: &Instruction) -> Result<Option<u8>, BoxError> {
            self.0.set(self.0.get() + 1);
            match instruction.data.first() {
                Some(0) => Ok(None),
                Some(value) => Ok(Some(*value)),
                None => Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "missing instruction data",
                )
                .into()),
            }
        }
    }

    struct Counter(Arc<std::sync::atomic::AtomicUsize>);

    impl Processor<InstructionProcessorInput<'_, u8>> for Counter {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            input: &InstructionProcessorInput<'_, u8>,
        ) -> crate::processor::ProcessorResult {
            assert_eq!(*input.decoded(), input.instruction().instruction.data[0]);
            self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        }
    }

    #[tokio::test]
    async fn instruction_route_handles_matches_non_matches_and_errors() {
        use std::{
            io,
            sync::atomic::{AtomicUsize, Ordering},
        };
        let pipeline_id = crate::id::Id::new("pipeline").unwrap();
        let datasource_id = crate::id::Id::new("source").unwrap();
        let route_id = crate::id::Id::new("instructions").unwrap();
        let context = RouteContext::new(&pipeline_id, &datasource_id, &route_id);
        let calls = Arc::new(AtomicUsize::new(0));
        let mut route = InstructionRoute::new(
            Decoder(std::cell::Cell::new(0)),
            Counter(calls.clone()),
            DecodedRouteOptions::default().decode_error_policy(ErrorPolicy::Exit),
        );
        for data in [vec![7], vec![0], vec![]] {
            let is_error = data.is_empty();
            let result = route.run(&context, &instruction(data)).await;
            if is_error {
                let Error::Decode(error) = result.unwrap_err() else {
                    panic!("expected decoder error");
                };
                assert_eq!(
                    error.downcast_ref::<io::Error>().unwrap().kind(),
                    io::ErrorKind::UnexpectedEof
                );
            } else {
                result.unwrap();
            }
            assert_eq!(calls.load(Ordering::Relaxed), 1);
        }

        let mut route = InstructionRoute::new(
            Decoder(std::cell::Cell::new(0)),
            Counter(calls.clone()),
            DecodedRouteOptions::default(),
        );
        route.run(&context, &instruction(vec![])).await.unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    struct Failing;

    impl Processor<InstructionProcessorInput<'_, u8>> for Failing {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            _input: &InstructionProcessorInput<'_, u8>,
        ) -> crate::processor::ProcessorResult {
            Err(std::io::Error::other("processor").into())
        }
    }

    struct CommitObserver {
        calls: Arc<std::sync::Mutex<Vec<usize>>>,
        index: usize,
        fail: bool,
    }

    impl crate::filter::Filter<NestedInstruction> for CommitObserver {
        async fn filter(
            &mut self,
            _context: &RouteContext<'_>,
            _instruction: &NestedInstruction,
        ) -> Result<bool, BoxError> {
            Ok(true)
        }

        async fn commit(
            &mut self,
            _context: &RouteContext<'_>,
            _instruction: &NestedInstruction,
            result: &crate::processor::ProcessorResult,
        ) -> Result<(), BoxError> {
            assert_eq!(
                result
                    .as_ref()
                    .unwrap_err()
                    .downcast_ref::<std::io::Error>()
                    .unwrap()
                    .to_string(),
                "processor"
            );
            self.calls.lock().unwrap().push(self.index);
            if self.fail {
                return Err(std::io::Error::other(format!("commit {}", self.index)).into());
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn processor_policy_controls_commits_and_commit_errors_remain_fatal() {
        let ids = ["pipeline", "source", "route"].map(|id| crate::id::Id::new(id).unwrap());
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        for policy in [ErrorPolicy::Exit, ErrorPolicy::Continue] {
            for fail in [false, true] {
                let calls = Arc::new(std::sync::Mutex::new(Vec::new()));
                let mut options = DecodedRouteOptions::default().processor_error_policy(policy);
                for index in 0..2 {
                    options = options.filter(CommitObserver {
                        calls: calls.clone(),
                        index,
                        fail,
                    });
                }
                let mut route =
                    InstructionRoute::new(Decoder(std::cell::Cell::new(0)), Failing, options);
                let result = route.run(&context, &instruction(vec![7])).await;
                if policy == ErrorPolicy::Exit {
                    assert!(matches!(result, Err(Error::Processor(_))));
                    assert!(calls.lock().unwrap().is_empty());
                    continue;
                }
                assert_eq!(*calls.lock().unwrap(), [0, 1]);
                if fail {
                    let Error::FilterCommit(error) = result.unwrap_err() else {
                        panic!("expected filter commit error");
                    };
                    assert_eq!(
                        error.downcast_ref::<std::io::Error>().unwrap().to_string(),
                        "commit 0"
                    );
                } else {
                    result.unwrap();
                }
            }
        }
    }

    #[test]
    fn input_borrows_instruction_and_owns_decoded_data() {
        let instruction = instruction(vec![7]);
        let decoded = String::from("decoded instruction");
        let decoded_ptr = decoded.as_ptr();
        let input = InstructionProcessorInput {
            instruction: &instruction,
            decoded,
        };

        assert!(std::ptr::eq(input.instruction(), &instruction));
        assert_eq!(input.decoded(), "decoded instruction");
        assert_eq!(input.decoded().as_ptr(), decoded_ptr);
        assert_eq!(input.instruction().metadata.absolute_path, vec![0]);
    }
}
