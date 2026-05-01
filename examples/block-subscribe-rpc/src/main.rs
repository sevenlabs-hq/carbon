use {
    carbon_core::{
        datasource::BlockDetails, error::CarbonResult, instruction::InstructionProcessorInputType,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_raydium_clmm_decoder::{
        instructions::RaydiumClmmInstruction, RaydiumClmmDecoder,
        PROGRAM_ID as RAYDIUM_CLMM_PROGRAM_ID,
    },
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    solana_commitment_config::CommitmentConfig,
    solana_transaction_status::{TransactionDetails, UiTransactionEncoding},
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let filters = Filters::new(
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(RAYDIUM_CLMM_PROGRAM_ID.to_string()),
        Some(RpcBlockSubscribeConfig {
            commitment: Some(CommitmentConfig::confirmed()),
            encoding: Some(UiTransactionEncoding::Base64),
            transaction_details: Some(TransactionDetails::Full),
            show_rewards: Some(false),
            max_supported_transaction_version: Some(0),
        }),
    );

    let datasource = RpcBlockSubscribe::new(
        env::var("RPC_WS_URL").unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
        filters,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(RaydiumClmmDecoder, RaydiumClmmInstructionProcessor)
        .block_details(BlockDetailsProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct RaydiumClmmInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, RaydiumClmmInstruction>>
    for RaydiumClmmInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, RaydiumClmmInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;
        let slot = input.metadata.transaction_metadata.slot;

        match input.decoded_instruction {
            RaydiumClmmInstruction::Swap { data, .. } => {
                log::info!("swap slot={slot} sig={signature} data={data:?}");
            }
            RaydiumClmmInstruction::SwapV2 { data, .. } => {
                log::info!("swap_v2 slot={slot} sig={signature} data={data:?}");
            }
            RaydiumClmmInstruction::OpenPosition { data, .. } => {
                log::info!("open_position slot={slot} sig={signature} data={data:?}");
            }
            other => {
                log::debug!("other slot={slot} sig={signature} variant={other:?}");
            }
        }

        Ok(())
    }
}

pub struct BlockDetailsProcessor;

impl Processor<BlockDetails> for BlockDetailsProcessor {
    async fn process(&mut self, block: &BlockDetails) -> CarbonResult<()> {
        log::info!(
            "block slot={} hash={:?} parent={:?} time={:?}",
            block.slot,
            block.block_hash,
            block.previous_block_hash,
            block.block_time,
        );
        Ok(())
    }
}
