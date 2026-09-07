//! Processor contract.

use {
    crate::{error::BoxError, route::RouteContext},
    std::future::Future,
};

pub type ProcessorResult = Result<(), BoxError>;

/// Processes a borrowed input. The route decides how to handle an error.
pub trait Processor<T>: Send
where
    T: Sync,
{
    fn process(
        &mut self,
        context: &RouteContext<'_>,
        value: &T,
    ) -> impl Future<Output = ProcessorResult> + Send;
}
