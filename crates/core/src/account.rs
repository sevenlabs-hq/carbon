//! Account-shaped pipe wiring + per-account metadata and decoder traits.
//!
//! # Components
//!
//! - [`AccountMetadata`] — slot/pubkey/signature trio attached to every decoded
//!   account.
//! - [`DecodedAccount<T>`] — decoder output: typed `data` plus the account's
//!   lamports/owner/executable/rent_epoch fields.
//! - [`AccountDecoder`] — user-implemented trait that turns a raw
//!   `solana_account::Account` into `Option<DecodedAccount<T>>`.
//! - [`AccountProcessorInputType<'a, T>`] — borrowed bundle the pipeline passes
//!   to user processors.
//! - [`AccountPipe`] / [`AccountPipes`] — internal pipe wrapping the decoder +
//!   processor + filters; constructed by `PipelineBuilder`.

use {
    crate::{
        error::CarbonResult, filter::Filter, pipeline::ACCOUNT_DECODE_FAILED, processor::Processor,
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
    pub bank_id: Option<u64>,
}

/// Decoder output: the typed account body plus the standard account
/// header fields.
#[derive(Debug, Clone)]
pub struct DecodedAccount<T> {
    pub lamports: u64,
    pub data: T,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// User-implemented decoder mapping a raw `solana_account::Account` to a
/// typed `DecodedAccount<Self::AccountType>`. Returning `None` means
/// "this account isn't mine" — the pipe skips it.
pub trait AccountDecoder<'a> {
    type AccountType;

    fn decode_account(
        &self,
        account: &'a solana_account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>>;

    /// Program these accounts belong to, when it is known statically.
    ///
    /// The pipe uses it to tell a foreign account (normal, not counted) apart
    /// from an account owned by this program that failed to decode (counted in
    /// `carbon_account_decode_failed_total`). Defaults to `None`, so decoders
    /// that do not override it keep the old silent-skip behaviour.
    fn program_id(&self) -> Option<Pubkey> {
        None
    }
}

/// Borrowed bundle handed to a `Processor<AccountProcessorInputType<T>>`.
///
/// Includes both the typed decoded form and the raw account so processors
/// that need fields the decoder discarded can recover them.
#[derive(Debug)]
pub struct AccountProcessorInputType<'a, T> {
    pub metadata: &'a AccountMetadata,
    pub decoded_account: &'a DecodedAccount<T>,
    pub raw_account: &'a solana_account::Account,
}

pub struct AccountPipe<T, P> {
    decoder: Box<dyn for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static>,
    processor: P,
    filters: Vec<Box<dyn Filter + 'static>>,
}

impl<T, P> AccountPipe<T, P> {
    pub fn new(
        decoder: Box<dyn for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static>,
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
pub trait AccountPipes: Send + Sync {
    async fn run(
        &mut self,
        account_metadata: &AccountMetadata,
        account: &solana_account::Account,
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
        account_metadata: &AccountMetadata,
        account: &solana_account::Account,
    ) -> CarbonResult<()> {
        match self.decoder.decode_account(account) {
            Some(decoded_account) => {
                let data = AccountProcessorInputType {
                    metadata: account_metadata,
                    decoded_account: &decoded_account,
                    raw_account: account,
                };

                self.processor.process(&data).await?;
            }
            None => {
                if self.decoder.program_id() == Some(account.owner) {
                    ACCOUNT_DECODE_FAILED.inc();
                    log::debug!(
                        "failed to decode account owned by the decoder's program. pubkey: {}, slot: {}",
                        account_metadata.pubkey,
                        account_metadata.slot
                    );
                }
            }
        }
        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::pipeline::ACCOUNT_DECODE_FAILED};

    const TEST_PROGRAM_ID: Pubkey = Pubkey::from_str_const("11111111111111111111111111111111");

    struct TestAccount;

    struct TestDecoder;

    impl<'a> AccountDecoder<'a> for TestDecoder {
        type AccountType = TestAccount;

        fn decode_account(
            &self,
            account: &'a solana_account::Account,
        ) -> Option<DecodedAccount<Self::AccountType>> {
            if account.owner != TEST_PROGRAM_ID {
                return None;
            }

            if account.data.is_empty() {
                return None;
            }

            Some(DecodedAccount {
                lamports: account.lamports,
                data: TestAccount,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            })
        }

        fn program_id(&self) -> Option<Pubkey> {
            Some(TEST_PROGRAM_ID)
        }
    }

    struct TestProcessor;

    impl<'a> Processor<AccountProcessorInputType<'a, TestAccount>> for TestProcessor {
        async fn process(
            &mut self,
            _data: &AccountProcessorInputType<'a, TestAccount>,
        ) -> CarbonResult<()> {
            Ok(())
        }
    }

    fn account(owner: Pubkey, data: Vec<u8>) -> solana_account::Account {
        solana_account::Account {
            lamports: 1,
            data,
            owner,
            executable: false,
            rent_epoch: 0,
        }
    }

    #[tokio::test]
    async fn decode_failures_are_counted_only_for_the_decoders_own_program() {
        let mut pipe = AccountPipe::new(Box::new(TestDecoder), TestProcessor, Vec::new());
        let metadata = AccountMetadata {
            slot: 1,
            pubkey: Pubkey::new_unique(),
            transaction_signature: None,
            bank_id: None,
        };

        let before = ACCOUNT_DECODE_FAILED.get();

        // Foreign account that also fails to decode: not counted.
        pipe.run(&metadata, &account(Pubkey::new_unique(), Vec::new()))
            .await
            .unwrap();
        assert_eq!(ACCOUNT_DECODE_FAILED.get(), before);

        // Own account with valid data: not counted.
        pipe.run(&metadata, &account(TEST_PROGRAM_ID, vec![1]))
            .await
            .unwrap();
        assert_eq!(ACCOUNT_DECODE_FAILED.get(), before);

        // Own account that fails to decode: counted once.
        pipe.run(&metadata, &account(TEST_PROGRAM_ID, Vec::new()))
            .await
            .unwrap();
        assert_eq!(ACCOUNT_DECODE_FAILED.get(), before + 1);
    }
}
