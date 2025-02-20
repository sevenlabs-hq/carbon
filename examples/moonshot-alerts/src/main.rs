use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_moonshot_decoder::{instructions::MoonshotInstruction, MoonshotDecoder},
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    solana_sdk::{pubkey, pubkey::Pubkey},
    std::{env, sync::Arc},
};

pub const MOONSHOT_PROGRAM_ID: Pubkey = pubkey!("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let filters = Filters::new(
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(MOONSHOT_PROGRAM_ID.to_string()),
        Some(RpcBlockSubscribeConfig {
            max_supported_transaction_version: Some(0),
            ..RpcBlockSubscribeConfig::default()
        }),
    );

    let rpc_ws_url =
        env::var("RPC_WS_URL").unwrap_or("wss://api.mainnet-beta.solana.com/".to_string());

    log::info!("Starting with RPC: {}", rpc_ws_url);
    let block_subscribe = RpcBlockSubscribe::new(rpc_ws_url, filters);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(block_subscribe)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
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
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;

        match instruction.data {
            MoonshotInstruction::TokenMint(token_mint) => {
                log::info!("TokenMint: signature: {signature}, token_mint: {token_mint:?}");
            }
            MoonshotInstruction::Buy(buy) => {
                log::info!("Buy: signature: {signature}, buy: {buy:?}");
            }
            MoonshotInstruction::Sell(sell) => {
                log::info!("Sell: signature: {signature}, sell: {sell:?}");
            }
            MoonshotInstruction::MigrateFunds(migrate_funds) => {
                log::info!(
                    "MigrateFunds: signature: {signature}, migrate_funds: {migrate_funds:?}"
                );
            }
            MoonshotInstruction::ConfigInit(config_init) => {
                log::info!("ConfigInit: signature: {signature}, config_init: {config_init:?}");
            }
            MoonshotInstruction::ConfigUpdate(config_update) => {
                log::info!(
                    "ConfigUpdate: signature: {signature}, config_update: {config_update:?}"
                );
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
