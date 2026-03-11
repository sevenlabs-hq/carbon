use {
    carbon_core::{
        error::CarbonResult,
        instruction::InstructionProcessorInputType,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_meteora_dlmm_decoder::{
        instructions::MeteoraDlmmInstruction, MeteoraDlmmDecoder, PROGRAM_ID as METEORA_PROGRAM_ID,
    },
    carbon_rpc_transaction_crawler_datasource::{
        ConnectionConfig, Filters, RetryConfig, RpcTransactionCrawler,
    },
    solana_commitment_config::CommitmentConfig,
    std::{env, sync::Arc, time::Duration},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let filters = Filters::new(None, None, None);
    let connection_config = ConnectionConfig::new(
        100,                     // Batch limit
        Duration::from_secs(5),  // Polling interval
        5,                       // Max Concurrent Requests
        RetryConfig::no_retry(), // Retry config
        None,                    // Max Signature Channel Size
        None,                    // Max Transaction Channel Size
        true,                    // Blocking send
    );

    let transaction_crawler = RpcTransactionCrawler::new(
        env::var("RPC_URL").unwrap_or_default(), // RPC URL
        METEORA_PROGRAM_ID,                      // The test account
        connection_config,                       // Connection config
        filters,                                 // Filters
        Some(CommitmentConfig::finalized()),     // Commitment config
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(transaction_crawler)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(MeteoraDlmmDecoder, MeteoraInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct MeteoraInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, MeteoraDlmmInstruction>> for MeteoraInstructionProcessor {
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, MeteoraDlmmInstruction>,
    ) -> CarbonResult<()> {
        println!("Meteora DLMM instruction: {:?}", input.decoded_instruction);
        Ok(())
    }
}
