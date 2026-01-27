use {
    crate::{
        datasource::AccountDeletion, error::CarbonResult, filter::Filter,
        metrics::MetricsCollection, processor::Processor,
    },
    async_trait::async_trait,
    std::sync::Arc,
};

pub struct AccountDeletionPipe {
    pub processor: Box<dyn Processor<InputType = AccountDeletion> + Send + Sync>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

#[async_trait]
pub trait AccountDeletionPipes: Send + Sync {
    async fn run(
        &mut self,
        account_deletion: AccountDeletion,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
}

#[async_trait]
impl AccountDeletionPipes for AccountDeletionPipe {
    async fn run(
        &mut self,
        account_deletion: AccountDeletion,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!("AccountDeletionPipe::run(account_deletion: {account_deletion:?}, metrics)",);

        self.processor.process(account_deletion, metrics).await?;

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}
