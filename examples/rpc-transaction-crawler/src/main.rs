mod env;
mod voting_program;

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
use env::Env;
use once_cell::sync::Lazy;
use serde::Deserialize;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{str::FromStr, time::Duration};
use voting_program::instructions::{VotingProgramInstruction, VotingProgramInstructionType};
use voting_program::VotingProgramDecoder;

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    let env = Env::new().expect("Failed to load environment settings");

    let test_account = Pubkey::from_str("ABVTNRYks9i8LkKQ439XRaRMbEjoFvWdmMzNLixj7BJm").unwrap();

    let filters = Filters::new(
        None, // No specific account filtering
        None, // No before signature
        None, // No until signature
    );

    let transaction_crawler = RpcTransactionCrawler::new(
        env.rpc_url,                         // RPC URL
        test_account,                        // The test account
        10,                                  // Batch limit
        Duration::from_secs(5),              // Polling interval
        filters,                             // Filters
        Some(CommitmentConfig::finalized()), // Commitment config
        20,                                  // Max Concurrent Requests
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(transaction_crawler)
        .transaction(CAST_VOTE_SCHEMA.clone(), CastVoteTransactionProcessor)
        .transaction(CREATE_VOTE_SCHEMA.clone(), CreateVoteTransactionProcessor)
        .instruction(VotingProgramDecoder, VotingInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
pub struct CastVoteOutput {
    pub cast_vote: AllInstructions,
}

pub struct CastVoteTransactionProcessor;

#[async_trait]
impl Processor for CastVoteTransactionProcessor {
    type InputType = CastVoteOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        println!("Matched transaction {:?}", data);
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateVoteOutput {
    pub create_vote: AllInstructions,
}

pub struct CreateVoteTransactionProcessor;

#[async_trait]
impl Processor for CreateVoteTransactionProcessor {
    type InputType = CreateVoteOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        println!("Matched transaction {:?}", data);
        Ok(())
    }
}

pub struct VotingInstructionProcessor;

#[async_trait]
impl Processor for VotingInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<VotingProgramInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        let (instruction_metadata, decoded_instruction, nested_instructions) = data;

        println!(
            "Matched instruction: {:?}",
            instruction_metadata.transaction_metadata.signature
        );

        match decoded_instruction.data {
            VotingProgramInstruction::CastVote(cast_vote) => {
                println!(
                    "Matched CastVote instruction with choice: {}",
                    cast_vote.choice
                );
            }
            VotingProgramInstruction::CreateVote(create_vote) => {
                println!(
                    "Matched CreateVote instruction with data: {:?}",
                    create_vote
                );
            }

            _ => {
                println!("Matched some other instruction type");
            }
        }
        Ok(())
    }
}

instruction_decoder_collection!(
    AllInstructions, AllInstructionTypes, AllPrograms,
    Voting => VotingProgramDecoder => VotingProgramInstruction
);

static CAST_VOTE_SCHEMA: Lazy<TransactionSchema<AllInstructions>> = Lazy::new(|| {
    schema![
        any
        [
            AllInstructionTypes::Voting(VotingProgramInstructionType::CastVote),
            "cast_vote",
            []
        ]
        any

    ]
});

static CREATE_VOTE_SCHEMA: Lazy<TransactionSchema<AllInstructions>> = Lazy::new(|| {
    schema![
        any
        [
            AllInstructionTypes::Voting(VotingProgramInstructionType::CreateVote),
            "create_vote",
            []
        ]
        any

    ]
});

