use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::{Counter, Histogram, MetricsRegistry},
    },
    carbon_jito_protos::shredstream::{
        shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
    },
    futures::{stream::try_unfold, TryStreamExt},
    scc::HashCache,
    solana_client::rpc_client::SerializableTransaction,
    solana_entry::entry::Entry,
    solana_transaction_status::TransactionStatusMeta,
    std::{
        sync::{Arc, LazyLock},
        time::{Instant, SystemTime, UNIX_EPOCH},
    },
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

static ENTRY_PROCESS_TIME_NANOS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "jito_shredstream_grpc_entry_process_time_nanoseconds",
        "Time to process entry in nanoseconds",
        vec![
            1_000.0,
            10_000.0,
            100_000.0,
            1_000_000.0,
            10_000_000.0,
            100_000_000.0,
            1_000_000_000.0,
        ],
    )
});
static ENTRY_UPDATES_RECEIVED: Counter = Counter::new(
    "jito_shredstream_grpc_entry_updates_received_total",
    "Entry updates received from Jito Shredstream gRPC",
);
static DUPLICATE_ENTRIES: Counter = Counter::new(
    "jito_shredstream_grpc_duplicate_entries_total",
    "Duplicate entries skipped in Jito Shredstream gRPC",
);

fn register_jito_shredstream_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&ENTRY_UPDATES_RECEIVED);
    registry.register_counter(&DUPLICATE_ENTRIES);
    registry.register_histogram(&ENTRY_PROCESS_TIME_NANOS);
}

#[derive(Debug)]
pub struct JitoShredstreamGrpcClient(String);

impl JitoShredstreamGrpcClient {
    pub fn new(endpoint: String) -> Self {
        JitoShredstreamGrpcClient(endpoint)
    }
}

#[async_trait]
impl Datasource for JitoShredstreamGrpcClient {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        exporters: Vec<Arc<dyn carbon_core::metrics::MetricsExporter>>,
        flush_interval_secs: Option<u64>,
    ) -> CarbonResult<()> {
        register_jito_shredstream_metrics();
        let endpoint = self.0.clone();
        let exporters_for_flush = exporters;
        let flush_interval = flush_interval_secs;

        let mut client = ShredstreamProxyClient::connect(endpoint)
            .await
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?;

        tokio::spawn(async move {
            if let (Some(interval), true) = (flush_interval, !exporters_for_flush.is_empty()) {
                carbon_core::pipeline::spawn_metrics_flush_task(
                    exporters_for_flush,
                    interval,
                    cancellation_token.clone(),
                );
            }

            let result = tokio::select! {
                _ = cancellation_token.cancelled() => {
                    log::info!("Cancelling Jito Shreadstream gRPC subscription.");
                    return;
                }

                result = client.subscribe_entries(SubscribeEntriesRequest {}) =>
                    result
            };

            let stream = match result {
                Ok(r) => r.into_inner(),
                Err(e) => {
                    log::error!("Failed to subscribe: {e:?}");
                    return;
                }
            };

            let stream = try_unfold(
                (stream, cancellation_token),
                |(mut stream, cancellation_token)| async move {
                    tokio::select! {
                        _ = cancellation_token.cancelled() => {
                            log::info!("Cancelling Jito Shreadstream gRPC subscription.");
                            Ok(None)
                        },
                        v = stream.message() => match v {
                            Ok(Some(v)) => Ok(Some((v, (stream, cancellation_token)))),
                            Ok(None) => Ok(None),
                            Err(e) => Err(e),
                        },
                    }
                },
            );

            let dedup_cache = Arc::new(HashCache::with_capacity(1024, 4096));

            if let Err(e) = stream
                .try_for_each_concurrent(None, |message| {
                    let sender = sender.clone();
                    let dedup_cache = dedup_cache.clone();
                    let id_for_closure = id.clone();

                    async move {
                        let start_time = Instant::now();
                        let recv_time = SystemTime::now();
                        let block_time =
                            Some(recv_time.duration_since(UNIX_EPOCH).expect("Time").as_millis() as i64);

                        let entries: Vec<Entry> = match bincode::deserialize(&message.entries) {
                            Ok(e) => e,
                            Err(e) => {
                                log::error!("Failed to deserialize entries at slot {}: {e:?}", message.slot);
                                return Ok(());
                            }
                        };

                        let total_entries = entries.len();
                        let mut duplicate_entries = 0;

                        for entry in entries {
                            if dedup_cache.contains(&entry.hash) {
                                duplicate_entries += 1;
                                continue;
                            }
                            let _ = dedup_cache.put(entry.hash, ());

                            for transaction in entry.transactions {
                                let signature = *transaction.get_signature();

                                let update = Update::Transaction(Box::new(TransactionUpdate {
                                    signature,
                                    is_vote: false,
                                    transaction,
                                    meta: TransactionStatusMeta {
                                        status: Ok(()),
                                        ..Default::default()
                                    },
                                    slot: message.slot,
                                    index: None,
                                    block_time,
                                    block_hash: None,
                                }));

                                if let Err(e) = sender.try_send((update, id_for_closure.clone())) {
                                    log::error!("Failed to send transaction update with signature {:?} at slot {}: {:?}", signature, message.slot, e);
                                    return Ok(());
                                }
                            }
                        }

                        ENTRY_PROCESS_TIME_NANOS.record(start_time.elapsed().as_nanos() as f64);
                        ENTRY_UPDATES_RECEIVED.inc_by(total_entries as u64);
                        DUPLICATE_ENTRIES.inc_by(duplicate_entries);

                        Ok(())
                    }
                })
                .await
            {
                log::error!("Grpc stream error: {e:?}");
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}
