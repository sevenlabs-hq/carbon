//! Account route execution and processor input.

use {
    super::{DecodedRouteOptions, ErrorPolicy, RouteContext},
    crate::{
        account::AccountDecoder,
        error::{CarbonResult, Error},
        processor::Processor,
        update::AccountUpdate,
    },
    std::{future::Future, pin::Pin},
};

/// An account update and its decoded data.
#[derive(Debug)]
pub struct AccountProcessorInput<'a, T> {
    pub(crate) update: &'a AccountUpdate,
    pub(crate) decoded: T,
}

impl<'a, T> AccountProcessorInput<'a, T> {
    pub fn update(&self) -> &'a AccountUpdate {
        self.update
    }

    pub fn decoded(&self) -> &T {
        &self.decoded
    }
}

pub(crate) struct AccountRoute<D, P> {
    decoder: D,
    processor: P,
    options: DecodedRouteOptions<AccountUpdate>,
}

impl<D, P> AccountRoute<D, P> {
    pub(crate) fn new(
        decoder: D,
        processor: P,
        options: DecodedRouteOptions<AccountUpdate>,
    ) -> Self {
        Self {
            decoder,
            processor,
            options,
        }
    }
}

impl<D, P> AccountRoute<D, P>
where
    D: AccountDecoder + Send,
    D::AccountType: Send + Sync,
    P: for<'a> Processor<AccountProcessorInput<'a, D::AccountType>>,
{
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        update: &AccountUpdate,
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

        let decoded = match self
            .decoder
            .decode_account(update.pubkey(), update.account())
        {
            Ok(Some(decoded)) => decoded,
            Ok(None) => return Ok(()),
            Err(error) => {
                if self.options.decode_error_policy == ErrorPolicy::Exit {
                    return Err(Error::Decode(error));
                }
                log::error!(
                    "account decoding failed in pipeline {}, datasource {}, route {}: {error}",
                    context.pipeline_id(),
                    context.datasource_id(),
                    context.route_id(),
                );
                return Ok(());
            }
        };

        let input = AccountProcessorInput { update, decoded };
        let result = self.processor.process(context, &input).await;
        if let Err(error) = &result {
            if self.options.processor_error_policy == ErrorPolicy::Exit {
                return result.map_err(Error::Processor);
            }
            log::error!(
                "account processing failed in pipeline {}, datasource {}, route {}: {error}",
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

pub(crate) trait DynAccountRoute: Send {
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        update: &'a AccountUpdate,
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>>;
}

impl<D, P> DynAccountRoute for AccountRoute<D, P>
where
    D: AccountDecoder + Send,
    D::AccountType: Send + Sync,
    P: for<'a> Processor<AccountProcessorInput<'a, D::AccountType>>,
{
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        update: &'a AccountUpdate,
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>> {
        Box::pin(AccountRoute::run(self, context, update))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{error::BoxError, processor::ProcessorResult},
        solana_account::Account,
        solana_pubkey::Pubkey,
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
            if matches!(value.account().data[0], 5 | 7) {
                assert_eq!(
                    result
                        .as_ref()
                        .unwrap_err()
                        .downcast_ref::<io::Error>()
                        .unwrap()
                        .to_string(),
                    "processor"
                );
            } else {
                assert!(result.is_ok());
            }
            self.record("commit");
            if matches!(value.account().data[0], 6 | 7) {
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
            if matches!(*input.decoded(), 5 | 7) {
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
        let options = DecodedRouteOptions::default().filter(trace.clone());
        let mut route = AccountRoute::new(trace.clone(), trace.clone(), options);
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
            let result = route.run(&context, &update).await;
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
                5 => assert!(matches!(result, Err(Error::Processor(_)))),
                6 => assert!(matches!(result, Err(Error::FilterCommit(_)))),
                _ => result.unwrap(),
            }
        }
    }

    #[tokio::test]
    async fn continued_processor_errors_reach_every_commit() {
        let ids = ["pipeline", "source", "route"].map(|id| crate::id::Id::new(id).unwrap());
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let trace = Trace(Arc::new(std::sync::Mutex::new(Vec::new())));
        let options = DecodedRouteOptions::default()
            .filter(trace.clone())
            .filter(trace.clone())
            .processor_error_policy(ErrorPolicy::Continue);
        let mut route = AccountRoute::new(trace.clone(), trace.clone(), options);

        for mode in [5, 7] {
            trace.0.lock().unwrap().clear();
            let update = AccountUpdate::new(
                Pubkey::new_unique(),
                Account {
                    data: vec![mode],
                    ..Default::default()
                },
                1,
            );
            let result = route.run(&context, &update).await;
            assert_eq!(
                *trace.0.lock().unwrap(),
                ["filter", "filter", "decode", "process", "commit", "commit"]
            );
            if mode == 7 {
                let Error::FilterCommit(error) = result.unwrap_err() else {
                    panic!("expected filter commit error");
                };
                assert_eq!(
                    error.downcast_ref::<io::Error>().unwrap().to_string(),
                    "commit"
                );
            } else {
                result.unwrap();
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
    async fn account_route_handles_matches_non_matches_and_errors() {
        let pipeline_id = crate::id::Id::new("pipeline").unwrap();
        let datasource_id = crate::id::Id::new("source").unwrap();
        let route_id = crate::id::Id::new("accounts").unwrap();
        let context = RouteContext::new(&pipeline_id, &datasource_id, &route_id);
        let pubkey = Pubkey::new_unique();
        let calls = Arc::new(AtomicUsize::new(0));
        let mut route = AccountRoute::new(
            Decoder {
                pubkey,
                calls: Cell::new(0),
            },
            Counter(calls.clone()),
            DecodedRouteOptions::default().decode_error_policy(ErrorPolicy::Exit),
        );
        for data in [vec![7], vec![0], vec![]] {
            let is_error = data.is_empty();
            let result = route
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

    #[test]
    fn input_borrows_update_and_owns_decoded_data() {
        let update =
            AccountUpdate::new(Pubkey::new_unique(), Account::default(), 7).with_write_version(12);
        let decoded = String::from("decoded account");
        let decoded_ptr = decoded.as_ptr();
        let input = AccountProcessorInput {
            update: &update,
            decoded,
        };

        assert!(std::ptr::eq(input.update(), &update));
        assert_eq!(input.decoded(), "decoded account");
        assert_eq!(input.decoded().as_ptr(), decoded_ptr);
        assert_eq!(input.update().write_version(), Some(12));
    }
}
