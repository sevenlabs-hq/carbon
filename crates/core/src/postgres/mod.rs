//! Postgres feature: typed and JSON-blob processors backed by `sqlx`.
//!
//! # Components
//!
//! - [`processors`] — `Processor` impls (`PostgresAccountProcessor`,
//!   `PostgresJsonAccountProcessor`, and instruction analogues) that upsert
//!   into `accounts` / `instructions` tables.
//! - [`rows`] — `AccountRow<T>` / `InstructionRow<T>` with `Insert`, `Upsert`,
//!   `Delete`, `LookUp` impls.
//! - [`operations`] — CRUD trait family used by the row types.
//! - [`metadata`] — `AccountRowMetadata` / `InstructionRowMetadata` conversions
//!   from the pipeline's metadata types into row prefixes.
//! - [`primitives`] — sqlx `Encode`/`Decode` newtypes for `Pubkey`,
//!   `U8`/`U16`/`U32`/`U64`/`U128`, and `I128`.

pub mod metadata;
pub mod operations;
pub mod primitives;
pub mod processors;
pub mod rows;
