//! Core building blocks for streaming-based Solana data indexing with Carbon.
//!
//! # Components
//!
//! - [`pipeline`] — orchestrates data flow from datasources to processors.
//! - [`datasource`] — ingestion layer defining `Datasource`.
//! - [`update`] — account, closure, transaction, and block updates.
//! - [`processor`] — user-defined handlers via `Processor<T>` trait.
//! - [`account`], [`instruction`], [`transaction`] — decoders and metadata.
//! - [`route`] — route execution, options, and processor inputs.
//! - [`filter`] — filters applied to routes (scoping, deduplication,
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
pub mod account_utils;
pub mod collection;
pub mod cursor;
pub mod datasource;
pub mod deserialize;
pub mod error;
pub mod filter;
pub mod id;
pub mod instruction;
pub mod metrics;
pub mod pipeline;
pub mod processor;
pub mod route;
pub mod transaction;
pub mod update;

#[cfg(feature = "yellowstone")]
pub mod yellowstone;

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
