use async_trait::async_trait;
use carbon_core::instruction::InstructionMetadata;
use carbon_core::processor::Processor;
use carbon_core::schema::{InstructionSchemaNode, SchemaNode, TransactionSchema};
use carbon_core::{
    collection::InstructionDecoderCollection,
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionDecoder},
};
pub use carbon_macros::*;
use carbon_proc_macros::instruction_decoder_collection;
use carbon_rpc_transaction_crawler_datasource::{Filters, RpcTransactionCrawler};
use jupiter_decoder::instructions::{JupiterInstruction, JupiterInstructionType};
use jupiter_decoder::JupiterDecoder;
use once_cell::sync::Lazy;
use serde::Deserialize;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{str::FromStr, time::Duration};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let test_account = Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap();

    let filters = Filters::new(
        None, // No specific account filtering
        None, // No before signature
        None, // No until signature
    );

    let transaction_crawler = RpcTransactionCrawler::new(
        "https://api.mainnet-beta.solana.com".to_string(), // RPC URL
        test_account,                                      // The test account
        1,                                                 // Batch limit
        Duration::from_secs(5),                            // Polling interval
        filters,                                           // Filters
        Some(CommitmentConfig::finalized()),               // Commitment config
        20,                                                // Max Concurrent Requests
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(transaction_crawler)
        .transaction(JUPITER_SCHEMA.clone(), JupTransactionProcessor)
        .instruction(JupiterDecoder, JupInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
pub struct JupOutput {}

pub struct JupTransactionProcessor;

#[async_trait]
impl Processor for JupTransactionProcessor {
    type InputType = JupOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Output: {:?}", data);
        println!("Matched Jupiter transaction");
        Ok(())
    }
}

pub struct JupInstructionProcessor;

#[async_trait]
impl Processor for JupInstructionProcessor {
    type InputType = (InstructionMetadata, DecodedInstruction<JupiterInstruction>);

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        println!("Matched Jupiter instruction: {:#?}", data);

        Ok(())
    }
}

instruction_decoder_collection!(
    AllInstructions, AllInstructionTypes, AllPrograms,
    JupSwap => JupiterDecoder => JupiterInstruction
);

static JUPITER_SCHEMA: Lazy<TransactionSchema<AllInstructions>> = Lazy::new(|| {
    schema![
        any
        [
           AllInstructionTypes::JupSwap(JupiterInstructionType::SwapEvent),
           "jup_swap_event",
            []
        ]
        any
    ]
});
