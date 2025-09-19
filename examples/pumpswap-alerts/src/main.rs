use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_helius_laserstream_datasource::{LaserStreamClientConfig, LaserStreamGeyserClient},
    carbon_log_metrics::LogMetrics,
    carbon_pump_swap_decoder::{
        instructions::PumpSwapInstruction, PumpSwapDecoder, PROGRAM_ID as PUMPSWAP_PROGRAM_ID,
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
        .metrics_flush_interval(3)
        .instruction(PumpSwapDecoder, PumpSwapInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpSwapInstructionProcessor;

#[async_trait]
impl Processor for PumpSwapInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<PumpSwapInstruction>,
        NestedInstructions,
        solana_instruction::Instruction,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions, _): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let _accounts = instruction.accounts;
        let pumpswap_instruction = instruction.data;

        match pumpswap_instruction {
            PumpSwapInstruction::Buy(buy) => {
                log::info!("Buy: signature: {signature}, buy: {buy:?}");
            }
            PumpSwapInstruction::Sell(sell) => {
                log::info!("Sell: signature: {signature}, sell: {sell:?}");
            }
            PumpSwapInstruction::CreatePool(create_pool) => {
                log::info!("CreatePool: signature: {signature}, create_pool: {create_pool:?}");
            }
            PumpSwapInstruction::Deposit(deposit) => {
                log::info!("Deposit: signature: {signature}, deposit: {deposit:?}");
            }
            PumpSwapInstruction::Withdraw(withdraw) => {
                log::info!("Withdraw: signature: {signature}, withdraw: {withdraw:?}");
            }
            PumpSwapInstruction::CreateConfig(create_config) => {
                log::info!(
                    "CreateConfig: signature: {signature}, create_config: {create_config:?}"
                );
            }
            PumpSwapInstruction::UpdateFeeConfig(update_fee_config) => {
                log::info!("UpdateFeeConfig: signature: {signature}, update_fee_config: {update_fee_config:?}");
            }
            PumpSwapInstruction::UpdateAdmin(update_admin) => {
                log::info!("UpdateAdmin: signature: {signature}, update_admin: {update_admin:?}");
            }
            PumpSwapInstruction::CollectCoinCreatorFee(collect_fee) => {
                log::info!(
                    "CollectCoinCreatorFee: signature: {signature}, collect_fee: {collect_fee:?}"
                );
            }
            PumpSwapInstruction::BuyEvent(buy_event) => {
                let sol_amount = buy_event.quote_amount_in as f64 / LAMPORTS_PER_SOL as f64;
                log::info!(
                    "BuyEvent: signature: {signature}, SOL: {:.4}, pool: {}, user: {}",
                    sol_amount,
                    buy_event.pool,
                    buy_event.user,
                );
            }
            PumpSwapInstruction::SellEvent(sell_event) => {
                let sol_amount = sell_event.quote_amount_out as f64 / LAMPORTS_PER_SOL as f64;
                log::info!(
                    "SellEvent: signature: {signature}, SOL: {:.4}, pool: {}, user: {}",
                    sol_amount,
                    sell_event.pool,
                    sell_event.user
                );
            }
            PumpSwapInstruction::CreatePoolEvent(pool_event) => {
                log::info!("CreatePoolEvent: signature: {signature}, pool_event: {pool_event:?}");
            }
            PumpSwapInstruction::DepositEvent(deposit_event) => {
                log::info!(
                    "DepositEvent: signature: {signature}, deposit_event: {deposit_event:?}"
                );
            }
            PumpSwapInstruction::WithdrawEvent(withdraw_event) => {
                log::info!(
                    "WithdrawEvent: signature: {signature}, withdraw_event: {withdraw_event:?}"
                );
            }
            PumpSwapInstruction::CreateConfigEvent(config_event) => {
                log::info!(
                    "CreateConfigEvent: signature: {signature}, config_event: {config_event:?}"
                );
            }
            PumpSwapInstruction::UpdateFeeConfigEvent(fee_config_event) => {
                log::info!("UpdateFeeConfigEvent: signature: {signature}, fee_config_event: {fee_config_event:?}");
            }
            PumpSwapInstruction::UpdateAdminEvent(admin_event) => {
                log::info!(
                    "UpdateAdminEvent: signature: {signature}, admin_event: {admin_event:?}"
                );
            }
            PumpSwapInstruction::CollectCoinCreatorFeeEvent(fee_event) => {
                let fee_amount = fee_event.coin_creator_fee as f64 / LAMPORTS_PER_SOL as f64;
                log::info!(
                    "CollectCoinCreatorFeeEvent: signature: {signature}, fee: {:.6} SOL, creator: {}",
                    fee_amount,
                    fee_event.coin_creator
                );
            }
            PumpSwapInstruction::ClaimTokenIncentivesEvent(incentive_event) => {
                log::info!("ClaimTokenIncentivesEvent: signature: {signature}, incentive_event: {incentive_event:?}");
            }
            PumpSwapInstruction::InitUserVolumeAccumulatorEvent(volume_event) => {
                log::info!("InitUserVolumeAccumulatorEvent: signature: {signature}, volume_event: {volume_event:?}");
            }
            _ => {
                log::debug!("Other PumpSwap instruction: signature: {signature}, data: {pumpswap_instruction:?}");
            }
        }

        Ok(())
    }
}
