use std::sync::Arc;

use async_trait::async_trait;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use crate::{error::CarbonResult, metrics::DatasourceMetrics};

#[async_trait]
pub trait Datasource {
    async fn consume(
        &self,
        sender: &tokio::sync::mpsc::UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle>;

    fn update_types(&self) -> Vec<UpdateType>;
    fn name(&self) -> String;
    fn get_metrics(&self) -> DatasourceMetrics;
}

#[derive(Debug, Clone)]
pub enum Update {
    Account(AccountUpdate),
    Transaction(TransactionUpdate),
    AccountDeletion(AccountDeletion),
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
    pub account: solana_sdk::account::Account,
    pub slot: u64,
}

#[derive(Debug, Clone)]
pub struct AccountDeletion {
    pub pubkey: Pubkey,
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
