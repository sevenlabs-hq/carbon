use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use carbon_core::{
    account::{AccountDecoder, AccountProcessorInputType, DecodedAccount},
    datasource::{AccountUpdate, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionDecoder, InstructionProcessorInputType},
    metrics::MetricsCollection,
    pipeline::{Pipeline, ShutdownStrategy},
    processor::Processor,
};
use carbon_log_metrics::LogMetrics;
use carbon_prometheus_metrics::PrometheusMetrics;
use clap::Parser;
use solana_account::Account;
use solana_message::{compiled_instruction::CompiledInstruction, Message, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;
use solana_transaction_status::TransactionStatusMeta;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional cap on how many updates the synthetic datasource sends before
    /// exiting.
    #[arg(long)]
    updates_per_producer: Option<u64>,

    /// Optional number of updates to produce per second. If set, the datasource
    /// will sleep between sends to approximate this rate.
    #[arg(long)]
    updates_per_second: Option<u64>,

    /// Choose processing mode: "account" or "instruction".
    #[arg(long, default_value = "instruction")]
    processing_mode: String,

    /// Size of the tokio::mpsc channel buffer in the pipeline.
    #[arg(long, default_value_t = carbon_core::pipeline::DEFAULT_CHANNEL_BUFFER_SIZE)]
    channel_buffer_size: usize,

    /// Enable Prometheus metrics in addition to LogMetrics.
    #[arg(long, default_value_t = false)]
    use_prometheus: bool,

    /// Prometheus exporter port (only used when --use-prometheus is set).
    #[arg(long, default_value_t = 9100)]
    prometheus_port: u16,

    /// When built with the `channel-experiments` feature, run an alternative
    /// benchmark path that bypasses the carbon-core pipeline and measures a
    /// direct channel + processor loop for comparison.
    #[cfg(feature = "channel-experiments")]
    #[arg(long, default_value_t = false)]
    direct_channel_benchmark: bool,
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let args = Args::parse();

    log::info!(
        "starting pipeline bench with processing_mode={}, updates_per_producer={:?}, updates_per_second={:?}, channel_buffer_size={}, use_prometheus={}, prometheus_port={}",
        args.processing_mode,
        args.updates_per_producer,
        args.updates_per_second,
        args.channel_buffer_size,
        args.use_prometheus,
        args.prometheus_port
    );

    #[cfg(feature = "channel-experiments")]
    if args.direct_channel_benchmark {
        run_direct_channel_benchmark(&args).await?;
        return Ok(());
    }

    let mut builder = Pipeline::builder().datasource(SyntheticDatasource {
        updates_per_producer: args.updates_per_producer,
        updates_per_second: args.updates_per_second,
        processing_mode: args.processing_mode.clone(),
    });

    builder = match args.processing_mode.as_str() {
        "account" => builder.account(SyntheticAccountDecoder, SyntheticAccountProcessor),
        "instruction" => {
            builder.instruction(SyntheticInstructionDecoder, SyntheticInstructionProcessor)
        }
        _ => {
            return Err(carbon_core::error::Error::Custom(format!(
                "unsupported processing mode: {}",
                args.processing_mode
            )));
        }
    }
    .metrics(Arc::new(LogMetrics::new()))
    .metrics_flush_interval(1)
    .channel_buffer_size(args.channel_buffer_size)
    .shutdown_strategy(ShutdownStrategy::Immediate);

    if args.use_prometheus {
        builder = builder.metrics(Arc::new(PrometheusMetrics::new_with_port(
            args.prometheus_port,
        )));
    }

    let mut pipeline = builder.build()?;
    pipeline.run().await
}

/// A simple baseline benchmark that bypasses the carbon-core pipeline and
/// measures how fast a direct tokio::mpsc channel plus a no-op processor can
/// move updates. This is only built when the `channel-experiments` feature is
/// enabled and must be explicitly selected via CLI.
#[cfg(feature = "channel-experiments")]
async fn run_direct_channel_benchmark(args: &Args) -> CarbonResult<()> {
    use tokio::sync::mpsc;

    log::info!(
        "running direct-channel benchmark with updates_per_producer={:?}, updates_per_second={:?}, channel_buffer_size={}",
        args.updates_per_producer,
        args.updates_per_second,
        args.channel_buffer_size
    );

    let (tx, mut rx) = mpsc::channel::<AccountUpdate>(args.channel_buffer_size);

    let cancellation_token = CancellationToken::new();
    let updates_per_producer = args.updates_per_producer;
    let updates_per_second = args.updates_per_second;

    let handle = tokio::spawn(async move {
        let mut sent: u64 = 0;
        let sleep_duration = updates_per_second.map(|ups| {
            if ups == 0 {
                Duration::from_secs(0)
            } else {
                Duration::from_nanos(1_000_000_000 / ups)
            }
        });

        loop {
            if cancellation_token.is_cancelled() {
                break;
            }

            if let Some(max_updates) = updates_per_producer {
                if sent >= max_updates {
                    break;
                }
            }

            let account_update = AccountUpdate {
                pubkey: Pubkey::new_unique(),
                account: Account {
                    lamports: 0,
                    data: Vec::new(),
                    owner: Pubkey::new_unique(),
                    executable: false,
                    rent_epoch: 0,
                },
                slot: 0,
                transaction_signature: None,
            };

            if tx.send(account_update).await.is_err() {
                break;
            }

            sent += 1;

            if let Some(dur) = sleep_duration {
                if !dur.is_zero() {
                    sleep(dur).await;
                }
            }
        }

        sent
    });

    let start = std::time::Instant::now();
    let mut received: u64 = 0;

    while let Some(_update) = rx.recv().await {
        received += 1;
    }

    let elapsed = start.elapsed();

    let mut produced: u64 = 0;
    if let Ok(count) = handle.await {
        produced += count;
    }

    log::info!(
        "direct-channel benchmark finished: produced={}, received={}, elapsed={:?}, approx_throughput={} updates/sec",
        produced,
        received,
        elapsed,
        if elapsed.as_secs_f64() > 0.0 {
            received as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        }
    );

    Ok(())
}

struct SyntheticDatasource {
    updates_per_producer: Option<u64>,
    updates_per_second: Option<u64>,
    processing_mode: String,
}

#[async_trait]
impl carbon_core::datasource::Datasource for SyntheticDatasource {
    async fn consume(
        &self,
        id: carbon_core::datasource::DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, carbon_core::datasource::DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let sender = sender.clone();
        let cancellation_token = cancellation_token.clone();
        let metrics = metrics.clone();
        let id = id.clone();
        let updates_per_producer = self.updates_per_producer;
        let updates_per_second = self.updates_per_second;
        let processing_mode = self.processing_mode.clone();

        let handle = tokio::spawn(async move {
            let mut sent: u64 = 0;
            let sleep_duration = updates_per_second.map(|ups| {
                if ups == 0 {
                    Duration::from_secs(0)
                } else {
                    Duration::from_nanos(1_000_000_000 / ups)
                }
            });

            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                if let Some(max_updates) = updates_per_producer {
                    if sent >= max_updates {
                        break;
                    }
                }

                let update = match processing_mode.as_str() {
                    "account" => {
                        let account_update = AccountUpdate {
                            pubkey: Pubkey::new_unique(),
                            account: Account {
                                lamports: 0,
                                data: Vec::new(),
                                owner: Pubkey::new_unique(),
                                executable: false,
                                rent_epoch: 0,
                            },
                            slot: 0,
                            transaction_signature: None,
                        };
                        Update::Account(account_update)
                    }
                    "instruction" | "transaction" => {
                        let signer = Pubkey::new_unique();
                        let signature = Signature::new_unique();

                        let num_instructions = 5 + (sent % 6) as usize;
                        let mut account_keys = vec![signer];
                        let mut compiled_instructions = Vec::with_capacity(num_instructions);

                        for ix_idx in 0..num_instructions {
                            let num_accounts = 5 + ((sent + ix_idx as u64) % 6) as usize;
                            let mut instruction_account_indices = Vec::with_capacity(num_accounts);

                            for _ in 0..num_accounts {
                                let account_key = Pubkey::new_unique();
                                let account_index = account_keys.len();
                                account_keys.push(account_key);
                                instruction_account_indices.push(account_index as u8);
                            }

                            let data_size = 10 + ((sent + ix_idx as u64) % 41) as usize;
                            let data: Vec<u8> = vec![0; data_size];

                            let program_id_index = 0;

                            compiled_instructions.push(CompiledInstruction {
                                program_id_index,
                                accounts: instruction_account_indices,
                                data,
                            });
                        }

                        let num_readonly_unsigned = (account_keys.len() - 1).max(0) as u8;
                        let message = Message::new_with_compiled_instructions(
                            1,
                            0,
                            num_readonly_unsigned,
                            account_keys.clone(),
                            Default::default(),
                            compiled_instructions,
                        );
                        let versioned_message = VersionedMessage::Legacy(message);

                        let transaction = VersionedTransaction {
                            signatures: vec![signature],
                            message: versioned_message,
                        };

                        let meta = TransactionStatusMeta::default();

                        let tx_update = TransactionUpdate {
                            signature,
                            transaction,
                            meta,
                            is_vote: false,
                            slot: 0,
                            block_time: None,
                            block_hash: None,
                        };
                        Update::Transaction(Box::new(tx_update))
                    }
                    _ => {
                        log::error!("unsupported processing mode: {processing_mode}");
                        break;
                    }
                };

                if sender.send((update, id.clone())).await.is_err() {
                    break;
                }

                let _ = metrics
                    .increment_counter("synthetic_updates_produced", 1)
                    .await;

                sent += 1;

                if let Some(dur) = sleep_duration {
                    if !dur.is_zero() {
                        sleep(dur).await;
                    }
                }
            }

            log::info!("synthetic producer finished after sending {sent} updates");
        });

        let _ = handle.await;

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        match self.processing_mode.as_str() {
            "account" => vec![UpdateType::AccountUpdate],
            "instruction" | "transaction" => vec![UpdateType::Transaction],
            _ => vec![],
        }
    }
}

struct SyntheticInstructionDecoder;

impl<'a> InstructionDecoder<'a> for SyntheticInstructionDecoder {
    type InstructionType = ();

    fn decode_instruction(
        &self,
        _instruction: &'a solana_instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        Some(DecodedInstruction {
            program_id: Pubkey::new_unique(),
            data: (),
            accounts: vec![],
        })
    }
}

#[derive(Default)]
struct SyntheticInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, ()>> for SyntheticInstructionProcessor {
    async fn process(
        &mut self,
        _data: &InstructionProcessorInputType<'_, ()>,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        Ok(())
    }
}

struct SyntheticAccountDecoder;

impl<'a> AccountDecoder<'a> for SyntheticAccountDecoder {
    type AccountType = ();

    fn decode_account(
        &self,
        _account: &'a solana_account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>> {
        Some(DecodedAccount {
            lamports: 0,
            data: (),
            owner: Pubkey::new_unique(),
            executable: false,
            rent_epoch: 0,
        })
    }
}

#[derive(Default)]
struct SyntheticAccountProcessor;

impl Processor<AccountProcessorInputType<'_, ()>> for SyntheticAccountProcessor {
    async fn process(
        &mut self,
        _data: &AccountProcessorInputType<'_, ()>,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        Ok(())
    }
}
