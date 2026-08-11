//! ClickHouse support for generated, append-oriented decoder rows.

pub mod metadata;
pub mod operations;

#[cfg(not(feature = "clickhouse-cluster"))]
pub use operations::Table;
#[cfg(feature = "clickhouse-cluster")]
pub use operations::{ClusterTable, MigrationConfig};
pub use {
    metadata::{AccountRowMetadata, InstructionRowMetadata},
    operations::{BatchCommit, BatchInsert, Insert, Migration, Operation},
};
