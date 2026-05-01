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
    crate::{error::CarbonResult, filter::Filter, processor::Processor},
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

        if let Some(decoded_account) = self.decoder.decode_account(&account) {
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
