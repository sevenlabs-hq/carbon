use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_pumpfun_decoder::{
        instructions::{CpiEvent, PumpfunInstruction},
        PumpfunDecoder, PROGRAM_ID as PUMPFUN_PROGRAM_ID,
    },
    helius::types::{
        Cluster, RpcTransactionsConfig, TransactionCommitment, TransactionDetails,
        TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding,
    },
    solana_native_token::LAMPORTS_PER_SOL,
    std::{collections::HashSet, sync::Arc},
    tokio::sync::RwLock,
};

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
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpfunInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, PumpfunInstruction>>
    for PumpfunInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, PumpfunInstruction>,
    ) -> CarbonResult<()> {
        match input.decoded_instruction {
            PumpfunInstruction::Create { data, accounts, .. } => {
                log::info!(
                    "New token created: name={}, symbol={}, uri={}, mint={}, slot={}",
                    data.name,
                    data.symbol,
                    data.uri,
                    accounts.mint,
                    input.metadata.transaction_metadata.slot
                );
            }
            PumpfunInstruction::CreateV2 { data, accounts, .. } => {
                log::info!(
                    "New token created (v2): name={}, symbol={}, uri={}, mint={}, slot={}",
                    data.name,
                    data.symbol,
                    data.uri,
                    accounts.mint,
                    input.metadata.transaction_metadata.slot
                );
            }
            PumpfunInstruction::Buy { data, accounts, .. } => {
                log::info!(
                    "Buy: amount={}, max_sol_cost={}, mint={}, user={}, slot={}",
                    data.amount,
                    data.max_sol_cost,
                    accounts.mint,
                    accounts.user,
                    input.metadata.transaction_metadata.slot
                );
            }
            PumpfunInstruction::BuyExactSolIn { data, accounts, .. } => {
                log::info!(
                    "Buy (exact SOL): spendable_sol_in={}, min_tokens_out={}, mint={}, user={}, slot={}",
                    data.spendable_sol_in,
                    data.min_tokens_out,
                    accounts.mint,
                    accounts.user,
                    input.metadata.transaction_metadata.slot
                );
            }
            PumpfunInstruction::Sell { data, accounts, .. } => {
                log::info!(
                    "Sell: amount={}, min_sol_output={}, mint={}, user={}, slot={}",
                    data.amount,
                    data.min_sol_output,
                    accounts.mint,
                    accounts.user,
                    input.metadata.transaction_metadata.slot
                );
            }
            PumpfunInstruction::CpiEvent { data, .. } => {
                match data {
                    CpiEvent::CreateEvent(create_event) => {
                        log::info!("Create event: {create_event:#?}");
                    }
                    CpiEvent::TradeEvent(trade_event) => {
                        if trade_event.sol_amount > 10 * LAMPORTS_PER_SOL {
                            log::info!("Big trade event: {trade_event:#?}");
                        }
                    }
                    CpiEvent::CompleteEvent(complete_event) => {
                        log::info!("Complete event: {complete_event:#?}");
                    }
                    _ => {}
                }
            }
            _ => {
                log::debug!(
                    "Other instruction: {:?} on slot {}",
                    input.decoded_instruction,
                    input.metadata.transaction_metadata.slot
                );
            }
        }

        Ok(())
    }
}
