//! Crate-wide error type and `Result` alias used across the pipeline.
//!
//! # Categories
//!
//! - Structural data errors — missing transaction fields required for
//!   processing: `MissingFeePayer`, `MissingInnerInstructions`,
//!   `MissingAccountInTransaction`, `MissingInstructionData`.
//! - Runtime failures — channel or datasource execution issues:
//!   `FailedToReceiveUpdates(String)`, `FailedToConsumeDatasource(String)`.
//! - `Custom(String)` — catch-all for external or feature-specific errors (e.g.
//!   `postgres` / `sqlx` wrapping).

use thiserror::Error;

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Transform(#[from] crate::instruction::TransformError),
    #[error("decoding failed: {0}")]
    Decode(#[source] BoxError),
    #[error("filter failed: {0}")]
    Filter(#[source] BoxError),
    #[error("filter commit failed: {0}")]
    FilterCommit(#[source] BoxError),
    #[error("processor failed: {0}")]
    Processor(#[source] BoxError),
    #[error("invalid datasource queue capacity: {0}")]
    InvalidQueueCapacity(usize),
    #[error("Failed to receive updates({0})")]
    FailedToReceiveUpdates(String),
    #[error("Transaction missing fee payer")]
    MissingFeePayer,
    #[error("Missing inner instructions")]
    MissingInnerInstructions,
    #[error("Missing account in transaction")]
    MissingAccountInTransaction,
    #[error("Missing instruction data")]
    MissingInstructionData,
    #[error("Failed to consume datasource ({0})")]
    FailedToConsumeDatasource(String),
    #[error("Custom error: {0}")]
    Custom(String),
}

pub type CarbonResult<T> = Result<T, Error>;
