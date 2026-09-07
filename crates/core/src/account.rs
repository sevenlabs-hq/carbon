//! Account-shaped pipe wiring + per-account metadata and decoder traits.
//!
//! # Components
//!
//! - [`AccountMetadata`] — slot/pubkey/signature trio attached to every decoded
//!   account.
//! - [`AccountDecoder`] — user-implemented trait that turns a raw
//!   `solana_account::Account` into decoded data.
//! - [`AccountProcessorInputType<'a, T>`] — borrowed bundle the pipeline passes
//!   to user processors.
//! - [`AccountPipe`] / [`AccountPipes`] — internal pipe wrapping the decoder +
//!   processor + filters; constructed by `PipelineBuilder`.

use {
    crate::{
        error::{BoxError, CarbonResult, Error},
        filter::Filter,
        processor::Processor,
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

/// Borrowed bundle handed to a `Processor<AccountProcessorInputType<T>>`.
///
/// Includes both the typed decoded form and the raw account so processors
/// that need fields the decoder discarded can recover them.
#[derive(Debug)]
pub struct AccountProcessorInputType<'a, T> {
    pub metadata: &'a AccountMetadata,
    pub decoded_account: &'a T,
    pub raw_account: &'a solana_account::Account,
}

pub struct AccountPipe<T, P> {
    decoder: Box<dyn AccountDecoder<AccountType = T> + Send + 'static>,
    processor: P,
    filters: Vec<Box<dyn Filter + 'static>>,
}

impl<T, P> AccountPipe<T, P> {
    pub fn new(
        decoder: Box<dyn AccountDecoder<AccountType = T> + Send + 'static>,
        processor: P,
        filters: Vec<Box<dyn Filter + 'static>>,
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
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_account::Account),
    ) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<T, P> AccountPipes for AccountPipe<T, P>
where
    T: Send + Sync,
    P: for<'a> Processor<AccountProcessorInputType<'a, T>> + Send + Sync,
{
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_account::Account),
    ) -> CarbonResult<()> {
        let (account_metadata, account) = account_with_metadata;

        if let Some(decoded_account) = self
            .decoder
            .decode_account(&account_metadata.pubkey, &account)
            .map_err(Error::Decode)?
        {
            let data = AccountProcessorInputType {
                metadata: &account_metadata,
                decoded_account: &decoded_account,
                raw_account: &account,
            };

            self.processor.process(&data).await?;
        }
        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
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

    struct Counter(Arc<AtomicUsize>);

    impl Processor<AccountProcessorInputType<'_, u8>> for Counter {
        async fn process(&mut self, input: &AccountProcessorInputType<'_, u8>) -> CarbonResult<()> {
            assert_eq!(*input.decoded_account, input.raw_account.data[0]);
            assert_eq!(input.raw_account.lamports, 42);
            self.0.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[tokio::test]
    async fn account_pipe_handles_matches_non_matches_and_errors() {
        let pubkey = Pubkey::new_unique();
        let calls = Arc::new(AtomicUsize::new(0));
        let mut pipe = AccountPipe::new(
            Box::new(Decoder {
                pubkey,
                calls: Cell::new(0),
            }),
            Counter(calls.clone()),
            vec![],
        );
        for data in [vec![7], vec![0], vec![]] {
            let is_error = data.is_empty();
            let result = pipe
                .run((
                    AccountMetadata {
                        slot: 1,
                        pubkey,
                        transaction_signature: None,
                    },
                    Account {
                        data,
                        lamports: 42,
                        ..Default::default()
                    },
                ))
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
