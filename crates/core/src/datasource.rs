use solana_clock::Slot;
use solana_hash::Hash;
use solana_transaction_status::Rewards;
use {
    crate::error::CarbonResult,
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    solana_account::Account,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_status::TransactionStatusMeta,
    tokio_util::sync::CancellationToken,
};

#[derive(Debug, Clone)]
pub struct DatasourceDisconnection {
    pub source: String,
    pub disconnect_time: DateTime<Utc>,
    pub last_slot_before_disconnect: Slot,
    pub first_slot_after_reconnect: Slot,
    /// Number of slots missed during disconnection
    pub missed_slots: u64,
}

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

#[derive(Debug, Clone)]
pub enum Update {
    Account(AccountUpdate),
    Transaction(Box<TransactionUpdate>),
    AccountDeletion(AccountDeletion),
    BlockDetails(BlockDetails),
}

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

#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub pubkey: Pubkey,
    pub account: Account,
    pub slot: u64,
    pub transaction_signature: Option<Signature>,
}

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

#[derive(Debug, Clone)]
pub struct AccountDeletion {
    pub pubkey: Pubkey,
    pub slot: u64,
    pub transaction_signature: Option<Signature>,
}

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
