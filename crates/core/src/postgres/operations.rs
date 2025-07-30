use crate::error::CarbonResult;

pub trait Table {
    fn table() -> &'static str;
    fn columns() -> Vec<&'static str>;
}

#[async_trait::async_trait]
pub trait Insert {
    async fn insert(&self, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

#[async_trait::async_trait]
pub trait Upsert {
    async fn upsert(&self, pool: &sqlx::PgPool) -> CarbonResult<()>;
}

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
