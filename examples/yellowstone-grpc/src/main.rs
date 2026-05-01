mod variants;

use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_jupiter_swap_decoder::{
        instructions::JupiterSwapInstruction, JupiterSwapDecoder,
        PROGRAM_ID as JUPITER_SWAP_PROGRAM_ID,
    },
    carbon_log_metrics::LogMetrics,
    std::{collections::HashMap, sync::Arc},
    yellowstone_grpc_proto::geyser::SubscribeRequestFilterTransactions,
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls default provider");

    let mut transaction_filters = HashMap::new();
    transaction_filters.insert(
        "jupiter_v6".to_string(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            account_include: vec![],
            account_exclude: vec![],
            account_required: vec![JUPITER_SWAP_PROGRAM_ID.to_string()],
            signature: None,
        },
    );

    let datasource = variants::yellowstone(transaction_filters);
    // alt: let datasource = variants::laserstream(transaction_filters);
    // alt: let datasource = variants::jito_shredstream(); // ignores
    // transaction_filters; decoder filters down to JUPITER_SWAP_PROGRAM_ID

    carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(JupiterSwapDecoder, JupiterSwapInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct JupiterSwapInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, JupiterSwapInstruction>>
    for JupiterSwapInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, JupiterSwapInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;
        let slot = input.metadata.transaction_metadata.slot;

        match input.decoded_instruction {
            JupiterSwapInstruction::Route { data, .. } => {
                log::info!("route slot={slot} sig={signature} data={data:?}");
            }
            JupiterSwapInstruction::SharedAccountsRoute { data, .. } => {
                log::info!("shared_accounts_route slot={slot} sig={signature} data={data:?}");
            }
            other => {
                log::debug!("other slot={slot} sig={signature} variant={other:?}");
            }
        }

        Ok(())
    }
}
