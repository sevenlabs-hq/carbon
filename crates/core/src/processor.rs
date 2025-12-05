//! Defines the `Processor` trait for processing data within the `carbon-core`
//! pipeline.
//!
//! The `Processor` trait provides a standardized interface for handling various
//! types of data within the processing pipeline. It includes asynchronous
//! processing capabilities and supports metric tracking, enabling real-time
//! insights into the performance of individual processing stages.
//!
//! ## Key Concepts
//!
//! - **Processor**: A trait that defines a single method, `process`, which
//!   asynchronously handles data of a specified type. This allows different
//!   stages of the pipeline to implement custom data handling logic.
//! - **Metrics**: Metrics are collected during processing, offering visibility
//!   into processing duration, success, failure rates, and other relevant
//!   statistics.
//!
//! ## Usage
//!
//! Implement the `Processor` trait for any type that needs to handle specific
//! data in the pipeline. The trait allows you to define how data of a given
//! type is processed asynchronously, with the option to record metrics for
//! performance monitoring.
//!
//! ## Trait Definition
//!
//! ### Required Methods
//!
//! - `process`: Handles processing of the specified data type. This method is
//!   asynchronous and should be implemented to define how data should be
//!   processed in your specific use case.
//!
//! ## Parameters
//!
//! - `data`: An instance of the type specified by `InputType`. This represents
//!   the data to be processed.
//! - `metrics`: A vector of `Metrics` objects, allowing you to update and track
//!   various performance metrics.
//!
//! ## Returns
//!
//! The `process` method returns a `CarbonResult<()>`, which indicates either
//! successful processing (`Ok(())`) or an error.
//!
//! ## Notes
//!
//! - This trait uses `async_trait` to enable asynchronous processing. Ensure
//!   your runtime environment supports asynchronous execution, such as a Tokio
//!   runtime, to fully utilize this trait.
//! - When implementing the `process` method, consider which metrics are
//!   relevant to your data processing, and update those metrics accordingly to
//!   enable monitoring and alerting on key performance indicators.

use {
    crate::{error::CarbonResult, metrics::MetricsCollection},
    std::{future::Future, sync::Arc},
};

/// A trait for defining asynchronous data processing within the pipeline.
///
/// The `Processor` trait provides a single asynchronous method, `process`,
/// which is responsible for handling data of a specific type (`InputType`).
/// This trait is designed to be implemented by types that need to process data
/// within the pipeline, allowing for customized handling of different data
/// types.
///
/// # Type Parameters
///
/// - `T`: The type of data this processor handles. Data is passed by reference
///   to avoid cloning. Must be `Sync` to allow safe concurrent access.
///
/// # Required Methods
///
/// - `process`: Processes the specified data asynchronously, optionally updating
///   associated metrics. Returns an `impl Future` that is `Send`.
///
/// # Example
///
/// ```ignore
/// use carbon_core::error::CarbonResult;
/// use carbon_core::metrics::MetricsCollection;
/// use carbon_core::processor::Processor;
/// use std::sync::Arc;
///
/// struct CustomProcessor;
///
/// impl Processor<DataType> for CustomProcessor {
///     fn process(
///         &mut self,
///         data: &DataType,
///         metrics: Arc<MetricsCollection>,
///     ) -> impl std::future::Future<Output = CarbonResult<()>> + Send {
///         async move {
///             // Perform data processing logic
///             // Optionally, update metrics
///             metrics.increment_counter("processed_items", 1).await?;
///             Ok(())
///         }
///     }
/// }
/// ```
pub trait Processor<T>
where
    T: Sync,
{
    fn process(
        &mut self,
        data: &T,
        metrics: Arc<MetricsCollection>,
    ) -> impl Future<Output = CarbonResult<()>> + Send;
}
