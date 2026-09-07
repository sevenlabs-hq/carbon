//! Datasource contracts.
//!
//! # Components
//!
//! - [`Datasource`] — async producer that streams `(Update, Id)` into
//!   the pipeline.
//! - [`Id`] — identity used for routing, filtering, and metrics.
//! - [`Update`] — unified payload consumed by downstream pipeline stages.
//! - [`UpdateType`] — declared set of update variants a datasource may emit.
//!
//! # Flow
//!
//! Each datasource runs in a dedicated Tokio task spawned by `Pipeline::run`.
//! It emits `(Update, Id)` pairs through an MPSC channel and
//! terminates when the `CancellationToken` is triggered or the channel is
//! closed.

pub mod receipt;

use {
    crate::{error::CarbonResult, id::Id, update::Update},
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    solana_clock::Slot,
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
/// Runs as a dedicated task and streams `(Update, Id)` into the
/// pipeline. Implementations must respect the provided `CancellationToken` and
/// exit on shutdown. `update_types` declares which `Update` variants may be
/// emitted.
#[async_trait]
pub trait Datasource: Send + Sync {
    async fn consume(
        &self,
        id: Id,
        sender: tokio::sync::mpsc::Sender<(Update, Id)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()>;

    fn update_types(&self) -> Vec<UpdateType>;
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
            Update::AccountClosure(_) => UpdateType::AccountDeletion,
            Update::Block(_) => UpdateType::BlockDetails,
        }
    }
}
