mod db;
mod env;
mod schema;
mod voting_program_decoder;
use async_trait::async_trait;
use carbon_core::deserialize::*;
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
        1,                                   // Batch limit
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

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        let (instruction_metadata, decoded_instruction, _nested_instructions) = data;

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

        // We are essentially skipping inner instructions,
        // as well as any instructions that have already been indexed in our database.
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
                let arranged_accounts =
                    match cast_vote.arrange_accounts(decoded_instruction.accounts) {
                        Some(accounts) => accounts,
                        None => {
                            return Ok(());
                        }
                    };

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

                // In our case, since we do not require any inner instructions,
                // we will create a record after processing the first instruction and skip any subsequent inner instructions.
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
                let arranged_accounts =
                    match create_vote.arrange_accounts(decoded_instruction.accounts) {
                        Some(accounts) => accounts,
                        None => {
                            return Ok(());
                        }
                    };

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

                // In our case, since we do not require any inner instructions,
                // we will create a record after processing the first instruction and skip any subsequent inner instructions.
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
