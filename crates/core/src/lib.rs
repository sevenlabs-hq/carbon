//! Core building blocks for streaming-based Solana data indexing with Carbon.
//!
//! # Components
//!
//! - [`pipeline`] — orchestrates data flow from datasources to processors.
//! - [`datasource`] — ingestion layer defining `Datasource` and `Update` types.
//! - [`processor`] — user-defined handlers via `Processor<T>` trait.
//! - [`account`], [`instruction`], [`transaction`], [`account_deletion`],
//!   [`block_details`] — pipeline stages and update-specific models.
//! - [`filter`] — routing layer applied to all pipes (scoping, deduplication,
//!   slot constraints).
//! - [`metrics`] — atomic observability primitives and global registry.
//! - [`collection`] — multi-decoder routing for transaction instruction sets.
//! - [`deserialize`] — shared Borsh utilities for decoder crates.
//!
//! # Optional features
//!
//! - `postgres` — SQL-backed processors using `sqlx`.
//! - `graphql` — GraphQL schema + Axum integration.

pub mod account;
pub mod account_deletion;
pub mod account_utils;
pub mod block_details;
pub mod collection;
pub mod datasource;
pub mod deserialize;
pub mod error;
pub mod filter;
pub mod instruction;
pub mod metrics;
pub mod pipeline;
pub mod processor;
pub mod transaction;
pub mod transformers;

#[cfg(feature = "graphql")]
pub mod graphql;
#[cfg(feature = "postgres")]
pub mod postgres;

pub use borsh;
#[cfg(feature = "macros")]
pub use carbon_macros::*;
#[cfg(feature = "macros")]
pub use carbon_proc_macros::*;
#[cfg(feature = "macros")]
#[doc(hidden)]
pub use log;
