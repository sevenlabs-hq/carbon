use {
    carbon_core::{
        account::AccountProcessorInputType, error::CarbonResult,
        instruction::InstructionProcessorInputType, metrics::MetricsCollection,
        processor::Processor,
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
    fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, KaminoLendingInstruction>,
        _metrics: Arc<MetricsCollection>,
    ) -> impl std::future::Future<Output = CarbonResult<()>> + Send {
        async move {
            let signature = input.metadata.transaction_metadata.signature;

            let signature = format!(
                "{}...{}",
                &signature.to_string()[..4],
                &signature.to_string()
                    [signature.to_string().len() - 4..signature.to_string().len()]
            );

            log::info!(
                "instruction processed ({}) {:?}",
                signature,
                input.decoded_instruction.data.clone()
            );

            Ok(())
        }
    }
}

pub struct KaminoLendingAccountProcessor;

impl Processor<AccountProcessorInputType<'_, KaminoLendingAccount>>
    for KaminoLendingAccountProcessor
{
    fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, KaminoLendingAccount>,
        _metrics: Arc<MetricsCollection>,
    ) -> impl std::future::Future<Output = CarbonResult<()>> + Send {
        async move {
            let pubkey_str = format!(
                "{}...{}",
                &input.metadata.pubkey.to_string()[..4],
                &input.metadata.pubkey.to_string()[4..]
            );

            fn max_total_chars(s: &str, max: usize) -> String {
                if s.len() > max {
                    format!("{}...", &s[..max])
                } else {
                    s.to_string()
                }
            }

            log::info!(
                "account updated ({}) {:?}",
                pubkey_str,
                max_total_chars(
                    &match &input.decoded_account.data {
                        KaminoLendingAccount::UserState(user_state) => format!("{user_state:?}"),
                        KaminoLendingAccount::LendingMarket(lending_market) =>
                            format!("{lending_market:?}"),
                        KaminoLendingAccount::Obligation(obligation) => format!("{obligation:?}"),
                        KaminoLendingAccount::ReferrerState(referrer_state) =>
                            format!("{referrer_state:?}"),
                        KaminoLendingAccount::ReferrerTokenState(referrer_token_state) => {
                            format!("{referrer_token_state:?}")
                        }
                        KaminoLendingAccount::ShortUrl(short_url) => format!("{short_url:?}"),
                        KaminoLendingAccount::UserMetadata(user_metadata) =>
                            format!("{user_metadata:?}"),
                        KaminoLendingAccount::Reserve(reserve) => format!("{reserve:?}"),
                    },
                    100
                )
            );

            Ok(())
        }
    }
}
