use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_openbook_v2_decoder::{
        instructions::OpenbookV2Instruction, OpenbookV2Decoder,
        PROGRAM_ID as OPENBOOK_V2_PROGRAM_ID,
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
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(OPENBOOK_V2_PROGRAM_ID.to_string()),
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
        .instruction(OpenbookV2Decoder, OpenbookV2InstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct OpenbookV2TransactionProcessor;

pub struct OpenbookV2InstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, OpenbookV2Instruction>>
    for OpenbookV2InstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, OpenbookV2Instruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;
        let signature_str = format!(
            "{}...{}",
            &signature.to_string()[..4],
            &signature.to_string()[signature.to_string().len() - 4..]
        );

        log::info!(
            "OpenBook V2 instruction ({}) {:?} on slot {}",
            signature_str,
            input.decoded_instruction,
            input.metadata.transaction_metadata.slot
        );

        Ok(())
    }
}
