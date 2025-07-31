//! Provides support for PostgreSQL database operations, including table
//! definitions, insert, upsert, and delete operations. This module is designed
//! to be used in conjunction with the `sqlx` crate for database interactions.
//!
//! # Overview
//!
//! This module provides the `Table`, `Insert`, `Upsert`, and `Delete` traits,
//! which are used to define and interact with PostgreSQL tables.
//!

use crate::error::CarbonResult;

/// A trait for defining PostgreSQL tables.
pub trait Table {
    fn table() -> &'static str;
    fn columns() -> Vec<&'static str>;
}

/// A trait for defining PostgreSQL insert operations.
#[async_trait::async_trait]
pub trait Insert {
    async fn insert(&self, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

/// A trait for defining PostgreSQL upsert operations.
#[async_trait::async_trait]
pub trait Upsert {
    async fn upsert(&self, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

/// A trait for defining PostgreSQL delete operations.
#[async_trait::async_trait]
pub trait Delete {
    type Key;

    async fn delete(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

#[async_trait::async_trait]
pub trait LookUp: Sized {
    type Key;

    async fn lookup(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<Option<Self>>;
}
