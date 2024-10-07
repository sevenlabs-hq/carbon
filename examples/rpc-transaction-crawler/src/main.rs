mod db;
mod env;
mod schema;
mod voting_program;
use async_trait::async_trait;
use carbon_core::deserialize::*;
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
use db::{init_pool, DbPool};
use diesel::prelude::*;
use dotenv::dotenv;
use env::Env;
use once_cell::sync::Lazy;
use schema::*;
use serde::Deserialize;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::collections::HashSet;
use std::{str::FromStr, time::Duration};
use voting_program::instructions::{VotingProgramInstruction, VotingProgramInstructionType};
use voting_program::VotingProgramDecoder;

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv().ok();

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

    let db_pool = init_pool(&env.database_url);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(transaction_crawler)
        //.transaction(CAST_VOTE_SCHEMA.clone(), CastVoteTransactionProcessor)
        //.transaction(CREATE_VOTE_SCHEMA.clone(), CreateVoteTransactionProcessor)
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

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        let (instruction_metadata, decoded_instruction, nested_instructions) = data;

        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(err) => {
                log::error!("Failed to consume datasource: {}", err);
                return Ok(());
            }
        };

        let signature = instruction_metadata
            .transaction_metadata
            .signature
            .to_string();

        let signature_exists: Option<String> = match activities::table
            .select(activities::signature)
            .filter(activities::signature.eq(&signature))
            .first(&mut conn)
            .optional()
        {
            Ok(result) => result,
            Err(e) => {
                log::error!("Error querying activities: {:?}", e);
                return Ok(());
            }
        };

        if signature_exists.is_some() {
            println!("Signature {} already processed, skipping.", signature);
            return Ok(());
        }

        match decoded_instruction.data {
            VotingProgramInstruction::CastVote(cast_vote) => {
                println!(
                    "Matched CreateVote instruction with data: {:?}, instruction_metadata: {:#?}, nested_instructions: {:#?}",
                    cast_vote,
                    instruction_metadata,
                    nested_instructions
                );

                let mut accounts_set: HashSet<Pubkey> = HashSet::new();

                for nested in nested_instructions.iter().rev() {
                    for account in &nested.instruction.accounts {
                        accounts_set.insert(account.pubkey);
                    }
                }

                let mut accounts: Vec<Pubkey> = accounts_set.into_iter().collect();

                if let Some(first_nested) = nested_instructions.first() {
                    accounts.push(first_nested.instruction.program_id);
                }

                if accounts.is_empty() {
                    log::warn!("No accounts found in nested instructions.");
                    return Ok(());
                }

                println!("{:#?}", accounts);

                let arranged_accounts = cast_vote
                    .arrange_accounts(accounts)
                    .expect("Failed to arrange accounts; no accounts were provided.");

                let insert_vote_result = diesel::insert_into(vote_entries::table)
                    .values((
                        vote_entries::voter_id.eq(arranged_accounts.voter.to_string()),
                        vote_entries::vote_id.eq(arranged_accounts.vote.to_string()),
                        vote_entries::choice.eq(cast_vote.choice),
                    ))
                    .execute(&mut conn);

                match insert_vote_result {
                    Ok(_) => println!("Successfully inserted vote."),
                    Err(e) => {
                        log::error!("Error inserting vote: {:?}", e);
                        return Ok(());
                    }
                }

                let vote_entries = vote_entries::table
                    .filter(vote_entries::vote_id.eq(arranged_accounts.vote.to_string()))
                    .load::<(String, String, bool)>(&mut conn)
                    .unwrap_or_default();

                let yes_count = vote_entries.iter().filter(|entry| entry.2).count();
                let no_count = vote_entries.iter().filter(|entry| !entry.2).count();

                let update_vote_result = diesel::update(
                    votes::table.filter(votes::vote_id.eq(arranged_accounts.vote.to_string())),
                )
                .set((
                    votes::yes.eq(yes_count as i32),
                    votes::no.eq(no_count as i32),
                ))
                .execute(&mut conn);

                match update_vote_result {
                    Ok(_) => println!("Successfully updated the votes."),
                    Err(e) => {
                        log::error!("Error to update votes: {:?}", e);
                        return Ok(());
                    }
                }

                let insert_signature_result = diesel::insert_into(activities::table)
                    .values(activities::signature.eq(&signature))
                    .execute(&mut conn);

                match insert_signature_result {
                    Ok(_) => println!("Successfully inserted the signature."),
                    Err(e) => {
                        log::error!("Error inserting signature: {:?}", e);
                        return Ok(());
                    }
                }

                println!("Vote counts updated: yes: {}, no: {}", yes_count, no_count);
            }

            VotingProgramInstruction::CreateVote(create_vote) => {
                println!(
                    "Matched CreateVote instruction with data: {:?}, instruction_metadata: {:#?}, nested_instructions: {:#?}",
                    create_vote,
                    instruction_metadata,
                    nested_instructions
                );

                let mut accounts_set: HashSet<Pubkey> = HashSet::new();

                for nested in nested_instructions.iter().rev() {
                    for account in &nested.instruction.accounts {
                        accounts_set.insert(account.pubkey);
                    }
                }

                let mut accounts: Vec<Pubkey> = accounts_set.into_iter().collect();

                if let Some(first_nested) = nested_instructions.first() {
                    accounts.push(first_nested.instruction.program_id);
                }

                if accounts.is_empty() {
                    log::warn!("No accounts found in nested instructions.");
                    return Ok(());
                }

                let arranged_accounts = create_vote
                    .arrange_accounts(accounts)
                    .expect("Failed to arrange accounts; no accounts were provided.");

                println!("arranged accounts {:#?}", arranged_accounts.vote);

                let insert_create_vote_result = diesel::insert_into(votes::table)
                    .values((
                        votes::vote_id.eq(arranged_accounts.vote.to_string()),
                        votes::authority.eq(arranged_accounts.authority.to_string()),
                        votes::yes.eq(0),
                        votes::no.eq(0),
                    ))
                    .execute(&mut conn);

                match insert_create_vote_result {
                    Ok(_) => println!("Successfully created vote."),
                    Err(e) => {
                        log::error!("Error inserting vote: {:?}", e);
                        return Ok(());
                    }
                }

                let insert_signature_result = diesel::insert_into(activities::table)
                    .values(activities::signature.eq(&signature))
                    .execute(&mut conn);

                match insert_signature_result {
                    Ok(_) => println!("Successfully inserted the signature."),
                    Err(e) => {
                        log::error!("Error inserting signature: {:?}", e);
                        return Ok(());
                    }
                }
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
