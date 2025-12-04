use std::{env, sync::Arc};

use solana_transaction_status::UiTransactionEncoding;

use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType,
        metrics::MetricsCollection, processor::Processor,
    },
    carbon_pumpfun_decoder::{instructions::PumpfunInstruction, PumpfunDecoder},
    carbon_rpc_block_crawler_datasource::{RpcBlockConfig, RpcBlockCrawler},
    clap::Parser,
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

    Ok(())
}

pub struct PumpfunInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, PumpfunInstruction>>
    for PumpfunInstructionProcessor
{
    fn process(
        &mut self,
        data: &InstructionProcessorInputType<'_, PumpfunInstruction>,
        _metrics: Arc<MetricsCollection>,
    ) -> impl std::future::Future<Output = CarbonResult<()>> + Send {
        async move {
            match data.decoded_instruction.data.clone() {
                PumpfunInstruction::CreateEvent(create_event) => {
                    log::info!(
                        "New token created: {:#?} on slot {}",
                        create_event,
                        data.metadata.transaction_metadata.slot
                    );
                }
                PumpfunInstruction::TradeEvent(trade_event) => {
                    log::info!(
                        "New trade occured: {:#?} on slot {:#?}",
                        trade_event,
                        data.metadata.transaction_metadata.slot
                    );
                }
                _ => {}
            };

            Ok(())
        }
    }
}
