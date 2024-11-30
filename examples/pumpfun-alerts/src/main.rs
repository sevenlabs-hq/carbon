use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType, metrics::MetricsCollection,
    processor::Processor,
};
use carbon_helius_atlas_ws_datasource::HeliusWebsocket;
use carbon_pumpfun_decoder::{instructions::PumpfunInstruction, PumpfunDecoder};
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use solana_sdk::pubkey;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
};

pub const PUMPFUN_PROGRAM_ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

pub fn create_yellowstone_grpc_datasource(
    grpc_url: String,
    x_token: String,
) -> YellowstoneGrpcGeyserClient {
    let account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![PUMPFUN_PROGRAM_ID.to_string().clone()],
        account_exclude: vec![],
        account_required: vec![],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("pumpfun_transaction_filter".to_string(), transaction_filter);

    YellowstoneGrpcGeyserClient::new(
        grpc_url,
        Some(x_token),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    )
}

pub fn create_helius_websocket_datasource(api_key: String) -> HeliusWebsocket {
    HeliusWebsocket::new(
        api_key,
        carbon_helius_atlas_ws_datasource::Filters {
            accounts: vec![],
            transactions: Some(helius::types::enhanced_websocket::RpcTransactionsConfig {
                filter: helius::types::enhanced_websocket::TransactionSubscribeFilter {
                    account_include: Some(vec![PUMPFUN_PROGRAM_ID.to_string().clone()]),
                    account_exclude: None,
                    account_required: None,
                    vote: None,
                    failed: None,
                    signature: None,
                },
                options: helius::types::enhanced_websocket::TransactionSubscribeOptions {
                    commitment: Some(helius::types::TransactionCommitment::Confirmed),
                    encoding: None,
                    transaction_details: Some(helius::types::TransactionDetails::Full),
                    show_rewards: None,
                    max_supported_transaction_version: None,
                },
            }),
        },
        Arc::new(RwLock::new(HashSet::new())),
    )
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    // let yellowstone_grpc = create_yellowstone_grpc_datasource(
    //     env::var("GEYSER_URL").unwrap_or_default(),
    //     env::var("X_TOKEN").unwrap_or_default(),
    // );

    let helius_websocket =
        create_helius_websocket_datasource(env::var("API_KEY").unwrap_or_default());

    carbon_core::pipeline::Pipeline::builder()
        // .datasource(yellowstone_grpc)
        .datasource(helius_websocket)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let pumpfun_instruction: PumpfunInstruction = data.1.data;

        match pumpfun_instruction {
            PumpfunInstruction::CreateEvent(create_event) => {
                println!("\nNew token created: {:#?}", create_event);
            }
            PumpfunInstruction::TradeEvent(trade_event) => {
                if trade_event.sol_amount > 10 * LAMPORTS_PER_SOL {
                    println!("\nBig trade occured: {:#?}", trade_event);
                }
            }
            PumpfunInstruction::CompleteEvent(complete_event) => {
                println!("\nBonded: {:#?}", complete_event);
            }
            _ => {}
        };

        Ok(())
    }
}
