use async_trait::async_trait;
use carbon_core::{
    account::{AccountMetadata, DecodedAccount},
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    metrics::MetricsCollection,
    processor::Processor,
};
use carbon_raydium_clmm_decoder::{
    accounts::RaydiumClmmAccount, instructions::RaydiumClmmInstruction, RaydiumClmmDecoder,
};
use solana_sdk::{pubkey, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
use tokio::sync::RwLock;

pub const RAYDIUM_CLMM_PROGRAM_ID: Pubkey = pubkey!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    Ok(())
}
