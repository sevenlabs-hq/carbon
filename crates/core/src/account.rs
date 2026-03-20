use {
    crate::{error::CarbonResult, filter::Filter, processor::Processor},
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
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

#[derive(Debug)]
pub struct AccountProcessorInputType<'a, T> {
    pub metadata: &'a AccountMetadata,
    pub decoded_account: &'a DecodedAccount<T>,
    pub raw_account: &'a solana_account::Account,
}

pub struct AccountPipe<T, P> {
    pub decoder: Box<dyn for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static>,
    pub processor: P,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

#[async_trait]
pub trait AccountPipes: Send + Sync {
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_account::Account),
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
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

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
