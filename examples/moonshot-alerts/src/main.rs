use {
    async_trait::async_trait,
    carbon_core::{
        deserialize::ArrangeAccounts,
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_moonshot_decoder::{
        instructions::{
            buy::Buy, config_init::ConfigInit, config_update::ConfigUpdate,
            migrate_funds::MigrateFunds, sell::Sell, token_mint::TokenMint, MoonshotInstruction,
        },
        MoonshotDecoder, PROGRAM_ID as MOONSHOT_PROGRAM_ID,
    },
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let filters = Filters::new(
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(MOONSHOT_PROGRAM_ID.to_string()),
        Some(RpcBlockSubscribeConfig {
            max_supported_transaction_version: Some(0),
            ..RpcBlockSubscribeConfig::default()
        }),
    );

    let rpc_ws_url =
        env::var("RPC_WS_URL").unwrap_or("wss://api.mainnet-beta.solana.com/".to_string());

    log::info!("Starting with RPC: {rpc_ws_url}");
    let block_subscribe = RpcBlockSubscribe::new(rpc_ws_url, filters);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(block_subscribe)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(20)
        .instruction(MoonshotDecoder, MoonshotInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct MoonshotInstructionProcessor;

#[async_trait]
impl Processor for MoonshotInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<MoonshotInstruction>,
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
            MoonshotInstruction::TokenMint(token_mint) => {
                match TokenMint::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!("TokenMint: signature: {signature}, token_mint: {token_mint:?}, accounts: {accounts:#?}");
                    }
                    None => log::error!(
                        "Failed to arrange accounts for TokenMint {}",
                        accounts.len()
                    ),
                }
            }
            MoonshotInstruction::Buy(buy) => match Buy::arrange_accounts(&accounts) {
                Some(accounts) => {
                    log::info!(
                        "Buy: signature: {signature}, buy: {buy:?}, accounts: {accounts:#?}"
                    );
                }
                None => log::error!("Failed to arrange accounts for Buy {}", accounts.len()),
            },
            MoonshotInstruction::Sell(sell) => match Sell::arrange_accounts(&accounts) {
                Some(accounts) => {
                    log::info!(
                        "Sell: signature: {signature}, sell: {sell:?}, accounts: {accounts:#?}"
                    );
                }
                None => log::error!("Failed to arrange accounts for Sell {}", accounts.len()),
            },
            MoonshotInstruction::MigrateFunds(migrate_funds) => {
                match MigrateFunds::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!(
                            "MigrateFunds: signature: {signature}, migrate_funds: {migrate_funds:?}, accounts: {accounts:#?}"
                        );
                    }
                    None => log::error!(
                        "Failed to arrange accounts for MigrateFunds {}",
                        accounts.len()
                    ),
                }
            }
            MoonshotInstruction::ConfigInit(config_init) => {
                match ConfigInit::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!(
                            "ConfigInit: signature: {signature}, config_init: {config_init:?}, accounts: {accounts:#?}"
                        );
                    }
                    None => log::error!(
                        "Failed to arrange accounts for ConfigInit {}",
                        accounts.len()
                    ),
                }
            }
            MoonshotInstruction::ConfigUpdate(config_update) => {
                match ConfigUpdate::arrange_accounts(&accounts) {
                    Some(accounts) => {
                        log::info!(
                            "ConfigUpdate: signature: {signature}, config_update: {config_update:?}, accounts: {accounts:#?}"
                        );
                    }
                    None => log::error!(
                        "Failed to arrange accounts for ConfigUpdate {}",
                        accounts.len()
                    ),
                }
            }
            MoonshotInstruction::TradeEvent(trade_event) => {
                log::info!("TradeEvent: signature: {signature}, trade_event: {trade_event:?}");
            }
            MoonshotInstruction::MigrationEvent(migration_event) => {
                log::info!(
                    "MigrationEvent: signature: {signature}, migration_event: {migration_event:?}"
                );
            }
        };

        Ok(())
    }
}
