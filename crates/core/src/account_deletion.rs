use {
    crate::{
        datasource::AccountDeletion, error::CarbonResult, filter::Filter, processor::Processor,
    },
    async_trait::async_trait,
};

pub struct AccountDeletionPipe<P> {
    pub processor: P,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

#[async_trait]
pub trait AccountDeletionPipes: Send + Sync {
    async fn run(&mut self, account_deletion: AccountDeletion) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + Send + Sync + 'static>];
}

#[async_trait]
impl<P> AccountDeletionPipes for AccountDeletionPipe<P>
where
    P: Processor<AccountDeletion> + Send + Sync,
{
    async fn run(&mut self, account_deletion: AccountDeletion) -> CarbonResult<()> {
        self.processor.process(&account_deletion).await?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + Send + Sync + 'static>] {
        &self.filters
    }
}
