use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    metrics::Metrics,
    processor::Processor,
};
use carbon_pumpfun_decoder::{instructions::PumpfunInstruction, PumpfunDecoder};
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::Arc,
};
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let pumpfun_program_id =
        Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

    let account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![pumpfun_program_id.to_string().clone()],
        account_exclude: vec![],
        account_required: vec![],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("pumpfun_transaction_filter".to_string(), transaction_filter);

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        "".to_string(),
        None,
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<PumpfunInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()> {
        let pumpfun_instruction: PumpfunInstruction = data.1.data;

        match pumpfun_instruction {
            PumpfunInstruction::CreateEvent(create_event) => {
                println!("\nNew token created: {:#?}", create_event);
            }
            PumpfunInstruction::TradeEvent(trade_event) => {
                if trade_event.sol_amount > 10 * LAMPORTS_PER_SOL {
                    println!("\nBig trade occured: {:#?}", trade_event);
                }
            }
            PumpfunInstruction::CompleteEvent(complete_event) => {
                println!("\nBonded: {:#?}", complete_event);
            }
            _ => {}
        };

        Ok(())
    }
}
