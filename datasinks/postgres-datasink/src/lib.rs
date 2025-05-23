use async_trait::async_trait;
use base64::Engine as _;
use carbon_core::datasink::{DataSink, PeriodicFlush};
use carbon_core::datasource::{AccountUpdate, TransactionUpdate};
use carbon_postgres_client::PgClient;
use serde_json;
use solana_transaction_status::UiTransactionStatusMeta;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

// Schema for raw data tables
/*
CREATE TABLE carbon_raw_account (
    id SERIAL PRIMARY KEY,
    pubkey BYTEA NOT NULL,
    owner BYTEA NOT NULL,
    lamports BIGINT NOT NULL,
    data BYTEA NOT NULL,
    executable BOOLEAN NOT NULL,
    rent_epoch BIGINT NOT NULL,
    slot BIGINT NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE carbon_raw_transaction (
    id SERIAL PRIMARY KEY,
    signature BYTEA NOT NULL,
    transaction TEXT NOT NULL,  -- Base64 encoded raw transaction
    meta TEXT NOT NULL,         -- JSON encoded transaction metadata
    is_vote BOOLEAN NOT NULL,
    slot BIGINT NOT NULL,
    block_time BIGINT,
    block_hash BYTEA,
    processed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
*/

pub struct PostgresWriter {
    client: Arc<PgClient>,
    account_buffer: Arc<Mutex<VecDeque<AccountUpdate>>>,
    transaction_buffer: Arc<Mutex<VecDeque<TransactionUpdate>>>,
    max_buffer_size: usize,
    flush_interval: Duration,
}

impl PostgresWriter {
    pub async fn new(
        connection_string: &str,
        max_buffer_size: usize,
        flush_interval_secs: u64,
    ) -> Result<Self, sqlx::Error> {
        let client = PgClient::new(connection_string, 1, 10).await?;

        Ok(PostgresWriter {
            client: Arc::new(client),
            account_buffer: Arc::new(Mutex::new(VecDeque::new())),
            transaction_buffer: Arc::new(Mutex::new(VecDeque::new())),
            max_buffer_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
        })
    }
}

#[async_trait]
impl DataSink for PostgresWriter {
    async fn capture_account(&self, account_update: AccountUpdate) {
        let mut buffer = self.account_buffer.lock().await;
        buffer.push_back(account_update);
        if buffer.len() >= self.max_buffer_size {
            self.flush_accounts().await;
        }
    }

    async fn capture_transaction(&self, transaction_update: TransactionUpdate) {
        let mut buffer = self.transaction_buffer.lock().await;
        buffer.push_back(transaction_update);
        if buffer.len() >= self.max_buffer_size {
            self.flush_transactions().await;
        }
    }

    async fn flush_accounts(&self) {
        let mut buffer = self.account_buffer.lock().await;
        if buffer.is_empty() {
            return;
        }

        let mut accounts = Vec::new();
        while let Some(update) = buffer.pop_front() {
            accounts.push((
                update.pubkey.to_bytes().to_vec(),
                update.account.owner.to_bytes().to_vec(),
                update.account.lamports as i64,
                update.account.data,
                update.account.executable,
                update.account.rent_epoch as i64,
                update.slot as i64,
            ));
        }

        if !accounts.is_empty() {
            let mut tx = self
                .client
                .pool
                .begin()
                .await
                .expect("Failed to begin transaction");

            for account in accounts {
                sqlx::query(
                    "INSERT INTO carbon_raw_account 
                    (pubkey, owner, lamports, data, executable, rent_epoch, slot) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7)",
                )
                .bind(account.0)
                .bind(account.1)
                .bind(account.2)
                .bind(account.3)
                .bind(account.4)
                .bind(account.5)
                .bind(account.6)
                .execute(&mut *tx)
                .await
                .expect("Failed to insert account");
            }

            tx.commit().await.expect("Failed to commit transaction");
        }
    }

    async fn flush_transactions(&self) {
        let mut buffer = self.transaction_buffer.lock().await;
        if buffer.is_empty() {
            return;
        }

        let mut transactions = Vec::new();
        while let Some(update) = buffer.pop_front() {
            // Convert transaction to base64 encoded string
            let tx_bytes = update.transaction.message.serialize();

            // Convert metadata to JSON string
            let meta_json =
                serde_json::to_string(&UiTransactionStatusMeta::from(update.meta.clone()))
                    .unwrap_or_default();

            transactions.push((
                update.signature.as_ref().to_vec(),
                base64::engine::general_purpose::STANDARD.encode(tx_bytes),
                meta_json,
                update.is_vote,
                update.slot as i64,
                update.block_time.map(|t| t as i64),
                update.block_hash.map(|h| h.to_bytes().to_vec()),
            ));
        }

        if !transactions.is_empty() {
            let mut tx = self
                .client
                .pool
                .begin()
                .await
                .expect("Failed to begin transaction");

            for tx_data in transactions {
                sqlx::query(
                    "INSERT INTO carbon_raw_transaction 
                    (signature, transaction, meta, is_vote, slot, block_time, block_hash) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7)",
                )
                .bind(tx_data.0)
                .bind(tx_data.1)
                .bind(tx_data.2)
                .bind(tx_data.3)
                .bind(tx_data.4)
                .bind(tx_data.5)
                .bind(tx_data.6)
                .execute(&mut *tx)
                .await
                .expect("Failed to insert transaction");
            }

            tx.commit().await.expect("Failed to commit transaction");
        }
    }
}

#[async_trait]
impl PeriodicFlush for PostgresWriter {
    async fn start_periodic_flush(&self) {
        let accounts_self = self.clone();
        let transactions_self = self.clone();

        // Spawn periodic flush for accounts
        tokio::spawn(async move {
            let mut interval = interval(accounts_self.flush_interval);
            loop {
                interval.tick().await;
                accounts_self.flush_accounts().await;
            }
        });

        // Spawn periodic flush for transactions
        tokio::spawn(async move {
            let mut interval = interval(transactions_self.flush_interval);
            loop {
                interval.tick().await;
                transactions_self.flush_transactions().await;
            }
        });
    }
}

// To allow cloning of PostgresWriter for periodic flush tasks
impl Clone for PostgresWriter {
    fn clone(&self) -> Self {
        PostgresWriter {
            client: Arc::clone(&self.client),
            account_buffer: Arc::clone(&self.account_buffer),
            transaction_buffer: Arc::clone(&self.transaction_buffer),
            max_buffer_size: self.max_buffer_size,
            flush_interval: self.flush_interval,
        }
    }
}
