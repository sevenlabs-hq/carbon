use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    datasource::AccountDeletion, error::CarbonResult, metrics::Metrics, processor::Processor,
};

pub struct AccountDeletionPipe {
    pub processor: Box<dyn Processor<InputType = AccountDeletion> + Send + Sync>,
}

#[async_trait]
pub trait AccountDeletionPipes {
    async fn run(
        &mut self,
        account_deletion: AccountDeletion,
        metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl AccountDeletionPipes for AccountDeletionPipe {
    async fn run(
        &mut self,
        account_deletion: AccountDeletion,
        metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()> {
        self.processor.process(account_deletion, metrics).await?;

        Ok(())
    }
}
