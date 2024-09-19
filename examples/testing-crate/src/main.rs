use carbon_core::account::{AccountDecoder, AccountMetadata, DecodedAccount};
use carbon_core::schema::{InstructionSchemaNode, SchemaNode, TransactionSchema};
use serde::Deserialize;
use solana_sdk::account::ReadableAccount;
use solana_sdk::program_pack::Pack;

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
};
pub use carbon_macros::*;

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

// pub struct TokenProgramInstructionDecoder;
// impl InstructionDecoder for TokenProgramInstructionDecoder {
//     type InstructionType = TokenInstruction;

//     fn decode(
//         &self,
//         instruction: solana_sdk::instruction::Instruction,
//     ) -> Option<DecodedInstruction<Self::InstructionType>> {
//         Some(DecodedInstruction {
//             program_id: instruction.program_id,
//             data: TokenInstruction::unpack(&instruction.data).ok()?,
//         })
//     }
// }

// pub struct TokenProgramInstructionProcessor;
// #[async_trait]
// impl Processor for TokenProgramInstructionProcessor {
//     type InputType = (InstructionMetadata, DecodedInstruction<TokenInstruction>);

//     async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
//         log::info!("Instruction: {:?}", data.1.data);
//         log::info!("Instruction metadata: {:?}", data.0);

//         Ok(())
//     }
// }

pub struct TokenProgramTransactionProcessor;
#[async_trait]
impl Processor for TokenProgramTransactionProcessor {
    type InputType = ParsedTransaction<AllInstructions>;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        // log::info!("Transaction: {:?}", data);

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Discriminator {
    OneByte(u8),
    TwoBytes([u8; 2]),
    FourBytes([u8; 4]),
    EightBytes([u8; 8]),
    SixteenBytes([u8; 16]),
}

#[allow(dead_code)]
impl Discriminator {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self.clone() {
            Discriminator::OneByte(d) => std::slice::from_ref(&d).to_vec(),
            Discriminator::TwoBytes(d) => d.to_vec(),
            Discriminator::FourBytes(d) => d.to_vec(),
            Discriminator::EightBytes(d) => d.to_vec(),
            Discriminator::SixteenBytes(d) => d.to_vec(),
        }
    }

    pub fn one_byte_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 1 {
            return Err(carbon_core::error::Error::MissingInstructionData);
        }
        Ok(Discriminator::OneByte(data[0]))
    }

    pub fn two_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 2 {
            return Err(carbon_core::error::Error::MissingInstructionData);
        }
        let mut buf = [0u8; 2];
        buf.copy_from_slice(&data[..2]);
        Ok(Discriminator::TwoBytes(buf))
    }

    pub fn four_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 4 {
            return Err(carbon_core::error::Error::MissingInstructionData);
        }
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&data[..4]);
        Ok(Discriminator::FourBytes(buf))
    }

    pub fn eight_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 8 {
            return Err(carbon_core::error::Error::MissingInstructionData);
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&data[..8]);
        Ok(Discriminator::EightBytes(buf))
    }

    pub fn sixteen_bytes_from_slice(data: &[u8]) -> CarbonResult<Self> {
        if data.len() < 16 {
            return Err(carbon_core::error::Error::MissingInstructionData);
        }
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&data[..16]);
        Ok(Discriminator::SixteenBytes(buf))
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, serde::Serialize)]
pub enum MeteoraInstruction {
    Swap,
}

impl MeteoraInstruction {
    // Filler whatever
    // pub fn unpack(input: &[u8]) -> CarbonResult<Self> {
    //     Ok(Self::Swap)
    // }

    pub fn unpack(input: &[u8]) -> CarbonResult<MeteoraInstruction> {
        // let discriminator = match input.len() {
        //     _ => Discriminator::eight_bytes_from_slice(input)?,
        // };
        // let ix = match discriminator {
        //     discriminator if discriminator == MeteoraSwapInstructionData::discriminator() => {
        //         MeteoraInstruction::Swap {
        //             data: InstructionData::unpack(input).map_err(Error::msg)?,
        //             accounts: MeteoraSwapInstructionAccounts::unpack(input).map_err(Error::msg)?,
        //         }
        //     }
        //     discriminator if discriminator == MeteoraSwapEventInstructionData::discriminator() => {
        //         MeteoraInstruction::SwapEvent {
        //             data: InstructionData::unpack(input).map_err(Error::msg)?,
        //             accounts: MeteoraSwapEventInstructionAccounts::unpack(input)
        //                 .map_err(Error::msg)?,
        //         }
        //     }
        //     _discriminator => bail!("Invalid meteora instruction discriminator".to_owned()),
        // };
        Ok(MeteoraInstruction::Swap)
    }
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

#[derive(Clone, Debug, Deserialize)]
pub struct MeteoraOutput {}

pub struct MeteoraTransactionProcessor;
#[async_trait]
impl Processor for MeteoraTransactionProcessor {
    type InputType = MeteoraOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Output: {:?}", data);

        Ok(())
    }
}

instruction_decoder_collection!(
    AllInstructions, AllInstructionTypes,
    // TokenTransfer => TokenProgramInstructionDecoder => TokenInstruction,
    MeteoraSwap => MeteoraInstructionDecoder => MeteoraInstruction,
    MeteoraTransfer => MeteoraInstructionDecoder => MeteoraInstruction,
);

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let schema: TransactionSchema<AllInstructions> = schema![
        any
        [
            AllInstructionTypes::MeteoraSwap,
            "meteora_swap_ix_1",
            []
        ]
        any
    ];

    carbon_core::pipeline::Pipeline::builder()
        .datasource(MockDatasource)
        // .account(TokenProgramAccountDecoder, TokenProgramAccountProcessor)
        .transaction(schema, MeteoraTransactionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}
