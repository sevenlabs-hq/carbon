use async_trait::async_trait;

use crate::datasource::{AccountUpdate, TransactionUpdate};

#[async_trait]
pub trait DataSink: Send + Sync {
    async fn capture_account(&self, account_update: AccountUpdate);
    async fn capture_transaction(&self, transaction_update: TransactionUpdate);
    async fn flush_accounts(&self);
    async fn flush_transactions(&self);
}

#[async_trait]
pub trait PeriodicFlush: Send + Sync {
    async fn start_periodic_flush(&self);
}
