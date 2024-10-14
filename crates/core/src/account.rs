use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;

use crate::{error::CarbonResult, processor::Processor};

#[derive(Debug, Clone)]
pub struct AccountMetadata {
    pub slot: u64,
    pub pubkey: Pubkey,
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
        account: &'a solana_sdk::account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>>;
}

pub struct AccountPipe<T: Send> {
    pub decoder: Box<dyn for<'a> AccountDecoder<'a, AccountType = T> + Send + Sync + 'static>,
    pub processor:
        Box<dyn Processor<InputType = (AccountMetadata, DecodedAccount<T>)> + Send + Sync>,
}

#[async_trait]
pub trait AccountPipes: Send + Sync {
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_sdk::account::Account),
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<T: Send> AccountPipes for AccountPipe<T> {
    async fn run(
        &mut self,
        account_with_metadata: (AccountMetadata, solana_sdk::account::Account),
    ) -> CarbonResult<()> {
        if let Some(decoded_account) = self.decoder.decode_account(&account_with_metadata.1) {
            self.processor
                .process((account_with_metadata.0, decoded_account))
                .await?;
        }
        Ok(())
    }
}
