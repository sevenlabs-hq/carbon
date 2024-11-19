use async_trait::async_trait;
use carbon_core::{
    account::{AccountMetadata, DecodedAccount},
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    metrics::MetricsCollection,
    processor::Processor,
};
use carbon_kamino_lending_decoder::{
    accounts::KaminoLendingAccount, instructions::KaminoLendingInstruction, KaminoLendingDecoder,
};
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use solana_sdk::{pubkey, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
};

pub const KAMINO_LENDING_PROGRAM_ID: Pubkey =
    pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

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

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
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

#[async_trait]
impl Processor for KaminoLendingInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<KaminoLendingInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;

        let signature = format!(
            "{}...{}",
            &signature.to_string()[..4],
            &signature.to_string()[signature.to_string().len() - 4..signature.to_string().len()]
        );

        println!(
            "instruction processed ({}) {:?}",
            signature, instruction.data
        );

        Ok(())
    }
}

pub struct KaminoLendingAccountProcessor;
#[async_trait]
impl Processor for KaminoLendingAccountProcessor {
    type InputType = (AccountMetadata, DecodedAccount<KaminoLendingAccount>);

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let account = data.1;

        let pubkey_str = format!(
            "{}...{}",
            &data.0.pubkey.to_string()[..4],
            &data.0.pubkey.to_string()[4..]
        );

        fn max_total_chars(s: &str, max: usize) -> String {
            if s.len() > max {
                format!("{}...", &s[..max])
            } else {
                s.to_string()
            }
        }

        println!(
            "account updated ({}) {:?}",
            pubkey_str,
            max_total_chars(
                &match account.data {
                    KaminoLendingAccount::UserState(user_state) => format!("{:?}", user_state),
                    KaminoLendingAccount::LendingMarket(lending_market) =>
                        format!("{:?}", lending_market),
                    KaminoLendingAccount::Obligation(obligation) => format!("{:?}", obligation),
                    KaminoLendingAccount::ReferrerState(referrer_state) =>
                        format!("{:?}", referrer_state),
                    KaminoLendingAccount::ReferrerTokenState(referrer_token_state) => {
                        format!("{:?}", referrer_token_state)
                    }
                    KaminoLendingAccount::ShortUrl(short_url) => format!("{:?}", short_url),
                    KaminoLendingAccount::UserMetadata(user_metadata) =>
                        format!("{:?}", user_metadata),
                    KaminoLendingAccount::Reserve(reserve) => format!("{:?}", reserve),
                },
                100
            )
        );

        Ok(())
    }
}
