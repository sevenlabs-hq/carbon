use carbon_core::account::{AccountDecoder, AccountMetadata, DecodedAccount};
use carbon_core::datasource::TransactionUpdate;
use carbon_core::metrics::Metrics;
use carbon_core::schema::{InstructionSchemaNode, SchemaNode, TransactionSchema};
use carbon_proc_macros::{instruction_decoder_collection, InstructionType};
// use investment_watches_program_decoder::instructions::InvestmentWatchesProgramInstruction;
// use jupiter_decoder::instructions::{JupiterInstruction, JupiterInstructionType};
// use jupiter_decoder::JupiterDecoder;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::account::ReadableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::CompiledInstruction;
use solana_sdk::message::v0::LoadedAddresses;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    InnerInstruction, InnerInstructions, Reward, TransactionStatusMeta, TransactionTokenBalance,
    UiInstruction,
};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
//use whirlpool_decoder::instructions::WhirlpoolInstruction;
//use whirlpool_decoder::WhirlpoolDecoder;

use async_trait::async_trait;
use carbon_core::instruction::{InstructionMetadata, NestedInstruction};
use carbon_core::processor::Processor;
use carbon_core::transaction::ParsedTransaction;
use carbon_core::{
    collection::InstructionDecoderCollection,
    datasource::{Datasource, Update, UpdateType},
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionDecoder},
};
pub use carbon_macros::*;
use once_cell::sync::Lazy;

pub struct TestDatasource;

#[async_trait]
impl Datasource for TestDatasource {
    async fn consume(
        &self,
        sender: &tokio::sync::mpsc::UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let sender = sender.clone();

        let rpc_client = RpcClient::new_with_commitment(
            "https://api.mainnet-beta.solana.com".to_string(),
            CommitmentConfig::confirmed(),
        );

        let tx = rpc_client.get_transaction_with_config(&Signature::from_str("3fKfV8CUSGr9exbWzFo7EwkTAWbK6Ymg5w5g2grtcru9DaeH8717jWpaUKPckBwKczghgTXYPSZfCfxBLYgHHx1p").unwrap(), RpcTransactionConfig {
            max_supported_transaction_version: Some(2),
            encoding: Some(solana_transaction_status::UiTransactionEncoding::Base58),
            commitment: Some(CommitmentConfig::confirmed())
        }).ok().unwrap();

        let meta_original = tx.transaction.meta.unwrap();

        let meta_needed = TransactionStatusMeta {
            status: meta_original.status,
            fee: meta_original.fee,
            pre_balances: meta_original.pre_balances,
            post_balances: meta_original.post_balances,
            log_messages: Some(meta_original.log_messages.unwrap()),
            pre_token_balances: Some(
                meta_original
                    .pre_token_balances
                    .unwrap()
                    .iter()
                    .map(|tok| TransactionTokenBalance {
                        account_index: tok.account_index,
                        mint: tok.mint.clone(),
                        ui_token_amount: tok.ui_token_amount.clone(),
                        owner: tok.owner.clone().unwrap(),
                        program_id: tok.program_id.clone().unwrap(),
                    })
                    .collect::<Vec<TransactionTokenBalance>>(),
            ),
            inner_instructions: Some(
                meta_original
                    .inner_instructions
                    .unwrap()
                    .iter()
                    .map(|iixs| InnerInstructions {
                        index: iixs.index,
                        instructions: iixs
                            .instructions
                            .iter()
                            .map(|iix| match iix {
                                UiInstruction::Compiled(ui_compiled_instruction) => {
                                    InnerInstruction {
                                        instruction: CompiledInstruction {
                                            program_id_index: ui_compiled_instruction
                                                .program_id_index,
                                            accounts: ui_compiled_instruction.accounts.clone(),
                                            data: bs58::decode(
                                                ui_compiled_instruction.data.clone(),
                                            )
                                            .into_vec()
                                            .unwrap(),
                                        },
                                        stack_height: ui_compiled_instruction.stack_height,
                                    }
                                }
                                _ => {
                                    panic!("unimplemented instruction type");
                                }
                            })
                            .collect::<Vec<InnerInstruction>>(),
                    })
                    .collect::<Vec<InnerInstructions>>(),
            ),
            post_token_balances: Some(
                meta_original
                    .post_token_balances
                    .unwrap()
                    .iter()
                    .map(|ptb| TransactionTokenBalance {
                        account_index: ptb.account_index,
                        mint: ptb.mint.clone(),
                        ui_token_amount: ptb.ui_token_amount.clone(),
                        owner: ptb.owner.clone().unwrap(),
                        program_id: ptb.program_id.clone().unwrap(),
                    })
                    .collect::<Vec<TransactionTokenBalance>>(),
            ),
            rewards: Some(
                meta_original
                    .rewards
                    .unwrap()
                    .iter()
                    .map(|rewards| Reward {
                        pubkey: rewards.pubkey.clone(),
                        lamports: rewards.lamports,
                        post_balance: rewards.post_balance,
                        reward_type: rewards.reward_type,
                        commission: rewards.commission,
                    })
                    .collect::<Vec<Reward>>(),
            ),
            loaded_addresses: {
                let loaded = meta_original.loaded_addresses.unwrap();
                LoadedAddresses {
                    writable: loaded
                        .writable
                        .iter()
                        .map(|w| Pubkey::from_str(&w).unwrap())
                        .collect::<Vec<Pubkey>>(),
                    readonly: loaded
                        .readonly
                        .iter()
                        .map(|r| Pubkey::from_str(&r).unwrap())
                        .collect::<Vec<Pubkey>>(),
                }
            },
            return_data: None,
            compute_units_consumed: Some(meta_original.compute_units_consumed.unwrap()),
        };

        let update = Update::Transaction(TransactionUpdate {
            signature: Signature::from_str("3fKfV8CUSGr9exbWzFo7EwkTAWbK6Ymg5w5g2grtcru9DaeH8717jWpaUKPckBwKczghgTXYPSZfCfxBLYgHHx1p").unwrap(),
            transaction: tx.transaction.transaction.decode().unwrap(),
            meta: meta_needed,
            is_vote: false,
            slot: tx.slot,
        });

        let abort_handle = tokio::spawn(async move {
            sender.send(update).unwrap();
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}

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

// Token Test Start

pub struct TokenProgramAccountDecoder;
pub enum TokenProgramAccount {
    Account(spl_token::state::Account),
    Mint(spl_token::state::Mint),
    Multisig(spl_token::state::Multisig),
}

impl<'a> AccountDecoder<'a> for TokenProgramAccountDecoder {
    type AccountType = TokenProgramAccount;

    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
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

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()> {
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
// impl<'a> InstructionDecoder<'a> for TokenProgramInstructionDecoder {
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

/*
pub struct TokenProgramTransactionProcessor;
#[async_trait]
impl Processor for TokenProgramTransactionProcessor {
    type InputType = ParsedTransaction<AllInstructions>;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        // log::info!("Transaction: {:?}", data);

        Ok(())
    }
}


// Token Test End

// Meteora Test Start

#[derive(Debug, Clone, Eq, Hash, PartialEq, serde::Serialize, InstructionType)]
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
impl<'a> InstructionDecoder<'a> for MeteoraInstructionDecoder {
    type InstructionType = MeteoraInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if instruction.program_id
            == Pubkey::from_str("mFivYY5xPoh3rDCxbdwzkgN1Rv2kC9s9kEpHHsnUWtf").unwrap()
        {
            Some(DecodedInstruction {
                program_id: instruction.program_id,
                data: MeteoraInstruction::unpack(&instruction.data).ok()?,
            })
        } else {
            None
        }
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

        println!("Matched meteora");

        Ok(())
    }
}

// Meteora Test End

// Orca Test Start


#[derive(Debug, Clone, Eq, Hash, PartialEq, serde::Serialize, InstructionType)]
pub enum OrcaInstruction {
    Swap,
}

impl OrcaInstruction {
    pub fn unpack(_input: &[u8]) -> CarbonResult<OrcaInstruction> {
        Ok(OrcaInstruction::Swap)
    }
}

pub struct OrcaInstructionDecoder;
impl<'a> InstructionDecoder<'a> for OrcaInstructionDecoder {
    type InstructionType = OrcaInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if instruction.program_id == Pubkey::default() {
            Some(DecodedInstruction {
                program_id: instruction.program_id,
                data: OrcaInstruction::unpack(&instruction.data).ok()?,
            })
        } else {
            None
        }
    }
}

/*
#[derive(Clone, Debug, Deserialize)]
pub struct OrcaOutput {}

pub struct OrcaTransactionProcessor;
#[async_trait]
impl Processor for OrcaTransactionProcessor {
    type InputType = OrcaOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Output: {:?}", data);

        println!("Matched orca");
        Ok(())
    }
}


// Orca Test End


instruction_decoder_collection!(
    AllInstructions, AllInstructionTypes, AllPrograms,
    // MeteoraSwap => MeteoraInstructionDecoder => MeteoraInstruction,
    // OrcaSwap => OrcaInstructionDecoder => OrcaInstruction,
    JupSwap => JupiterDecoder => JupiterInstruction,
    // RaydiumClmm => AmmV3Decoder => AmmV3Instruction,
);

static JUPITER_SCHEMA: Lazy<TransactionSchema<AllInstructions>> = Lazy::new(|| {
    schema![
        any
        [
            AllInstructionTypes::JupSwap(JupiterInstructionType::SharedAccountsRoute),
            "jup_swap_event_ix_1",
            []
        ]
        any
    ]
});
*/
// define_schema_output_accounts!(OrcaOutput, JUPITER_SCHEMA);

 */

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    carbon_core::pipeline::Pipeline::builder()
        .datasource(TestDatasource)
        //.account(JupiterDecoder, JupiterAccountProcessor)
        //.instruction(JupiterDecoder, JupiterInstructionProcessor)
        // .transaction(JUPITER_SCHEMA.clone(), OrcaTransactionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}
