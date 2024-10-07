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
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use jupiter::instructions::{JupiterInstruction, JupiterInstructionType};
use jupiter::JupiterDecoder;
use once_cell::sync::Lazy;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::{str::FromStr, time::Duration};
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let test_account = Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap();

    let account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),   // Example: set to false if you don't want to filter by votes
        failed: Some(false), // Example: set to false to exclude failed transactions
        account_include: vec![test_account.to_string().clone()], //
        account_exclude: vec![], // Optionally, exclude any accounts if needed
        account_required: vec![], // Optionally, specify required accounts if needed
        signature: None,     // Optional: specify a signature if needed
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("test_account_filter".to_string(), transaction_filter);

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        "".to_string(),                        // RPC URL
        None,                                  // X Token
        Some(CommitmentLevel::Confirmed),      // Commitment Level
        account_filters,                       // Account filters
        transaction_filters,                   // Transaction filters
        Arc::new(RwLock::new(HashSet::new())), // Track deletions
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .transaction(JUPITER_SCHEMA.clone(), JupTransactionProcessor)
        .instruction(JupiterDecoder, JupInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
pub struct JupOutput {
    pub jup_swap_event: JupiterInstruction,
}

pub struct JupTransactionProcessor;

#[async_trait]
impl Processor for JupTransactionProcessor {
    type InputType = JupOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Output: {:?}", data);
        println!("Matched Jupiter transaction: {:#?}", data);
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
