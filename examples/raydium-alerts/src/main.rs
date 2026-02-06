use {
    async_trait::async_trait,
    carbon_core::{
        account::{AccountMetadata, DecodedAccount},
        deserialize::ArrangeAccounts,
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_raydium_amm_v4_decoder::{
        accounts::RaydiumAmmV4Account,
        instructions::{
            swap_base_in::SwapBaseIn, swap_base_in_v2::SwapBaseInV2, swap_base_out::SwapBaseOut,
            swap_base_out_v2::SwapBaseOutV2, RaydiumAmmV4Instruction,
        },
        RaydiumAmmV4Decoder, PROGRAM_ID as RAYDIUM_AMM_V4_PROGRAM_ID,
    },
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
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
        "raydium_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![RAYDIUM_AMM_V4_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![RAYDIUM_AMM_V4_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("raydium_transaction_filter".to_string(), transaction_filter);

    let geyser_config = YellowstoneGrpcClientConfig::new(
        None,
        Some(Duration::from_secs(15)),
        Some(Duration::from_secs(15)),
        None,
        None,
        None,
    );

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        geyser_config,
        None,
        None,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .instruction(RaydiumAmmV4Decoder, RaydiumAmmV4InstructionProcessor)
        .account(RaydiumAmmV4Decoder, RaydiumAmmV4AccountProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct RaydiumAmmV4InstructionProcessor;

#[async_trait]
impl Processor for RaydiumAmmV4InstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<RaydiumAmmV4Instruction>,
        NestedInstructions,
        solana_instruction::Instruction,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions, _): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;

        match instruction.data {
            RaydiumAmmV4Instruction::Initialize2(init_pool) => {
                log::info!("Initialize2: signature: {signature}, init_pool: {init_pool:?}");
            }
            RaydiumAmmV4Instruction::Initialize(initialize) => {
                log::info!("Initialize: signature: {signature}, initialize: {initialize:?}");
            }
            RaydiumAmmV4Instruction::MonitorStep(monitor_step) => {
                log::info!("MonitorStep: signature: {signature}, monitor_step: {monitor_step:?}");
            }
            RaydiumAmmV4Instruction::Deposit(deposit) => {
                log::info!("Deposit: signature: {signature}, deposit: {deposit:?}");
            }
            RaydiumAmmV4Instruction::Withdraw(withdraw) => {
                log::info!("Withdraw: signature: {signature}, withdraw: {withdraw:?}");
            }
            RaydiumAmmV4Instruction::MigrateToOpenBook(migrate_to_open_book) => {
                log::info!("MigrateToOpenBook: signature: {signature}, migrate_to_open_book: {migrate_to_open_book:?}");
            }
            RaydiumAmmV4Instruction::SetParams(set_params) => {
                log::info!("SetParams: signature: {signature}, set_params: {set_params:?}");
            }
            RaydiumAmmV4Instruction::WithdrawPnl(withdraw_pnl) => {
                log::info!(
                    "SetPaWithdrawPnlrams: signature: {signature}, withdraw_pnl: {withdraw_pnl:?}"
                );
            }
            RaydiumAmmV4Instruction::WithdrawSrm(withdraw_srm) => {
                log::info!("WithdrawSrm: signature: {signature}, withdraw_srm: {withdraw_srm:?}");
            }
            RaydiumAmmV4Instruction::SwapBaseIn(swap_base_in) => {
                match SwapBaseIn::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!(
                        "SwapBaseIn: signature: {signature}, swap_base_in: {swap_base_in:?}, accounts: {accounts:#?}",
                    );
                    }
                    None => log::error!(
                        "Failed to arrange accounts for SwapBaseIn {}",
                        accounts.len()
                    ),
                }
            }
            RaydiumAmmV4Instruction::SwapBaseInV2(swap_base_in) => {
                match SwapBaseInV2::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!(
                        "SwapBaseInV2: signature: {signature}, swap_base_in: {swap_base_in:?}, accounts: {accounts:#?}",
                    );
                    }
                    None => log::error!(
                        "Failed to arrange accounts for SwapBaseInV2 {}",
                        accounts.len()
                    ),
                }
            }
            RaydiumAmmV4Instruction::PreInitialize(pre_initialize) => {
                log::info!(
                    "PreInitialize: signature: {signature}, pre_initialize: {pre_initialize:?}"
                );
            }
            RaydiumAmmV4Instruction::SwapBaseOut(swap_base_out) => {
                match SwapBaseOut::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!(
                            "SwapBaseOut: signature: {signature}, swap_base_out: {swap_base_out:?}, accounts: {accounts:#?}",
                        );
                    }
                    None => log::error!(
                        "Failed to arrange accounts for SwapBaseOut {}",
                        accounts.len()
                    ),
                }
            }
            RaydiumAmmV4Instruction::SwapBaseOutV2(swap_base_out) => {
                match SwapBaseOutV2::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!(
                            "SwapBaseOutV2: signature: {signature}, swap_base_out: {swap_base_out:?}, accounts: {accounts:#?}",
                        );
                    }
                    None => log::error!(
                        "Failed to arrange accounts for SwapBaseOutV2 {}",
                        accounts.len()
                    ),
                }
            }
            RaydiumAmmV4Instruction::SimulateInfo(simulate_info) => {
                log::info!(
                    "SimulateInfo: signature: {signature}, simulate_info: {simulate_info:?}"
                );
            }
            RaydiumAmmV4Instruction::AdminCancelOrders(admin_cancel_orders) => {
                log::info!(
                    "AdminCancelOrders: signature: {signature}, admin_cancel_orders: {admin_cancel_orders:?}"
                );
            }
            RaydiumAmmV4Instruction::CreateConfigAccount(create_config_account) => {
                log::info!(
                    "CreateConfigAccount: signature: {signature}, create_config_account: {create_config_account:?}"
                );
            }
            RaydiumAmmV4Instruction::UpdateConfigAccount(update_config_account) => {
                log::info!(
                    "UpdateConfigAccount: signature: {signature}, update_config_account: {update_config_account:?}"
                );
            }
        };

        Ok(())
    }
}

pub struct RaydiumAmmV4AccountProcessor;
#[async_trait]
impl Processor for RaydiumAmmV4AccountProcessor {
    type InputType = (
        AccountMetadata,
        DecodedAccount<RaydiumAmmV4Account>,
        solana_account::Account,
    );

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let account = data.1;

        match account.data {
            RaydiumAmmV4Account::AmmInfo(pool) => {
                log::info!("Account: {:#?}\nPool: {:#?}", data.0.pubkey, pool);
            }
            _ => {
                log::warn!("Unnecessary Account: {:#?}", data.0.pubkey);
            }
        };

        Ok(())
    }
}
