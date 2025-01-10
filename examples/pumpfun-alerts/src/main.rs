use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType, metrics::MetricsCollection,
    processor::Processor,
};
use carbon_pumpfun_decoder::{instructions::PumpfunInstruction, PumpfunDecoder};
use helius::types::{
    Cluster, RpcTransactionsConfig, TransactionCommitment, TransactionDetails,
    TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding,
};
use solana_sdk::pubkey;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;

pub const PUMPFUN_PROGRAM_ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let helius_websocket = carbon_helius_atlas_ws_datasource::HeliusWebsocket::new(
        std::env::var("API_KEY").unwrap(),
        carbon_helius_atlas_ws_datasource::Filters {
            accounts: vec![],
            transactions: Some(RpcTransactionsConfig {
                filter: TransactionSubscribeFilter {
                    account_include: Some(vec![PUMPFUN_PROGRAM_ID.to_string().clone()]),
                    account_exclude: None,
                    account_required: None,
                    vote: None,
                    failed: None,
                    signature: None,
                },
                options: TransactionSubscribeOptions {
                    commitment: Some(TransactionCommitment::Confirmed),
                    encoding: Some(UiEnhancedTransactionEncoding::Base64),
                    transaction_details: Some(TransactionDetails::Full),
                    show_rewards: None,
                    max_supported_transaction_version: Some(0),
                },
            }),
        },
        Arc::new(RwLock::new(HashSet::new())),
        Cluster::MainnetBeta,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(helius_websocket)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
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
