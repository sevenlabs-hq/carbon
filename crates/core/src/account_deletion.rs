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
        error::{CarbonResult, Error},
        filter::Filter,
        processor::Processor,
        route::RouteContext,
        update::AccountClosureUpdate,
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
pub trait AccountDeletionPipes: Send {
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        account_deletion: &AccountClosureUpdate,
    ) -> CarbonResult<()>;

    fn filters(&self) -> &[Box<dyn Filter + 'static>];
}

#[async_trait]
impl<P> AccountDeletionPipes for AccountDeletionPipe<P>
where
    P: Processor<AccountClosureUpdate>,
{
    async fn run(
        &mut self,
        context: &RouteContext<'_>,
        account_deletion: &AccountClosureUpdate,
    ) -> CarbonResult<()> {
        self.processor
            .process(context, account_deletion)
            .await
            .map_err(Error::Processor)?;

        Ok(())
    }

    fn filters(&self) -> &[Box<dyn Filter + 'static>] {
        &self.filters
    }
}
