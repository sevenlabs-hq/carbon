use {
    crate::{
        error::CarbonResult, filter::Filter, metrics::MetricsCollection, processor::Processor,
    },
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::sync::Arc,
};

#[derive(Debug, Clone)]
pub struct AccountMetadata {
    pub slot: u64,
    pub pubkey: Pubkey,
    pub transaction_signature: Option<Signature>,
}

#[derive(Debug, Clone)]
pub struct DecodedAccount<T> {
    pub lamports: u64,
    pub data: T,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

pub trait AccountDecoder<'a> {
    type AccountType;

    fn decode_account(
        &self,
        account: &'a solana_account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>>;
}

pub type AccountProcessorInputType<T> =
    (AccountMetadata, DecodedAccount<T>, solana_account::Account);

pub struct AccountPipe<T: Send> {
    pub decoder: Box<dyn for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static>,
    pub processor: Box<dyn Processor<InputType = AccountProcessorInputType<T>> + Send + Sync>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

#[async_trait]
pub trait AccountPipes: Send + Sync {
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_account::Account),
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
}

#[async_trait]
impl<T: Send> AccountPipes for AccountPipe<T> {
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_account::Account),
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!("AccountPipe::run(account_with_metadata: {account_with_metadata:?}, metrics)",);

        if let Some(decoded_account) = self.decoder.decode_account(&account_with_metadata.1) {
            self.processor
                .process(
                    (
                        account_with_metadata.0.clone(),
                        decoded_account,
                        account_with_metadata.1,
                    ),
                    metrics.clone(),
                )
                .await?;
        }
        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
