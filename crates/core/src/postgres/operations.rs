//! CRUD trait family used by the `postgres` row types.

use crate::error::CarbonResult;

/// Static schema metadata: table name + column list.
pub trait Table {
    fn table() -> &'static str;
    fn columns() -> Vec<&'static str>;
}

/// Inserts the row, failing on primary-key conflict.
#[async_trait::async_trait]
pub trait Insert {
    async fn insert(&self, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

/// Inserts or updates the row by primary key (`ON CONFLICT DO UPDATE`).
/// Used by `PostgresJsonAccountProcessor` and friends on every update.
#[async_trait::async_trait]
pub trait Upsert {
    async fn upsert(&self, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

/// Deletes the row matching `Key`.
#[async_trait::async_trait]
pub trait Delete {
    type Key;

    async fn delete(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

/// Selects the row matching `Key`, returning `Ok(None)` when absent.
#[async_trait::async_trait]
pub trait LookUp: Sized {
    type Key;

    async fn lookup(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<Option<Self>>;
}
