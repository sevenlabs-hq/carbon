use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_helius_laserstream_datasource::{LaserStreamClientConfig, LaserStreamGeyserClient},
    carbon_log_metrics::LogMetrics,
    carbon_pump_swap_decoder::{
        instructions::{CpiEvent, PumpSwapInstruction},
        PumpSwapDecoder, PROGRAM_ID as PUMPSWAP_PROGRAM_ID,
    },
    solana_native_token::LAMPORTS_PER_SOL,
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
        time::Duration,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "pumpswap_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![PUMPSWAP_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![PUMPSWAP_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert(
        "pumpswap_transaction_filter".to_string(),
        transaction_filter,
    );

    let laserstream_config = LaserStreamClientConfig::new(
        None,                          // compression
        Some(Duration::from_secs(15)), // connect_timeout
        Some(Duration::from_secs(15)), // timeout
        None,                          // max_decoding_message_size
        None,                          // tls_config
        None,                          // tcp_nodelay
        true,                          // replay_enabled - This is the key LaserStream feature!
    );

    let laserstream_datasource = LaserStreamGeyserClient::new(
        env::var("LASERSTREAM_URL").unwrap_or_default(),
        env::var("API_KEY").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        laserstream_config,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(laserstream_datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(PumpSwapDecoder, PumpSwapInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpSwapInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, PumpSwapInstruction>>
    for PumpSwapInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, PumpSwapInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        match input.decoded_instruction {
            PumpSwapInstruction::Buy { data, .. } => {
                log::info!("Buy: signature: {signature}, buy: {data:?}");
            }
            PumpSwapInstruction::Sell { data, .. } => {
                log::info!("Sell: signature: {signature}, sell: {data:?}");
            }
            PumpSwapInstruction::CreatePool { data, .. } => {
                log::info!("CreatePool: signature: {signature}, create_pool: {data:?}");
            }
            PumpSwapInstruction::Deposit { data, .. } => {
                log::info!("Deposit: signature: {signature}, deposit: {data:?}");
            }
            PumpSwapInstruction::Withdraw { data, .. } => {
                log::info!("Withdraw: signature: {signature}, withdraw: {data:?}");
            }
            PumpSwapInstruction::CreateConfig { data, .. } => {
                log::info!("CreateConfig: signature: {signature}, create_config: {data:?}");
            }
            PumpSwapInstruction::UpdateFeeConfig { data, .. } => {
                log::info!("UpdateFeeConfig: signature: {signature}, update_fee_config: {data:?}");
            }
            PumpSwapInstruction::UpdateAdmin { data, .. } => {
                log::info!("UpdateAdmin: signature: {signature}, update_admin: {data:?}");
            }
            PumpSwapInstruction::CollectCoinCreatorFee { data, .. } => {
                log::info!("CollectCoinCreatorFee: signature: {signature}, collect_fee: {data:?}");
            }
            PumpSwapInstruction::CpiEvent { data, .. } => match data {
                CpiEvent::BuyEvent(buy_event) => {
                    let sol_amount = buy_event.quote_amount_in as f64 / LAMPORTS_PER_SOL as f64;
                    log::info!(
                        "BuyEvent: signature: {signature}, SOL: {sol_amount:.4}, pool: {}, user: {}",
                        buy_event.pool,
                        buy_event.user,
                    );
                }
                CpiEvent::SellEvent(sell_event) => {
                    let sol_amount = sell_event.quote_amount_out as f64 / LAMPORTS_PER_SOL as f64;
                    log::info!(
                        "SellEvent: signature: {signature}, SOL: {sol_amount:.4}, pool: {}, user: {}",
                        sell_event.pool,
                        sell_event.user,
                    );
                }
                CpiEvent::CreatePoolEvent(pool_event) => {
                    log::info!(
                        "CreatePoolEvent: signature: {signature}, pool_event: {pool_event:?}"
                    );
                }
                CpiEvent::DepositEvent(deposit_event) => {
                    log::info!(
                        "DepositEvent: signature: {signature}, deposit_event: {deposit_event:?}"
                    );
                }
                CpiEvent::WithdrawEvent(withdraw_event) => {
                    log::info!(
                        "WithdrawEvent: signature: {signature}, withdraw_event: {withdraw_event:?}"
                    );
                }
                CpiEvent::CreateConfigEvent(config_event) => {
                    log::info!(
                        "CreateConfigEvent: signature: {signature}, config_event: {config_event:?}"
                    );
                }
                CpiEvent::UpdateFeeConfigEvent(fee_config_event) => {
                    log::info!(
                        "UpdateFeeConfigEvent: signature: {signature}, fee_config_event: {fee_config_event:?}"
                    );
                }
                CpiEvent::UpdateAdminEvent(admin_event) => {
                    log::info!(
                        "UpdateAdminEvent: signature: {signature}, admin_event: {admin_event:?}"
                    );
                }
                CpiEvent::CollectCoinCreatorFeeEvent(fee_event) => {
                    let fee_amount = fee_event.coin_creator_fee as f64 / LAMPORTS_PER_SOL as f64;
                    log::info!(
                        "CollectCoinCreatorFeeEvent: signature: {signature}, fee: {fee_amount:.6} SOL, creator: {}",
                        fee_event.coin_creator
                    );
                }
                CpiEvent::ClaimTokenIncentivesEvent(incentive_event) => {
                    log::info!(
                        "ClaimTokenIncentivesEvent: signature: {signature}, incentive_event: {incentive_event:?}"
                    );
                }
                CpiEvent::InitUserVolumeAccumulatorEvent(volume_event) => {
                    log::info!(
                        "InitUserVolumeAccumulatorEvent: signature: {signature}, volume_event: {volume_event:?}"
                    );
                }
                _ => {
                    log::debug!("Other PumpSwap CpiEvent: signature: {signature}, data: {data:?}");
                }
            },
            _ => {
                log::debug!(
                    "Other PumpSwap instruction: signature: {signature}, data: {:?}",
                    input.decoded_instruction
                );
            }
        }

        Ok(())
    }
}
