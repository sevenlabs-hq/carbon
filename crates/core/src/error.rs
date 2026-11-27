//! Crate-wide error type and `Result` alias used across the pipeline.
//!
//! # Categories
//!
//! - Structural data errors — missing transaction fields required for
//!   processing: `MissingFeePayer`, `MissingAccountInTransaction`.
//! - Runtime failures — datasource execution issues:
//!   `FailedToConsumeDatasource(String)`.
//! - `Custom(String)` — catch-all for external or feature-specific errors (e.g.
//!   `postgres` / `sqlx` wrapping).

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Transaction missing fee payer")]
    MissingFeePayer,
    #[error("Missing account in transaction")]
    MissingAccountInTransaction,
    #[error("Failed to consume datasource ({0})")]
    FailedToConsumeDatasource(String),
    #[error("Custom error: {0}")]
    Custom(String),
}

pub type CarbonResult<T> = Result<T, Error>;
