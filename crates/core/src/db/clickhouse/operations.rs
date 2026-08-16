//! Small operation traits used by generated ClickHouse rows.
//!
//! ClickHouse is append-oriented: an upsert is represented by inserting a new
//! version of a row and is normally resolved by the table's engine/query.

use crate::error::CarbonResult;

#[cfg(feature = "clickhouse-cluster")]
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Name of the ClickHouse cluster
    pub cluster_name: String,
    /// Shard name for ZooKeeper path (defaults to "{shard}" macro if None)
    pub shard_name: String,
}

#[cfg(feature = "clickhouse-cluster")]
impl MigrationConfig {
    pub fn new(cluster_name: String) -> Self {
        Self {
            cluster_name,
            shard_name: "{shared}".into(),
        }
    }

    pub fn with_shard_name(mut self, shard_name: String) -> Self {
        self.shard_name = shard_name;
        self
    }
}

#[cfg(feature = "clickhouse-cluster")]
pub trait ClusterTable {
    fn local_table() -> &'static str
    where
        Self: Sized;

    fn distributed_table() -> &'static str
    where
        Self: Sized;
}

#[cfg(not(feature = "clickhouse-cluster"))]
pub trait Table {
    fn table() -> &'static str
    where
        Self: Sized;
}

pub trait BatchInsert: Sized {
    type Row;

    fn batch_insert(&self, rows: &mut Vec<Self::Row>) -> CarbonResult<()>;
}

#[async_trait::async_trait]
pub trait BatchCommit: Sized {
    async fn batch_commit(client: &clickhouse::Client, rows: &[Self]) -> CarbonResult<()>;
}

#[cfg(feature = "clickhouse-cluster")]
#[async_trait::async_trait]
pub trait Insert: ClusterTable + Sized {
    async fn insert(client: &clickhouse::Client, rows: &[Self]) -> CarbonResult<()>;
}

#[cfg(not(feature = "clickhouse-cluster"))]
#[async_trait::async_trait]
pub trait Insert: Table + Sized {
    async fn insert(client: &clickhouse::Client, rows: &[Self]) -> CarbonResult<()>;
}

#[cfg(feature = "clickhouse-cluster")]
#[async_trait::async_trait]
pub trait Operation {
    async fn up(
        &self,
        client: &clickhouse::Client,
        config: &MigrationConfig,
    ) -> clickhouse::error::Result<()>;

    async fn down(
        &self,
        client: &clickhouse::Client,
        config: &MigrationConfig,
    ) -> clickhouse::error::Result<()>;
}

#[cfg(not(feature = "clickhouse-cluster"))]
#[async_trait::async_trait]
pub trait Operation {
    async fn up(&self, client: &clickhouse::Client) -> clickhouse::error::Result<()>;

    async fn down(&self, client: &clickhouse::Client) -> clickhouse::error::Result<()>;
}

pub trait Migration {
    fn app(&self) -> &str;

    fn name(&self) -> &str;

    fn operations(&self) -> Vec<Box<dyn Operation>>;
}
