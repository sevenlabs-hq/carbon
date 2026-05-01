use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_pumpfun_decoder::{instructions::PumpfunInstruction, PumpfunDecoder},
    carbon_rpc_block_crawler_datasource::{RpcBlockConfig, RpcBlockCrawler},
    clap::Parser,
    solana_transaction_status::UiTransactionEncoding,
    std::env,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    start_slot: u64,

    #[arg(short, long)]
    end_slot: u64,
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let args = Args::parse();

    let rpc_block_ds = RpcBlockCrawler::new(
        env::var("RPC_URL").unwrap_or_default(),
        args.start_slot,
        Some(args.end_slot),
        None,
        RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::Binary),
            max_supported_transaction_version: Some(0),
            ..Default::default()
        },
        Some(5),
        Some(10),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(rpc_block_ds)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    log::info!("Pipeline completed successfully");
    Ok(())
}

pub struct PumpfunInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, PumpfunInstruction>>
    for PumpfunInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, PumpfunInstruction>,
    ) -> CarbonResult<()> {
        match input.decoded_instruction {
            PumpfunInstruction::Create { data, .. } => {
                log::info!(
                    "Token created: {:?} on slot {}",
                    data,
                    input.metadata.transaction_metadata.slot
                );
            }
            PumpfunInstruction::Buy { data, .. } => {
                log::info!(
                    "Buy transaction: {:?} on slot {}",
                    data,
                    input.metadata.transaction_metadata.slot
                );
            }
            PumpfunInstruction::Sell { data, .. } => {
                log::info!(
                    "Sell transaction: {:?} on slot {}",
                    data,
                    input.metadata.transaction_metadata.slot
                );
            }
            _ => {
                log::info!(
                    "Other Pumpfun instruction on slot {}: {:?}",
                    input.metadata.transaction_metadata.slot,
                    input.decoded_instruction
                );
            }
        }

        Ok(())
    }
}
