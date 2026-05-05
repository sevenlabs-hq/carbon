use std::{env, sync::Arc};

use solana_transaction_status::UiTransactionEncoding;

use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType,
        metrics::MetricsCollection, processor::Processor,
    },
    carbon_pumpfun_decoder::{
        instructions::{CpiEvent, PumpInstruction},
        PumpDecoder,
    },
    carbon_rpc_block_crawler_datasource::{RpcBlockConfig, RpcBlockCrawler},
    clap::Parser,
};

const NATIVE_SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

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
        .instruction(PumpDecoder, PumpInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpInstructionProcessor;

#[async_trait]
impl Processor for PumpInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, pumpfun_instruction, _nested_instructions, _) = data;

        if let PumpInstruction::CpiEvent(cpi_event) = pumpfun_instruction.data {
            match cpi_event {
                CpiEvent::CreateEvent(create_event) => {
                    log::info!(
                        "New token created with quote mint {}: {:#?} on slot {}",
                        create_event.quote_mint,
                        create_event,
                        metadata.transaction_metadata.slot
                    );
                }
                CpiEvent::TradeEvent(trade_event) => {
                    log::info!(
                        "New trade occurred: quote: {}, trade: {:#?} on slot {:#?}",
                        format_quote_amount(&trade_event.quote_mint, trade_event.quote_amount),
                        trade_event,
                        metadata.transaction_metadata.slot
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn format_quote_amount(quote_mint: &impl std::fmt::Display, quote_amount: u64) -> String {
    if quote_mint.to_string() == NATIVE_SOL_MINT {
        return format!(
            "{:.4} SOL ({quote_amount} lamports)",
            quote_amount as f64 / LAMPORTS_PER_SOL as f64
        );
    }

    format!("{quote_amount} raw units of {quote_mint}")
}
