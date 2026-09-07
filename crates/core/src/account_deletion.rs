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
        filter::Filters,
        processor::Processor,
        route::RouteContext,
        update::AccountClosureUpdate,
    },
    async_trait::async_trait,
};

pub struct AccountDeletionPipe<P> {
    processor: P,
    filters: Filters<AccountClosureUpdate>,
}

impl<P> AccountDeletionPipe<P> {
    pub fn new(processor: P, filters: Filters<AccountClosureUpdate>) -> Self {
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
        if !self
            .filters
            .filter(context, account_deletion)
            .await
            .map_err(Error::Filter)?
        {
            return Ok(());
        }

        let result = self.processor.process(context, account_deletion).await;

        if result.is_ok() {
            self.filters
                .commit(context, account_deletion, &result)
                .await
                .map_err(Error::FilterCommit)?;
        }

        result.map_err(Error::Processor)?;

        Ok(())
    }
}
