use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        metrics::MetricsCollection,
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
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    },
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

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
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let endpoint = self.0.clone();

        let mut client = ShredstreamProxyClient::connect(endpoint)
            .await
            .map_err(|err| carbon_core::error::Error::FailedToConsumeDatasource(err.to_string()))?;

        tokio::spawn(async move {
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
                    log::error!("Failed to subscribe: {:?}", e);
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
                    let metrics = metrics.clone();
                    let sender = sender.clone();
                    let dedup_cache = dedup_cache.clone();
                    let id_for_closure = id.clone();

                    async move {
                        let start_time = SystemTime::now();
                        let block_time =
                            Some(start_time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);

                        let entries: Vec<Entry> = match bincode::deserialize(&message.entries) {
                            Ok(e) => e,
                            Err(e) => {
                                log::error!("Failed to deserialize entries: {:?}", e);
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
                                    block_time,
                                    block_hash: None,
                                }));

                                if let Err(e) = sender.try_send((update, id_for_closure.clone())) {
                                    log::error!("Failed to send transaction update with signature {:?} at slot {}: {:?}", signature, message.slot, e);
                                    return Ok(());
                                }
                            }
                        }

                        metrics
                            .record_histogram(
                                "jito_shredstream_grpc_entry_process_time_nanoseconds",
                                start_time.elapsed().unwrap().as_nanos() as f64,
                            )
                            .await
                            .unwrap();

                        metrics
                            .increment_counter(
                                "jito_shredstream_grpc_entry_updates_received",
                                total_entries as u64,
                            )
                            .await
                            .unwrap_or_else(|value| {
                                log::error!("Error recording metric: {}", value)
                            });

                        metrics
                            .increment_counter(
                                "jito_shredstream_grpc_duplicate_entries",
                                duplicate_entries,
                            )
                            .await
                            .unwrap_or_else(|value| {
                                log::error!("Error recording metric: {}", value)
                            });

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
