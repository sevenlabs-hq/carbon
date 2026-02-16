use std::collections::HashSet;

use async_trait::async_trait;
use carbon_core::{
    datasource::{BlockDetails, Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
    metrics::{Counter, Gauge, MetricsRegistry},
};
use futures_util::FutureExt;
use jetstreamer_firehose::firehose::{
    BlockData, EntryData, HandlerFn, OnErrorFn, RewardsData, Stats, StatsTracking, TransactionData,
};
use solana_transaction_status_client_types::Reward;
use tokio_util::sync::CancellationToken;

use crate::filter::JetstreamerFilter;
use crate::{filter::TransactionFilter, range::JetstreamerRange};

static BLOCKS_SENT: Counter = Counter::new(
    "jetstreamer_blocks_sent_total",
    "Block details sent by Jetstreamer datasource",
);
static TRANSACTIONS_SENT: Counter = Counter::new(
    "jetstreamer_transactions_sent_total",
    "Transactions sent by Jetstreamer datasource",
);
static TRANSACTIONS_FILTERED_OUT: Counter = Counter::new(
    "jetstreamer_transactions_filtered_out_total",
    "Transactions filtered out by Jetstreamer datasource (did not match filters)",
);
static TRANSACTIONS_FILTERED_IN: Counter = Counter::new(
    "jetstreamer_transactions_filtered_in_total",
    "Transactions that passed filters (before send) in Jetstreamer datasource",
);
static INTERNAL_SLOTS_PROCESSED: Gauge = Gauge::new(
    "jetstreamer_internal_slots_processed",
    "Internal firehose slots processed (from Stats)",
);
static INTERNAL_BLOCKS_PROCESSED: Gauge = Gauge::new(
    "jetstreamer_internal_blocks_processed",
    "Internal firehose blocks processed (from Stats)",
);
static INTERNAL_TRANSACTIONS_PROCESSED: Gauge = Gauge::new(
    "jetstreamer_internal_transactions_processed",
    "Internal firehose transactions processed (from Stats)",
);

fn register_jetstreamer_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&BLOCKS_SENT);
    registry.register_counter(&TRANSACTIONS_SENT);
    registry.register_counter(&TRANSACTIONS_FILTERED_OUT);
    registry.register_counter(&TRANSACTIONS_FILTERED_IN);
    registry.register_gauge(&INTERNAL_SLOTS_PROCESSED);
    registry.register_gauge(&INTERNAL_BLOCKS_PROCESSED);
    registry.register_gauge(&INTERNAL_TRANSACTIONS_PROCESSED);
}

pub mod filter;
pub mod range;

pub struct JetstreamerDatasource {
    pub range: JetstreamerRange,
    pub filter: JetstreamerFilter,
    pub threads: u64,
    pub tracking_interval_slots: Option<u64>,
    pub archive_url: Option<String>,
    pub network: Option<String>,
}

impl JetstreamerDatasource {
    pub fn new(
        range: JetstreamerRange,
        filter: JetstreamerFilter,
        threads: u64,
        tracking_interval_slots: Option<u64>,
        archive_url: Option<String>,
        network: Option<String>,
    ) -> Self {
        Self {
            range,
            filter,
            threads,
            tracking_interval_slots,
            archive_url,
            network,
        }
    }

    pub fn new_with_old_faithful_mainnet(
        range: JetstreamerRange,
        filter: JetstreamerFilter,
        threads: u64,
        tracking_interval_slots: Option<u64>,
    ) -> Self {
        Self::new(range, filter, threads, tracking_interval_slots, None, None)
    }
}

#[async_trait]
impl Datasource for JetstreamerDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
        _cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        register_jetstreamer_metrics();
        let (start_slot, end_slot) = self.range.into_slots();
        let (include_transactions, include_blocks) =
            (self.filter.include_transactions, self.filter.include_blocks);

        if let Some(archive_url) = &self.archive_url {
            unsafe { std::env::set_var("JETSTREAMER_COMPACT_INDEX_BASE_URL", archive_url) }
        }

        if let Some(network) = &self.network {
            unsafe { std::env::set_var("JETSTREAMER_NETWORK", network) }
        }

        let sender_for_block = sender.clone();
        let id_for_block = id.clone();
        let on_block_fn = move |_thread_id: usize, block: BlockData| {
            let sender = sender_for_block.clone();
            let id = id_for_block.clone();
            async move { JetstreamerDatasource::on_block(block, id, sender).await }.boxed()
        };

        let sender_for_transaction = sender.clone();
        let id_for_transaction = id.clone();
        let filter_for_transaction = self.filter.transaction_filters.clone();
        let on_transaction_fn = move |_thread_id: usize, transaction: TransactionData| {
            let sender = sender_for_transaction.clone();
            let id = id_for_transaction.clone();
            let transaction_filters = filter_for_transaction.clone();
            async move {
                JetstreamerDatasource::on_transaction(transaction, id, sender, transaction_filters)
                    .await
            }
            .boxed()
        };

        let on_stats_fn = move |_thread_id: usize, stats: Stats| {
            async move { JetstreamerDatasource::on_stats(stats).await }.boxed()
        };

        let stats_tracking = self
            .tracking_interval_slots
            .map(|interval_slots| StatsTracking {
                on_stats: on_stats_fn,
                tracking_interval_slots: interval_slots,
            });

        jetstreamer_firehose::firehose::firehose(
            self.threads,
            start_slot..end_slot,
            if include_blocks {
                Some(on_block_fn)
            } else {
                None
            },
            if include_transactions {
                Some(on_transaction_fn)
            } else {
                None
            },
            None::<HandlerFn<EntryData>>,
            None::<HandlerFn<RewardsData>>,
            None::<OnErrorFn>,
            stats_tracking,
            None,
        )
        .await
        .map_err(|(error, _)| {
            carbon_core::error::Error::FailedToConsumeDatasource(error.to_string())
        })?;

        Ok(())
    }

    fn update_types(&self) -> Vec<carbon_core::datasource::UpdateType> {
        vec![UpdateType::Transaction]
    }
}

impl JetstreamerDatasource {
    pub async fn on_block(
        block: BlockData,
        id: DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let BlockData::Block {
            parent_blockhash,
            slot,
            blockhash,
            rewards,
            block_time,
            block_height,
            ..
        } = block
        else {
            return Ok(());
        };

        sender
            .send((
                Update::BlockDetails(BlockDetails {
                    slot,
                    block_hash: Some(blockhash),
                    previous_block_hash: Some(parent_blockhash),
                    rewards: Some(
                        rewards
                            .keyed_rewards
                            .iter()
                            .map(|(pubkey, reward)| Reward {
                                pubkey: pubkey.to_string(),
                                lamports: reward.lamports,
                                post_balance: reward.post_balance,
                                reward_type: Some(reward.reward_type),
                                commission: reward.commission,
                            })
                            .collect::<Vec<_>>(),
                    ),
                    num_reward_partitions: rewards.num_partitions,
                    block_time,
                    block_height,
                }),
                id,
            ))
            .await?;

        BLOCKS_SENT.inc();
        Ok(())
    }

    pub async fn on_transaction(
        transaction: TransactionData,
        id: DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
        transaction_filters: Vec<TransactionFilter>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !transaction_filters.is_empty() {
            let mut accounts = HashSet::new();
            accounts.extend(transaction.transaction.message.static_account_keys());
            accounts.extend(
                transaction
                    .transaction_status_meta
                    .loaded_addresses
                    .readonly
                    .iter(),
            );
            accounts.extend(
                transaction
                    .transaction_status_meta
                    .loaded_addresses
                    .writable
                    .iter(),
            );

            if transaction_filters.iter().all(|filter| {
                !filter.matches(
                    &accounts,
                    transaction.is_vote,
                    transaction.transaction_status_meta.status.is_err(),
                )
            }) {
                TRANSACTIONS_FILTERED_OUT.inc();
                return Ok(());
            }
        }

        TRANSACTIONS_FILTERED_IN.inc();

        sender
            .send((
                Update::Transaction(Box::new(TransactionUpdate {
                    signature: transaction.signature,
                    transaction: transaction.transaction,
                    meta: transaction.transaction_status_meta,
                    is_vote: transaction.is_vote,
                    slot: transaction.slot,
                    index: Some(transaction.transaction_slot_index as u64),
                    block_time: None,
                    block_hash: None,
                })),
                id,
            ))
            .await?;

        TRANSACTIONS_SENT.inc();
        Ok(())
    }

    pub async fn on_stats(stats: Stats) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        INTERNAL_SLOTS_PROCESSED.set(stats.slots_processed as f64);
        INTERNAL_BLOCKS_PROCESSED.set(stats.blocks_processed as f64);
        INTERNAL_TRANSACTIONS_PROCESSED.set(stats.transactions_processed as f64);
        Ok(())
    }
}
