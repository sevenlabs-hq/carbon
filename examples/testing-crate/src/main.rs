use carbon_core::account::{AccountDecoder, AccountMetadata, DecodedAccount};
use paste::paste;
use solana_sdk::account::ReadableAccount;
use solana_sdk::program_pack::Pack;
use spl_token::instruction::TokenInstruction;
use std::time::Duration;

use async_trait::async_trait;
use carbon_core::instruction::InstructionMetadata;
use carbon_core::processor::Processor;
use carbon_core::transaction::ParsedTransaction;
use carbon_core::{
    collection::InstructionDecoderCollection,
    datasource::{Datasource, Update, UpdateType},
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionDecoder},
    schema::{SchemaNode, TransactionSchema},
};
use carbon_macros::{instruction_decoder_collection, ix, schema};

pub struct MockDatasource;

#[async_trait]
impl Datasource for MockDatasource {
    async fn consume(
        &self,
        sender: &tokio::sync::mpsc::UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let sender = sender.clone();
        let abort_handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                sender
                    .send(Update::Account(carbon_core::datasource::AccountUpdate {
                        pubkey: solana_sdk::pubkey::new_rand(),
                        account: solana_sdk::account::Account {
                            lamports: 0,
                            data: vec![],
                            owner: solana_sdk::pubkey::new_rand(),
                            executable: false,
                            rent_epoch: 0,
                        },
                        slot: 0,
                    }))
                    .unwrap();
                sender
                    .send(Update::Transaction(
                        carbon_core::datasource::TransactionUpdate {
                            signature: solana_sdk::signature::Signature::default(),
                            transaction: solana_sdk::transaction::VersionedTransaction::default(),
                            meta: solana_transaction_status::TransactionStatusMeta::default(),
                            is_vote: false,
                            slot: 0,
                        },
                    ))
                    .unwrap();
            }
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate, UpdateType::Transaction]
    }
}

pub struct TokenProgramAccountDecoder;
pub enum TokenProgramAccount {
    Account(spl_token::state::Account),
    Mint(spl_token::state::Mint),
    Multisig(spl_token::state::Multisig),
}

impl AccountDecoder for TokenProgramAccountDecoder {
    type AccountType = TokenProgramAccount;

    fn decode(
        &self,
        account: solana_sdk::account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner() != &spl_token::id() {
            return None;
        }

        if let Some(data) = spl_token::state::Account::unpack(account.data()).ok() {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Account(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Some(data) = spl_token::state::Mint::unpack(account.data()).ok() {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Mint(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Some(data) = spl_token::state::Multisig::unpack(account.data()).ok() {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Multisig(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };

        None
    }
}

pub struct TokenProgramAccountProcessor;
#[async_trait]
impl Processor for TokenProgramAccountProcessor {
    type InputType = (AccountMetadata, DecodedAccount<TokenProgramAccount>);

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        match data.1.data {
            TokenProgramAccount::Account(account) => {
                log::info!("Account: {:?}", account);
            }
            TokenProgramAccount::Mint(mint) => {
                log::info!("Mint: {:?}", mint);
            }
            TokenProgramAccount::Multisig(multisig) => {
                log::info!("Multisig: {:?}", multisig);
            }
        }
        log::info!("Account metadata: {:?}", data.0);

        Ok(())
    }
}

pub struct TokenProgramInstructionDecoder;
impl InstructionDecoder for TokenProgramInstructionDecoder {
    type InstructionType = TokenInstruction;

    fn decode(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        Some(DecodedInstruction {
            program_id: instruction.program_id,
            data: TokenInstruction::unpack(&instruction.data).ok()?,
        })
    }
}

pub struct TokenProgramInstructionProcessor;
#[async_trait]
impl Processor for TokenProgramInstructionProcessor {
    type InputType = (InstructionMetadata, DecodedInstruction<TokenInstruction>);

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Instruction: {:?}", data.1.data);
        log::info!("Instruction metadata: {:?}", data.0);

        Ok(())
    }
}

pub struct TokenProgramTransactionProcessor;
#[async_trait]
impl Processor for TokenProgramTransactionProcessor {
    type InputType = ParsedTransaction<AllInstructions>;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        // log::info!("Transaction: {:?}", data);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum MeteoraInstruction {
    Swap,
}

pub struct MeteoraInstructionDecoder;
impl InstructionDecoder for MeteoraInstructionDecoder {
    type InstructionType = MeteoraInstruction;

    fn decode(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        Some(DecodedInstruction {
            program_id: instruction.program_id,
            data: MeteoraInstruction::unpack(&instruction.data).ok()?,
        })
    }
}

pub struct MeteoraInstructionProcessor;
#[async_trait]
impl Processor for MeteoraInstructionProcessor {
    type InputType = (InstructionMetadata, DecodedInstruction<MeteoraInstruction>);

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Instruction: {:?}", data.1.data);
        log::info!("Instruction metadata: {:?}", data.0);

        Ok(())
    }
}

pub struct MeteoraTransactionProcessor;
#[async_trait]
impl Processor for MeteoraTransactionProcessor {
    type InputType = ParsedTransaction<AllInstructions>;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        // log::info!("Transaction: {:?}", data);

        Ok(())
    }
}

instruction_decoder_collection!(
    AllInstructions, AllInstructionTypes,
    TokenTransfer => TokenProgramInstructionDecoder => TokenInstruction,
    MeteoraSwap => MeteoraInstructionDecoder => MeteoraInstruction,
);

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let meteora_schema = schema![
        ix!(
            name = "meteora_swap_ix_1",
            type = AllInstructionTypes::MeteoraSwap,
            iixs = [
                ix!(name = "meteora_transfer_1", type = AllInstructionTypes::TokenTransfer),
                ix!(name = "meteora_transfer_2", type = AllInstructionTypes::TokenTransfer)
            ]
        ),
        ix!(
            name = "meteora_swap_ix_2",
            type = AllInstructionTypes::MeteoraSwap,
            iixs = [
                ix!(name = "meteora_transfer_1", type = AllInstructionTypes::TokenTransfer),
                ix!(name = "meteora_transfer_2", type = AllInstructionTypes::TokenTransfer)
            ]
        )
    ];

    carbon_core::pipeline::Pipeline::builder()
        .datasource(MockDatasource)
        .account(TokenProgramAccountDecoder, TokenProgramAccountProcessor)
        .transaction::<AllInstructions>(meteora_schema, MeteoraTransactionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}
