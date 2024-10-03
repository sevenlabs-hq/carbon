mod jupiter;
use async_trait::async_trait;
use carbon_core::instruction::{InstructionMetadata, NestedInstruction};
use carbon_core::processor::Processor;
use carbon_core::schema::{InstructionSchemaNode, SchemaNode, TransactionSchema};
use carbon_core::{
    collection::InstructionDecoderCollection,
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionDecoder},
};
use carbon_macros::*;
use carbon_proc_macros::instruction_decoder_collection;
use carbon_rpc_transaction_crawler_datasource::{Filters, RpcTransactionCrawler};
use jupiter::instructions::swap_event::SwapEvent;
use jupiter::instructions::{JupiterInstruction, JupiterInstructionType};
use jupiter::JupiterDecoder;
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
pub struct JupOutput {
    jup_swap_event: DecodedInstruction<JupiterInstruction>,
}

pub struct JupTransactionProcessor;

#[async_trait]
impl Processor for JupTransactionProcessor {
    type InputType = JupOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        println!("Matched Jupiter Swap Event transaction");

        if let JupiterInstruction::SwapEvent(SwapEvent {
            amm,
            input_mint,
            input_amount,
            output_mint,
            output_amount,
        }) = data.jup_swap_event.data
        {
            println!("AMM: {}", amm);
            println!("Input Mint: {}", input_mint);
            println!("Input Amount: {}", input_amount);
            println!("Output Mint: {}", output_mint);
            println!("Output Amount: {}", output_amount);
        }

        Ok(())
    }
}

pub struct JupInstructionProcessor;

#[async_trait]
impl Processor for JupInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<JupiterInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        println!("Matched Jupiter instruction: {:#?}", data);

        match data.1.data {
            JupiterInstruction::SwapEvent(SwapEvent {
                amm,
                input_mint,
                input_amount,
                output_mint,
                output_amount,
            }) => {
                println!("AMM: {}", amm);
                println!("Input Mint: {}", input_mint);
                println!("Input Amount: {}", input_amount);
                println!("Output Mint: {}", output_mint);
                println!("Output Amount: {}", output_amount);
            }
            JupiterInstruction::FeeEvent(_) => {}
            _ => {}
        };

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
    ]
});
