use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
    transformers::transaction_metadata_from_original_meta,
};
use futures::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
};
use solana_sdk::signature::Signature;
use std::str::FromStr;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Filters {
    pub block_filter: RpcBlockSubscribeFilter,
    pub block_subscribe_config: Option<RpcBlockSubscribeConfig>,
}

impl Filters {
    pub fn new(
        block_filter: RpcBlockSubscribeFilter,
        block_subscribe_config: Option<RpcBlockSubscribeConfig>,
    ) -> CarbonResult<Self> {
        Ok(Filters {
            block_filter,
            block_subscribe_config,
        })
    }
}

pub struct RpcBlockSubscribe {
    pub rpc_url: String,
    pub filters: Filters,
}

impl RpcBlockSubscribe {
    pub fn new(rpc_url: String, filters: Filters) -> Self {
        Self { rpc_url, filters }
    }
}

#[async_trait]
impl Datasource for RpcBlockSubscribe {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        let client = PubsubClient::new(&self.rpc_url).await.map_err(|err| {
            carbon_core::error::Error::Custom(format!(
                "Failed to create an RPC subscribe client: {err}"
            ))
        })?;

        let filters = self.filters.clone();
        let sender = sender.clone();

        tokio::spawn(async move {
            let sender_clone = sender.clone();
            let (mut stream, _unsub) = match client
                .block_subscribe(filters.block_filter, filters.block_subscribe_config)
                .await
            {
                Ok(subscription) => subscription,
                Err(err) => {
                    log::error!("Failed to subscribe to blocks updates: {:?}", err);
                    return;
                }
            };

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        log::info!("Cancelling RPC blocks subscription...");
                        break;
                    }
                    event_result = stream.next() => {
                        match event_result {
                            Some(tx_event) => {
                                let slot = tx_event.context.slot;

                                if let Some(block) = tx_event.value.block {
                                    if let (Some(transactions), Some(signatures)) = (block.transactions, block.signatures) {
                                        for (encoded_transaction_with_status_meta, signature_str) in transactions.into_iter().zip(signatures.into_iter()) {
                                            let Ok(signature) = Signature::from_str(&signature_str) else {
                                                log::error!("Error getting Signature from string");
                                                continue;
                                            };

                                            let meta_original = if let Some(meta) = encoded_transaction_with_status_meta.clone().meta {
                                                meta
                                            } else {
                                                log::warn!("Meta is malformed for transaction: {:?}", signature_str);
                                                continue;
                                            };

                                            if meta_original.status.is_err() {
                                                log::warn!(
                                                    "Transaction failed or encountered an error: {:?} (signature: {:?})",
                                                    meta_original.status,
                                                    signature_str
                                                );
                                                continue;
                                            }

                                            let Some(decoded_transaction) = encoded_transaction_with_status_meta.transaction.decode() else {
                                                log::error!("Failed to decode transaction: {:?}", encoded_transaction_with_status_meta);
                                                continue;
                                            };

                                            let Ok(meta_needed) = transaction_metadata_from_original_meta(meta_original) else {
                                                log::error!("Error getting metadata from transaction original meta.");
                                                continue;
                                            };

                                            let update = Update::Transaction(TransactionUpdate {
                                                signature,
                                                transaction: decoded_transaction.clone(),
                                                meta: meta_needed,
                                                is_vote: false,
                                                slot,
                                            });

                                            if let Err(err) = sender_clone.send(update) {
                                                log::error!("Error sending transaction update: {:?}", err);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            None => {
                                log::info!("Blocks stream has been closed");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}
