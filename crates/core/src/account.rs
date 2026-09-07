//! Account-shaped pipe wiring + per-account metadata and decoder traits.
//!
//! # Components
//!
//! - [`AccountMetadata`] — slot/pubkey/signature trio attached to every decoded
//!   account.
//! - [`AccountDecoder`] — user-implemented trait that turns a raw
//!   `solana_account::Account` into decoded data.
//! - [`AccountProcessorInput<'a, T>`] — borrowed bundle the pipeline passes
//!   to user processors.
//! - [`AccountPipe`] / [`AccountPipes`] — internal pipe wrapping the decoder +
//!   processor + filters; constructed by `PipelineBuilder`.

use {
    crate::{
        error::{BoxError, CarbonResult, Error},
        filter::Filters,
        processor::Processor,
        route::{AccountProcessorInput, RouteContext},
        update::AccountUpdate,
    },
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
};

/// Slot + identity context attached to every decoded account.
#[derive(Debug, Clone)]
pub struct AccountMetadata {
    pub slot: u64,
    pub pubkey: Pubkey,
    pub transaction_signature: Option<Signature>,
}

/// Decodes account data. `Ok(None)` means the account is outside this decoder's scope.
pub trait AccountDecoder {
    type AccountType;

    fn decode_account(
        &self,
        pubkey: &Pubkey,
        account: &solana_account::Account,
    ) -> Result<Option<Self::AccountType>, BoxError>;
}

impl From<&AccountUpdate> for AccountMetadata {
    fn from(update: &AccountUpdate) -> Self {
        Self {
            slot: update.slot(),
            pubkey: *update.pubkey(),
            transaction_signature: update.transaction_signature().copied(),
        }
    }
}

pub struct AccountPipe<T, P> {
    decoder: Box<dyn AccountDecoder<AccountType = T> + Send + 'static>,
    processor: P,
    filters: Filters<AccountUpdate>,
}

impl<T, P> AccountPipe<T, P> {
    pub fn new(
        decoder: Box<dyn AccountDecoder<AccountType = T> + Send + 'static>,
        processor: P,
        filters: Filters<AccountUpdate>,
    ) -> Self {
        Self {
            decoder,
            processor,
            filters,
        }
    }
}

#[async_trait]
pub trait AccountPipes: Send {
    async fn run(&mut self, context: &RouteContext<'_>, update: &AccountUpdate)
        -> CarbonResult<()>;
}

#[async_trait]
impl<T, P> AccountPipes for AccountPipe<T, P>
where
    T: Send + Sync,
    P: for<'a> Processor<AccountProcessorInput<'a, T>>,
{
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        update: &AccountUpdate,
    ) -> CarbonResult<()> {
        if !self
            .filters
            .filter(context, update)
            .await
            .map_err(Error::Filter)?
        {
            return Ok(());
        }

        if let Some(decoded_account) = self
            .decoder
            .decode_account(update.pubkey(), update.account())
            .map_err(Error::Decode)?
        {
            let input = AccountProcessorInput {
                update,
                decoded: decoded_account,
            };
            let result = self.processor.process(context, &input).await;

            if result.is_ok() {
                self.filters
                    .commit(context, update, &result)
                    .await
                    .map_err(Error::FilterCommit)?;
            }

            result.map_err(Error::Processor)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::processor::ProcessorResult,
        solana_account::Account,
        std::{
            cell::Cell,
            io,
            sync::{
                atomic::{AtomicUsize, Ordering},
                Arc,
            },
        },
    };

    struct Decoder {
        pubkey: Pubkey,
        calls: Cell<usize>,
    }

    impl AccountDecoder for Decoder {
        type AccountType = u8;

        fn decode_account(
            &self,
            pubkey: &Pubkey,
            account: &Account,
        ) -> Result<Option<u8>, BoxError> {
            assert_eq!(pubkey, &self.pubkey);
            self.calls.set(self.calls.get() + 1);
            match account.data.first() {
                Some(0) => Ok(None),
                Some(value) => Ok(Some(*value)),
                None => {
                    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "missing account data").into())
                }
            }
        }
    }

    #[derive(Clone)]
    struct Trace(Arc<std::sync::Mutex<Vec<&'static str>>>);

    impl Trace {
        fn record(&self, event: &'static str) {
            self.0.lock().unwrap().push(event);
        }
    }

    impl crate::filter::Filter<AccountUpdate> for Trace {
        async fn filter(
            &mut self,
            _context: &RouteContext<'_>,
            value: &AccountUpdate,
        ) -> Result<bool, BoxError> {
            self.record("filter");
            match value.account().data[0] {
                1 => Ok(false),
                2 => Err(io::Error::other("filter").into()),
                _ => Ok(true),
            }
        }

        async fn commit(
            &mut self,
            _context: &RouteContext<'_>,
            value: &AccountUpdate,
            result: &ProcessorResult,
        ) -> Result<(), BoxError> {
            assert!(result.is_ok());
            self.record("commit");
            if value.account().data[0] == 6 {
                return Err(io::Error::other("commit").into());
            }
            Ok(())
        }
    }

    impl AccountDecoder for Trace {
        type AccountType = u8;
        fn decode_account(
            &self,
            _pubkey: &Pubkey,
            account: &Account,
        ) -> Result<Option<u8>, BoxError> {
            self.record("decode");
            match account.data[0] {
                3 => Ok(None),
                4 => Err(io::Error::other("decode").into()),
                mode => Ok(Some(mode)),
            }
        }
    }

    impl Processor<AccountProcessorInput<'_, u8>> for Trace {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            input: &AccountProcessorInput<'_, u8>,
        ) -> ProcessorResult {
            self.record("process");
            if *input.decoded() == 5 {
                return Err(io::Error::other("processor").into());
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn filters_and_commits_observe_route_boundaries() {
        let ids = ["pipeline", "source", "route"].map(|id| crate::id::Id::new(id).unwrap());
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let trace = Trace(Arc::new(std::sync::Mutex::new(Vec::new())));
        let mut filters = Filters::new();
        filters.push(trace.clone());
        let mut pipe = AccountPipe::new(Box::new(trace.clone()), trace.clone(), filters);
        for mode in 0..=6 {
            trace.0.lock().unwrap().clear();
            let update = AccountUpdate::new(
                Pubkey::new_unique(),
                Account {
                    data: vec![mode],
                    ..Default::default()
                },
                1,
            );
            let result = pipe.run(&context, &update).await;
            let expected = match mode {
                0 | 6 => vec!["filter", "decode", "process", "commit"],
                1 | 2 => vec!["filter"],
                3 | 4 => vec!["filter", "decode"],
                5 => vec!["filter", "decode", "process"],
                _ => unreachable!(),
            };
            assert_eq!(*trace.0.lock().unwrap(), expected);
            match mode {
                2 => assert!(matches!(result, Err(Error::Filter(_)))),
                4 => assert!(matches!(result, Err(Error::Decode(_)))),
                5 => assert!(matches!(result, Err(Error::Processor(_)))),
                6 => assert!(matches!(result, Err(Error::FilterCommit(_)))),
                _ => result.unwrap(),
            }
        }
    }

    struct Counter(Arc<AtomicUsize>);

    impl Processor<AccountProcessorInput<'_, u8>> for Counter {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            input: &AccountProcessorInput<'_, u8>,
        ) -> ProcessorResult {
            assert_eq!(*input.decoded(), input.update().account().data[0]);
            assert_eq!(input.update().account().lamports, 42);
            self.0.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[tokio::test]
    async fn account_pipe_handles_matches_non_matches_and_errors() {
        let pipeline_id = crate::id::Id::new("pipeline").unwrap();
        let datasource_id = crate::id::Id::new("source").unwrap();
        let route_id = crate::id::Id::new("accounts").unwrap();
        let context = RouteContext::new(&pipeline_id, &datasource_id, &route_id);
        let pubkey = Pubkey::new_unique();
        let calls = Arc::new(AtomicUsize::new(0));
        let mut pipe = AccountPipe::new(
            Box::new(Decoder {
                pubkey,
                calls: Cell::new(0),
            }),
            Counter(calls.clone()),
            Filters::new(),
        );
        for data in [vec![7], vec![0], vec![]] {
            let is_error = data.is_empty();
            let result = pipe
                .run(
                    &context,
                    &AccountUpdate::new(
                        pubkey,
                        Account {
                            data,
                            lamports: 42,
                            ..Default::default()
                        },
                        1,
                    ),
                )
                .await;
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
    }
}
