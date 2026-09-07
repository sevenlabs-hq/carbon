//! Account-deletion pipe wiring.
//!
//! # Components
//!
//! - [`AccountDeletionPipe`] — internal pipe wrapping the user processor and
//!   filters for `Update::AccountClosure`. Constructed by
//!   `PipelineBuilder::account_deletions(...)` and
//!   `account_deletions_with_filters(...)`.
//! - [`AccountDeletionPipes`] — dyn-dispatch trait the pipeline holds as
//!   `Box<dyn AccountDeletionPipes>`.

use {
    crate::{
        error::CarbonResult, filter::Filter, processor::Processor, update::AccountClosureUpdate,
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
    async fn run(&mut self, account_deletion: AccountClosureUpdate) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<P> AccountDeletionPipes for AccountDeletionPipe<P>
where
    P: Processor<AccountClosureUpdate> + Send + Sync,
{
    async fn run(&mut self, account_deletion: AccountClosureUpdate) -> CarbonResult<()> {
        self.processor.process(&account_deletion).await?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}
