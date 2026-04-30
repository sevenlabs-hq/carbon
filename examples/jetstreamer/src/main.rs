use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, pipeline::Pipeline,
        processor::Processor,
    },
    carbon_jetstreamer_datasource::{
        filter::{JetstreamerFilter, TransactionFilter},
        range::JetstreamerRange,
        JetstreamerDatasource,
    },
    carbon_log_metrics::LogMetrics,
    carbon_token_2022_decoder::{instructions::Token2022Instruction, Token2022Decoder},
    std::{collections::HashSet, env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let start_slot = env_u64("START_SLOT", 415_500_000);
    let end_slot = env_u64("END_SLOT", 415_931_999);
    let worker_count = env_u64("WORKER_COUNT", 4);
    let page_size = env_u64("PAGE_SIZE", 100);

    let datasource = JetstreamerDatasource::new_with_old_faithful_mainnet(
        JetstreamerRange::Slot(start_slot, end_slot),
        JetstreamerFilter {
            include_transactions: true,
            include_blocks: false,
            transaction_filters: vec![TransactionFilter {
                vote: Some(false),
                failed: Some(false),
                account_include: HashSet::from([carbon_token_2022_decoder::PROGRAM_ID]),
                account_exclude: HashSet::new(),
                account_required: HashSet::new(),
            }],
        },
        worker_count,
        Some(page_size),
    );

    Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(Token2022Decoder, Token2022InstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::ProcessPending)
        .build()?
        .run()
        .await?;

    log::info!("backfill complete");
    Ok(())
}

fn env_u64(name: &str, default: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

pub struct Token2022InstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, Token2022Instruction>>
    for Token2022InstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, Token2022Instruction>,
    ) -> CarbonResult<()> {
        let slot = input.metadata.transaction_metadata.slot;
        log::info!("token2022 slot={slot} ix={:?}", input.decoded_instruction);
        Ok(())
    }
}
