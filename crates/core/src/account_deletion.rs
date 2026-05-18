//! Account-deletion pipe wiring.
//!
//! # Components
//!
//! - [`AccountDeletionPipe`] — internal pipe wrapping the user processor and
//!   filters for `Update::AccountDeletion`. Constructed by
//!   `PipelineBuilder::account_deletions(...)` and
//!   `account_deletions_with_filters(...)`.
//! - [`AccountDeletionPipes`] — dyn-dispatch trait the pipeline holds as
//!   `Box<dyn AccountDeletionPipes>`.

use {
    crate::{
        datasource::AccountDeletion, error::CarbonResult, filter::Filter, processor::Processor,
    },
    async_trait::async_trait,
};

pub struct AccountDeletionPipe<P> {
    processor: P,
    filters: Vec<Box<dyn Filter + 'static>>,
}

impl<P> AccountDeletionPipe<P> {
    pub fn new(processor: P, filters: Vec<Box<dyn Filter + 'static>>) -> Self {
        Self { processor, filters }
    }
}

#[async_trait]
pub trait AccountDeletionPipes: Send + Sync {
    async fn run(&mut self, account_deletion: AccountDeletion) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
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

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}
