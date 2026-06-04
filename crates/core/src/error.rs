//! Crate-wide error type and `Result` alias used across the pipeline.
//!
//! # Categories
//!
//! - Structural data errors — missing transaction fields required for
//!   processing: `MissingFeePayer`, `MissingInnerInstructions`,
//!   `MissingAccountInTransaction`, `MissingInstructionData`.
//! - Datasource contract violations —
//!   `MissingUpdateTypeInDatasource(UpdateType)`.
//! - Runtime failures — channel or datasource execution issues:
//!   `FailedToReceiveUpdates(String)`, `FailedToConsumeDatasource(String)`.
//! - `Custom(String)` — catch-all for external or feature-specific errors (e.g.
//!   `postgres` / `sqlx` wrapping).

use {crate::datasource::UpdateType, thiserror::Error};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing update type in datasource")]
    MissingUpdateTypeInDatasource(UpdateType),
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
