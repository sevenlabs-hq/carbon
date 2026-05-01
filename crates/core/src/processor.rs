//! Processor trait — user-defined handler executed for each decoded update.
//!
//! # Flow
//!
//! 1. A datasource emits an `Update` into the pipeline.
//! 2. Matching pipes filter and route the update.
//! 3. Each pipe decodes (or forwards) and invokes `Processor::process`.
//!
//! # Notes
//!
//! - Uses native `impl Future` (Rust 1.75+) since `Processor` is always used as
//!   a generic bound, avoiding per-call boxing.
//! - Dyn-dispatched traits in the crate (`Datasource`, `*Pipes`) retain
//!   `#[async_trait]` due to current async trait object limitations.

use {crate::error::CarbonResult, std::future::Future};

/// Async handler invoked for each decoded update of type `T`.
///
/// Registered via `PipelineBuilder` (`account`, `instruction`,
/// `transaction`, `account_deletions`, `block_details`). The pipeline
/// awaits `process` for every update that passes filters.
///
/// # Errors
///
/// Returning `Err` is logged and counted in metrics; processing continues.
/// Use for recoverable failures, not pipeline termination.
pub trait Processor<T>
where
    T: Sync,
{
    fn process(&mut self, data: &T) -> impl Future<Output = CarbonResult<()>> + Send;
}
