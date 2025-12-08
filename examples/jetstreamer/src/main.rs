use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType, metrics::MetricsCollection,
    pipeline::Pipeline, processor::Processor,
};
use carbon_jetstreamer_datasource::{
    filter::{JetstreamerFilter, TransactionFilter},
    range::JetstreamerRange,
    JetstreamerDatasource,
};
use carbon_log_metrics::LogMetrics;
use carbon_token_2022_decoder::{instructions::Token2022Instruction, Token2022Decoder};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let jetstreamer = JetstreamerDatasource::new_with_old_faithful_mainnet(
        JetstreamerRange::Slot(367_200_000, 367_631_999), // OPTION 1: Slot Range
        // JetstreamerRange::Epoch(850), // OPTION 2: Single Epoch
        JetstreamerFilter {
            include_transactions: true,
            include_blocks: false,
            transaction_filters: vec![TransactionFilter {
                vote: Some(false),   // Exclude vote transactions
                failed: Some(false), // Exclude failed transactions
                account_include: HashSet::from([carbon_token_2022_decoder::PROGRAM_ID]), // Include only Token 2022 program transactions
                account_exclude: HashSet::new(),
                account_required: HashSet::new(),
            }],
        },
        1,         // Number of Jetstreamer threads
        Some(100), // Jetstreamer tracking interval in slots
    );

    Pipeline::builder()
        .datasource(jetstreamer)
        .instruction(Token2022Decoder, Token2022InstructionLogger)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(10) // Display metrics in console every 10 seconds
        .build()?
        .run()
        .await?;

    Ok(())
}

struct Token2022InstructionLogger;

#[async_trait]
impl Processor for Token2022InstructionLogger {
    type InputType = InstructionProcessorInputType<Token2022Instruction>;

    async fn process(
        &mut self,
        (metadata, decoded_instruction, _nested_instructions, _raw_instruction): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::info!(
            "Token2022InstructionLogger: signature: {:?}, absolute path: {:?}, decoded_instruction: {:?}",
            metadata.transaction_metadata.signature,
            metadata.absolute_path,
            decoded_instruction.data
        );
        Ok(())
    }
}
