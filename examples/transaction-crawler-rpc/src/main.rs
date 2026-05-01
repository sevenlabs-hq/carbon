mod variants;

use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_meteora_dlmm_decoder::{
        instructions::MeteoraDlmmInstruction, MeteoraDlmmDecoder, PROGRAM_ID as METEORA_PROGRAM_ID,
    },
    std::sync::Arc,
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let datasource = variants::rpc(METEORA_PROGRAM_ID);
    // alt: let datasource = variants::helius_gtfa(METEORA_PROGRAM_ID);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(MeteoraDlmmDecoder, MeteoraDlmmInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct MeteoraDlmmInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, MeteoraDlmmInstruction>>
    for MeteoraDlmmInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, MeteoraDlmmInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;
        let slot = input.metadata.transaction_metadata.slot;

        match input.decoded_instruction {
            MeteoraDlmmInstruction::Swap { data, .. } => {
                log::info!("swap slot={slot} sig={signature} data={data:?}");
            }
            MeteoraDlmmInstruction::AddLiquidity { data, .. } => {
                log::info!("add_liquidity slot={slot} sig={signature} data={data:?}");
            }
            MeteoraDlmmInstruction::RemoveLiquidity { data, .. } => {
                log::info!("remove_liquidity slot={slot} sig={signature} data={data:?}");
            }
            other => {
                log::debug!("other slot={slot} sig={signature} variant={other:?}");
            }
        }

        Ok(())
    }
}
