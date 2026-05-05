use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType,
        metrics::MetricsCollection, processor::Processor,
    },
    carbon_pumpfun_decoder::{
        instructions::{CpiEvent, PumpInstruction},
        PumpDecoder, PROGRAM_ID as PUMPFUN_PROGRAM_ID,
    },
    helius::types::{
        Cluster, RpcTransactionsConfig, TransactionCommitment, TransactionDetails,
        TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding,
    },
    solana_native_token::LAMPORTS_PER_SOL,
    std::{collections::HashSet, sync::Arc},
    tokio::sync::RwLock,
};

const NATIVE_SOL_MINT: &str = "So11111111111111111111111111111111111111112";

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let ping_interval_secs = std::env::var("PING_INTERVAL_SECS")
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .unwrap_or(10);
    let pong_timeout_secs = std::env::var("PONG_TIMEOUT_SECS")
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .unwrap_or(10);
    let transaction_idle_timeout_secs = std::env::var("TRANSACTION_IDLE_TIMEOUT_SECS")
        .unwrap_or("60".to_string())
        .parse::<u64>()
        .unwrap_or(60);

    let helius_websocket = carbon_helius_atlas_ws_datasource::HeliusWebsocket::new(
        std::env::var("API_KEY").expect("API_KEY must be set"),
        carbon_helius_atlas_ws_datasource::Filters {
            accounts: vec![],
            transactions: Some(RpcTransactionsConfig {
                filter: TransactionSubscribeFilter {
                    account_include: Some(vec![PUMPFUN_PROGRAM_ID.to_string().clone()]),
                    account_exclude: None,
                    account_required: None,
                    vote: None,
                    failed: None,
                    signature: None,
                },
                options: TransactionSubscribeOptions {
                    commitment: Some(TransactionCommitment::Confirmed),
                    encoding: Some(UiEnhancedTransactionEncoding::Base64),
                    transaction_details: Some(TransactionDetails::Full),
                    show_rewards: None,
                    max_supported_transaction_version: Some(0),
                },
            }),
        },
        Arc::new(RwLock::new(HashSet::new())),
        Cluster::MainnetBeta,
    )
    .with_ping_interval_secs(ping_interval_secs)
    .with_pong_timeout_secs(pong_timeout_secs)
    .with_transaction_idle_timeout_secs(transaction_idle_timeout_secs);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(helius_websocket)
        .instruction(PumpDecoder, PumpInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpInstructionProcessor;

#[async_trait]
impl Processor for PumpInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let pumpfun_instruction: PumpInstruction = data.1.data;

        if let PumpInstruction::CpiEvent(cpi_event) = pumpfun_instruction {
            match cpi_event {
                CpiEvent::CreateEvent(create_event) => {
                    log::info!(
                        "New token created with quote mint {}: {create_event:#?}",
                        create_event.quote_mint,
                    );
                }
                CpiEvent::TradeEvent(trade_event) => {
                    if is_native_sol_quote(&trade_event.quote_mint)
                        && trade_event.quote_amount > 10 * LAMPORTS_PER_SOL
                    {
                        log::info!(
                            "Big SOL quote trade occurred: quote_amount: {:.4} SOL, trade: {trade_event:#?}",
                            trade_event.quote_amount as f64 / LAMPORTS_PER_SOL as f64,
                        );
                    } else {
                        log::info!(
                            "Trade occurred: quote_mint: {}, quote_amount: {}, trade: {trade_event:#?}",
                            trade_event.quote_mint,
                            trade_event.quote_amount,
                        );
                    }
                }
                CpiEvent::CompleteEvent(complete_event) => {
                    log::info!(
                        "Bonded with quote mint {}: {complete_event:#?}",
                        complete_event.quote_mint,
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn is_native_sol_quote(quote_mint: &impl std::fmt::Display) -> bool {
    quote_mint.to_string() == NATIVE_SOL_MINT
}
