mod db;
mod env;
mod schema;
mod voting_program_decoder;
use async_trait::async_trait;
use carbon_core::error::Error;
use carbon_core::instruction::{InstructionMetadata, NestedInstruction};
use carbon_core::processor::Processor;
use carbon_core::{error::CarbonResult, instruction::DecodedInstruction};
use carbon_rpc_transaction_crawler_datasource::{Filters, RpcTransactionCrawler};
use db::{init_pool, DbPool};
use diesel::prelude::*;
use dotenv::dotenv;
use env::Env;
use schema::*;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{str::FromStr, time::Duration};
use voting_program_decoder::instructions::VotingProgramInstruction;
use voting_program_decoder::VotingProgramDecoder;

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv().ok();

    let env = Env::new().expect("Failed to load environment settings");

    let test_account = Pubkey::from_str("ABVTNRYks9i8LkKQ439XRaRMbEjoFvWdmMzNLixj7BJm").unwrap(); // Voting Program Address

    let filters = Filters::new(
        None, // No specific account filtering
        None, // No before signature
        None, // No until signature
    );

    let transaction_crawler = RpcTransactionCrawler::new(
        env.rpc_url,                         // RPC URL
        test_account,                        // The test account
        100,                                 // Batch limit
        Duration::from_secs(5),              // Polling interval
        filters,                             // Filters
        Some(CommitmentConfig::finalized()), // Commitment config
        20,                                  // Max Concurrent Requests
    );

    let db_pool = init_pool(&env.database_url);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(transaction_crawler)
        .instruction(
            VotingProgramDecoder,
            VotingInstructionProcessor {
                db_pool: db_pool.clone(),
            },
        )
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct VotingInstructionProcessor {
    pub db_pool: DbPool,
}

#[async_trait]
impl Processor for VotingInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<VotingProgramInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(&mut self, data: Self::InputType) -> CarbonResult<()> {
        let (instruction_metadata, decoded_instruction, _nested_instructions) = data;

        let mut conn = self
            .db_pool
            .get()
            .map_err(|err| Error::Custom(format!("Failed to connect db pool: {}", err)))?;

        let signature = instruction_metadata
            .transaction_metadata
            .signature
            .to_string();

        let signature_exists: Option<String> = activities::table
            .select(activities::signature)
            .filter(activities::signature.eq(&signature))
            .first(&mut conn)
            .optional()
            .map_err(|err| Error::Custom(format!("Failed to querying activities: {}", err)))?;

        if signature_exists.is_some() {
            // println!("Signature {} already processed, skipping.", signature);
            return Ok(());
        }

        match decoded_instruction.data {
            VotingProgramInstruction::CastVote(_cast_vote) => {
                diesel::insert_into(activities::table)
                    .values((
                        activities::signature.eq(&signature),
                        activities::activity_type.eq("CastVote".to_string()),
                    ))
                    .execute(&mut conn)
                    .map_err(|err| Error::Custom(format!("Failed to insert activity: {}", err)))?;

                println!("Successfully inserted CastVote activity.");
            }

            VotingProgramInstruction::CreateVote(_create_vote) => {
                diesel::insert_into(activities::table)
                    .values((
                        activities::signature.eq(&signature),
                        activities::activity_type.eq("CreateVote".to_string()),
                    ))
                    .execute(&mut conn)
                    .map_err(|err| Error::Custom(format!("Failed to insert activity: {}", err)))?;

                println!("Successfully inserted CreateVote activity.");
            }

            _ => {
                println!("Matched some other instruction type");
            }
        }
        Ok(())
    }
}
