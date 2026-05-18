//! Ingestion layer of the pipeline: defines upstream data sources and the
//! normalized update types they emit.
//!
//! # Components
//!
//! - [`Datasource`] — async producer that streams `(Update, DatasourceId)` into
//!   the pipeline.
//! - [`DatasourceId`] — identity used for routing, filtering, and metrics.
//! - [`Update`] — unified payload consumed by downstream pipeline stages.
//! - [`UpdateType`] — declared set of update variants a datasource may emit.
//!
//! # Flow
//!
//! Each datasource runs in a dedicated Tokio task spawned by `Pipeline::run`.
//! It emits `(Update, DatasourceId)` pairs through an MPSC channel and
//! terminates when the `CancellationToken` is triggered or the channel is
//! closed.

use {
    crate::error::CarbonResult,
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    solana_account::Account,
    solana_clock::Slot,
    solana_hash::Hash,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_status::{Rewards, TransactionStatusMeta},
    tokio_util::sync::CancellationToken,
};

/// Metadata describing a datasource disconnection event.
///
/// Used for observability and gap detection in streaming sources.
#[derive(Debug, Clone)]
pub struct DatasourceDisconnection {
    pub source: String,
    pub disconnect_time: DateTime<Utc>,
    pub last_slot_before_disconnect: Slot,
    pub first_slot_after_reconnect: Slot,
    /// Number of slots missed during downtime.
    pub missed_slots: u64,
}

/// Async producer trait implemented by all upstream data sources.
///
/// Runs as a dedicated task and streams `(Update, DatasourceId)` into the
/// pipeline. Implementations must respect the provided `CancellationToken` and
/// exit on shutdown. `update_types` declares which `Update` variants may be
/// emitted.
#[async_trait]
pub trait Datasource: Send + Sync {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()>;

    fn update_types(&self) -> Vec<UpdateType>;
}

/// Unique identifier for a datasource instance.
///
/// Used for filtering, routing, and per-source metrics aggregation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatasourceId(String);

impl DatasourceId {
    pub fn new_unique() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn new_named(name: &str) -> Self {
        Self(name.to_string())
    }
}

/// Unified payload emitted by datasources into the pipeline.
#[derive(Debug, Clone)]
pub enum Update {
    Account(AccountUpdate),
    Transaction(Box<TransactionUpdate>),
    AccountDeletion(AccountDeletion),
    BlockDetails(BlockDetails),
}

/// Declared set of update variants a datasource may emit.
///
/// Used by the pipeline to validate that emitted updates match expectations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateType {
    AccountUpdate,
    Transaction,
    AccountDeletion,
    BlockDetails,
}

impl Update {
    pub fn update_type(&self) -> UpdateType {
        match self {
            Update::Account(_) => UpdateType::AccountUpdate,
            Update::Transaction(_) => UpdateType::Transaction,
            Update::AccountDeletion(_) => UpdateType::AccountDeletion,
            Update::BlockDetails(_) => UpdateType::BlockDetails,
        }
    }
}

/// Account state update emitted by streaming or snapshot sources.
#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub pubkey: Pubkey,
    pub account: Account,
    pub slot: u64,
    pub transaction_signature: Option<Signature>,
}

/// Full transaction payload with execution metadata.
#[derive(Debug, Clone)]
pub struct TransactionUpdate {
    pub signature: Signature,
    pub transaction: VersionedTransaction,
    pub meta: TransactionStatusMeta,
    pub is_vote: bool,
    pub slot: u64,
    pub index: Option<u64>,
    pub block_time: Option<i64>,
    pub block_hash: Option<Hash>,
}

/// Account closure event (lamports drained to zero).
#[derive(Debug, Clone)]
pub struct AccountDeletion {
    pub pubkey: Pubkey,
    pub slot: u64,
    pub transaction_signature: Option<Signature>,
}

/// Block-level metadata emitted by block-aware datasources.
#[derive(Debug, Clone)]
pub struct BlockDetails {
    pub slot: u64,
    pub block_hash: Option<Hash>,
    pub previous_block_hash: Option<Hash>,
    pub rewards: Option<Rewards>,
    pub num_reward_partitions: Option<u64>,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
}
