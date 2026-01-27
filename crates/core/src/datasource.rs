use solana_program::hash::Hash;
use solana_transaction_status::Rewards;
use {
    crate::{error::CarbonResult, metrics::MetricsCollection},
    async_trait::async_trait,
    solana_account::Account,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_status::TransactionStatusMeta,
    std::sync::Arc,
    tokio_util::sync::CancellationToken,
};

#[async_trait]
pub trait Datasource: Send + Sync {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
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
}

#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub pubkey: Pubkey,
    pub account: Account,
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

#[derive(Debug, Clone)]
pub struct AccountDeletion {
    pub pubkey: Pubkey,
    pub slot: u64,
    pub transaction_signature: Option<Signature>,
}

#[derive(Debug, Clone)]
pub struct TransactionUpdate {
    pub signature: Signature,
    pub transaction: VersionedTransaction, // TODO: replace with solana_transaction crate after 2.2.0 release
    pub meta: TransactionStatusMeta,
    pub is_vote: bool,
    pub slot: u64,
    pub index: Option<u64>,
    pub block_time: Option<i64>,
    pub block_hash: Option<Hash>,
}
