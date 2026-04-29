use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::CarbonResult,
        instruction::InstructionProcessorInputType,
        metrics::{Counter, MetricsRegistry},
        pipeline::Pipeline,
        processor::Processor,
    },
    carbon_jupiter_swap_decoder::{
        instructions::JupiterSwapInstruction, JupiterSwapDecoder,
        PROGRAM_ID as JUPITER_SWAP_PROGRAM_ID,
    },
    carbon_log_metrics::LogMetrics,
    solana_client::{
        nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
    },
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction_status::{TransactionStatusMeta, UiTransactionEncoding},
    std::{env, sync::Arc, time::Duration},
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

static FETCHED: Counter = Counter::new(
    "custom_http_poll_transactions_fetched_total",
    "Transactions fetched by the custom HTTP-poll datasource",
);
static FETCH_FAILURES: Counter = Counter::new(
    "custom_http_poll_fetch_failures_total",
    "Transient fetch failures retried by the custom HTTP-poll datasource",
);

pub struct HttpPollDatasource {
    pub rpc_url: String,
    pub program: Pubkey,
    pub poll_interval: Duration,
    pub batch_limit: usize,
}

#[async_trait]
impl Datasource for HttpPollDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        let registry = MetricsRegistry::global();
        registry.register_counter(&FETCHED);
        registry.register_counter(&FETCH_FAILURES);

        let rpc = Arc::new(RpcClient::new_with_commitment(
            self.rpc_url.clone(),
            CommitmentConfig::confirmed(),
        ));

        let mut last_seen: Option<Signature> = None;

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("custom datasource cancelled, exiting consume loop");
                return Ok(());
            }

            let sigs = match rpc
                .get_signatures_for_address_with_config(
                    &self.program,
                    GetConfirmedSignaturesForAddress2Config {
                        before: None,
                        until: last_seen,
                        limit: Some(self.batch_limit),
                        commitment: Some(CommitmentConfig::confirmed()),
                    },
                )
                .await
            {
                Ok(sigs) => sigs,
                Err(e) => {
                    FETCH_FAILURES.inc();
                    log::warn!("getSignaturesForAddress failed: {e}; backing off");
                    if cancel_aware_sleep(&cancellation_token, Duration::from_secs(5)).await {
                        return Ok(());
                    }
                    continue;
                }
            };

            if sigs.is_empty() {
                if cancel_aware_sleep(&cancellation_token, self.poll_interval).await {
                    return Ok(());
                }
                continue;
            }

            for sig_info in sigs.iter() {
                if cancellation_token.is_cancelled() {
                    return Ok(());
                }

                let signature: Signature = match sig_info.signature.parse() {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!("unparseable signature {}: {e}", sig_info.signature);
                        continue;
                    }
                };

                let tx = match rpc
                    .get_transaction_with_config(
                        &signature,
                        solana_client::rpc_config::RpcTransactionConfig {
                            encoding: Some(UiTransactionEncoding::Base64),
                            commitment: Some(CommitmentConfig::confirmed()),
                            max_supported_transaction_version: Some(0),
                        },
                    )
                    .await
                {
                    Ok(t) => t,
                    Err(e) => {
                        FETCH_FAILURES.inc();
                        log::warn!("getTransaction({signature}) failed: {e}");
                        continue;
                    }
                };

                let Some(decoded) = tx.transaction.transaction.decode() else {
                    continue;
                };

                let meta = TransactionStatusMeta {
                    status: Ok(()),
                    ..Default::default()
                };

                let update = Update::Transaction(Box::new(TransactionUpdate {
                    signature,
                    transaction: decoded,
                    meta,
                    is_vote: false,
                    slot: tx.slot,
                    index: None,
                    block_time: tx.block_time,
                    block_hash: None,
                }));

                if sender.send((update, id.clone())).await.is_err() {
                    log::info!("pipeline closed, stopping custom datasource");
                    return Ok(());
                }
                FETCHED.inc();
            }

            if let Some(top) = sigs.first() {
                last_seen = top.signature.parse().ok();
            }

            if cancel_aware_sleep(&cancellation_token, self.poll_interval).await {
                return Ok(());
            }
        }
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}

async fn cancel_aware_sleep(token: &CancellationToken, dur: Duration) -> bool {
    tokio::select! {
        _ = tokio::time::sleep(dur) => false,
        _ = token.cancelled() => true,
    }
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let datasource = HttpPollDatasource {
        rpc_url: env::var("RPC_URL").expect("RPC_URL must be set"),
        program: JUPITER_SWAP_PROGRAM_ID,
        poll_interval: Duration::from_secs(5),
        batch_limit: 100,
    };

    Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(JupiterSwapDecoder, JupiterSwapInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct JupiterSwapInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, JupiterSwapInstruction>>
    for JupiterSwapInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, JupiterSwapInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;
        let slot = input.metadata.transaction_metadata.slot;
        log::info!(
            "ix slot={slot} sig={signature} variant={:?}",
            std::mem::discriminant(input.decoded_instruction)
        );
        Ok(())
    }
}
