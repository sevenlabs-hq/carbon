use {
    carbon_core::{
        account::AccountProcessorInputType, error::CarbonResult,
        instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_kamino_lending_decoder::{
        accounts::KaminoLendingAccount, instructions::KaminoLendingInstruction,
        KaminoLendingDecoder, PROGRAM_ID as KAMINO_LENDING_PROGRAM_ID,
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
        "kamino_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![KAMINO_LENDING_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![KAMINO_LENDING_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("kamino_transaction_filter".to_string(), transaction_filter);

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
        .instruction(KaminoLendingDecoder, KaminoLendingInstructionProcessor)
        .account(KaminoLendingDecoder, KaminoLendingAccountProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct KaminoLendingInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, KaminoLendingInstruction>>
    for KaminoLendingInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, KaminoLendingInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        log::info!(
            "instruction processed ({signature}) {:?}",
            input.decoded_instruction
        );

        Ok(())
    }
}

pub struct KaminoLendingAccountProcessor;
impl Processor<AccountProcessorInputType<'_, KaminoLendingAccount>>
    for KaminoLendingAccountProcessor
{
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, KaminoLendingAccount>,
    ) -> CarbonResult<()> {
        log::info!(
            "account updated ({}) {:?}",
            input.metadata.pubkey,
            &input.decoded_account.data
        );

        Ok(())
    }
}
