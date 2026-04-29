use {
    carbon_core::{
        account::AccountProcessorInputType, error::CarbonResult, filter::DeduplicationFilter,
        instruction::InstructionProcessorInputType, pipeline::Pipeline, processor::Processor,
    },
    carbon_jetstreamer_datasource::{
        filter::{JetstreamerFilter, TransactionFilter},
        range::JetstreamerRange,
        JetstreamerDatasource,
    },
    carbon_kamino_lending_decoder::{
        accounts::KaminoLendingAccount, instructions::KaminoLendingInstruction,
        KaminoLendingDecoder, PROGRAM_ID as KAMINO_PROGRAM_ID,
    },
    carbon_log_metrics::LogMetrics,
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

    // https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls default provider");

    let jetstream = JetstreamerDatasource::new_with_old_faithful_mainnet(
        JetstreamerRange::Slot(415_500_000, 415_931_999),
        // alt: JetstreamerRange::Epoch(962),
        JetstreamerFilter {
            include_transactions: true,
            include_blocks: false,
            transaction_filters: vec![TransactionFilter {
                vote: Some(false),
                failed: Some(false),
                account_include: HashSet::from([KAMINO_PROGRAM_ID]),
                account_exclude: HashSet::new(),
                account_required: HashSet::new(),
            }],
        },
        4,
        Some(100),
    );

    let mut yellowstone_account_filters = HashMap::new();
    yellowstone_account_filters.insert(
        "kamino".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![KAMINO_PROGRAM_ID.to_string()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );
    let mut yellowstone_tx_filters = HashMap::new();
    yellowstone_tx_filters.insert(
        "kamino".to_string(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            account_include: vec![],
            account_exclude: vec![],
            account_required: vec![KAMINO_PROGRAM_ID.to_string()],
            signature: None,
        },
    );
    let yellowstone = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").expect("GEYSER_URL must be set"),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        yellowstone_account_filters,
        yellowstone_tx_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        YellowstoneGrpcClientConfig::default(),
        None,
        None,
    );

    let dedup = DeduplicationFilter::new(Duration::from_secs(10));

    Pipeline::builder()
        .datasource(jetstream)
        .datasource(yellowstone)
        .metrics(Arc::new(LogMetrics::new()))
        .account(KaminoLendingDecoder, AccountProcessor)
        .instruction_with_filters(
            KaminoLendingDecoder,
            InstructionProcessor,
            vec![Box::new(dedup)],
        )
        .channel_buffer_size(1_000_000)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct AccountProcessor;
impl Processor<AccountProcessorInputType<'_, KaminoLendingAccount>> for AccountProcessor {
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, KaminoLendingAccount>,
    ) -> CarbonResult<()> {
        log::info!(
            "account pubkey={} slot={} kind={:?}",
            input.metadata.pubkey,
            input.metadata.slot,
            account_kind(&input.decoded_account.data),
        );
        Ok(())
    }
}

pub struct InstructionProcessor;
impl Processor<InstructionProcessorInputType<'_, KaminoLendingInstruction>>
    for InstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, KaminoLendingInstruction>,
    ) -> CarbonResult<()> {
        log::info!(
            "instruction slot={} sig={} ix={:?}",
            input.metadata.transaction_metadata.slot,
            input.metadata.transaction_metadata.signature,
            instruction_kind(input.decoded_instruction),
        );
        Ok(())
    }
}

fn account_kind(account: &KaminoLendingAccount) -> &'static str {
    match account {
        KaminoLendingAccount::LendingMarket(_) => "LendingMarket",
        KaminoLendingAccount::Reserve(_) => "Reserve",
        KaminoLendingAccount::Obligation(_) => "Obligation",
        _ => "other",
    }
}

fn instruction_kind(ix: &KaminoLendingInstruction) -> &'static str {
    match ix {
        KaminoLendingInstruction::DepositReserveLiquidity { .. } => "DepositReserveLiquidity",
        KaminoLendingInstruction::BorrowObligationLiquidity { .. } => "BorrowObligationLiquidity",
        KaminoLendingInstruction::RepayObligationLiquidity { .. } => "RepayObligationLiquidity",
        KaminoLendingInstruction::LiquidateObligationAndRedeemReserveCollateral { .. } => {
            "LiquidateObligationAndRedeem"
        }
        _ => "other",
    }
}
