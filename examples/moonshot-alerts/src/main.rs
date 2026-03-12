use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_moonshot_decoder::{
        instructions::MoonshotInstruction, MoonshotDecoder, PROGRAM_ID as MOONSHOT_PROGRAM_ID,
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
        .instruction(MoonshotDecoder, MoonshotInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct MoonshotInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, MoonshotInstruction>>
    for MoonshotInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, MoonshotInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        match input.decoded_instruction {
            MoonshotInstruction::Buy { data, .. } => {
                log::info!("Buy: signature: {signature}, buy: {data:?}");
            }
            MoonshotInstruction::ConfigInit { data, .. } => {
                log::info!("ConfigInit: signature: {signature}, config_init: {data:?}");
            }
            MoonshotInstruction::ConfigUpdate { data, .. } => {
                log::info!("ConfigUpdate: signature: {signature}, config_update: {data:?}");
            }
            MoonshotInstruction::MigrateFunds { data, .. } => {
                log::info!("MigrateFunds: signature: {signature}, migrate_funds: {data:?}");
            }
            MoonshotInstruction::Sell { data, .. } => {
                log::info!("Sell: signature: {signature}, sell: {data:?}");
            }
            MoonshotInstruction::TokenMint { data, .. } => {
                log::info!("TokenMint: signature: {signature}, token_mint: {data:?}");
            }
            MoonshotInstruction::CpiEvent { data, .. } => {
                log::info!("CpiEvent: signature: {signature}, cpi_event: {data:?}");
            }
        }

        Ok(())
    }
}
