use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use carbon_core::{
    datasource::{BlockDetails, Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
    metrics::MetricsCollection,
};
use futures_util::FutureExt;
use jetstreamer_firehose::firehose::{
    BlockData, EntryData, HandlerFn, OnErrorFn, RewardsData, Stats, StatsTracking, TransactionData,
};
use solana_transaction_status_client_types::Reward;
use tokio_util::sync::CancellationToken;

use crate::filter::JetstreamerFilter;
use crate::{filter::TransactionFilter, range::JetstreamerRange};

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
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
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
        let metrics_for_block = metrics.clone();
        let on_block_fn = move |_thread_id: usize, block: BlockData| {
            let sender = sender_for_block.clone();
            let id = id_for_block.clone();
            let metrics = metrics_for_block.clone();
            async move { JetstreamerDatasource::on_block(block, id, sender, metrics).await }.boxed()
        };

        let sender_for_transaction = sender.clone();
        let id_for_transaction = id.clone();
        let filter_for_transaction = self.filter.transaction_filters.clone();
        let metrics_for_transaction = metrics.clone();
        let on_transaction_fn = move |_thread_id: usize, transaction: TransactionData| {
            let sender = sender_for_transaction.clone();
            let id = id_for_transaction.clone();
            let transaction_filters = filter_for_transaction.clone();
            let metrics = metrics_for_transaction.clone();
            async move {
                JetstreamerDatasource::on_transaction(
                    transaction,
                    id,
                    sender,
                    transaction_filters,
                    metrics,
                )
                .await
            }
            .boxed()
        };

        let metrics_for_stats = metrics.clone();
        let on_stats_fn = move |_thread_id: usize, stats: Stats| {
            let metrics = metrics_for_stats.clone();
            async move { JetstreamerDatasource::on_stats(stats, metrics).await }.boxed()
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
        metrics: Arc<MetricsCollection>,
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

        metrics
            .increment_counter("jetstreamer_blocks_sent", 1)
            .await?;

        Ok(())
    }

    pub async fn on_transaction(
        transaction: TransactionData,
        id: DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
        transaction_filters: Vec<TransactionFilter>,
        metrics: Arc<MetricsCollection>,
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
                metrics
                    .increment_counter("jetstreamer_transactions_filtered_out", 1)
                    .await?;
                return Ok(());
            }
        }

        metrics
            .increment_counter("jetstreamer_transactions_filtered_in", 1)
            .await?;

        sender
            .send((
                Update::Transaction(Box::new(TransactionUpdate {
                    signature: transaction.signature,
                    transaction: transaction.transaction,
                    meta: transaction.transaction_status_meta,
                    is_vote: transaction.is_vote,
                    slot: transaction.slot,
                    block_time: None,
                    block_hash: None,
                })),
                id,
            ))
            .await?;

        metrics
            .increment_counter("jetstreamer_transactions_sent", 1)
            .await?;

        Ok(())
    }

    pub async fn on_stats(
        stats: Stats,
        metrics: Arc<MetricsCollection>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        metrics
            .update_gauge(
                "jetstreamer_internal_slots_processed",
                stats.slots_processed as f64,
            )
            .await?;
        metrics
            .update_gauge(
                "jetstreamer_internal_blocks_processed",
                stats.blocks_processed as f64,
            )
            .await?;
        metrics
            .update_gauge(
                "jetstreamer_internal_transactions_processed",
                stats.transactions_processed as f64,
            )
            .await?;
        Ok(())
    }
}
