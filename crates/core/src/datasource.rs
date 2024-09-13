use crate::error::CarbonResult;
use async_trait::async_trait;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

#[async_trait]
pub trait Datasource {
    async fn consume(
        &self,
        sender: &tokio::sync::mpsc::UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle>;

    fn update_types(&self) -> Vec<UpdateType>;
}

#[derive(Debug, Clone)]
pub enum Update {
    Account(AccountUpdate),
    Transaction(TransactionUpdate),
    Block(BlockUpdate),
    Slot(SlotUpdate),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateType {
    AccountUpdate,
    Transaction,
    Block,
    Slot,
}

#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub pubkey: Pubkey,
    pub account: solana_sdk::account::Account,
    pub slot: u64,
}

#[derive(Debug, Clone)]
pub struct TransactionUpdate {
    pub signature: Signature,
    pub transaction: solana_sdk::transaction::VersionedTransaction,
    pub meta: solana_transaction_status::TransactionStatusMeta,
    pub is_vote: bool,
    pub slot: u64,
}

#[derive(Debug, Clone)]
pub struct SlotUpdate {
    pub slot: u64,
    pub parent: Option<u64>,
    pub status: solana_sdk::commitment_config::CommitmentLevel,
}

#[derive(Debug, Clone)]
pub struct BlockUpdate {
    pub previous_blockhash: String,
    pub blockhash: String,
    pub parent_slot: solana_sdk::clock::Slot,
    pub transactions: Vec<solana_transaction_status::TransactionWithStatusMeta>,
    pub rewards: solana_transaction_status::Rewards,
    pub num_partitions: Option<u64>,
    pub block_time: Option<solana_sdk::clock::UnixTimestamp>,
    pub block_height: Option<u64>,
}
