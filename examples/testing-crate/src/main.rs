use async_trait::async_trait;
use carbon_core::account::{AccountDecoder, AccountMetadata, DecodedAccount};
use carbon_core::block::{BlockDecoder, DecodedBlock};
use carbon_core::datasource::{BlockUpdate, SlotUpdate};
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
use carbon_macros::define_schema;
use carbon_macros::instruction_decoder_collection;
use carbon_macros::transaction_schema;
use solana_sdk::account::ReadableAccount;
use solana_sdk::program_option::COption;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status::{ConfirmedBlock, TransactionWithStatusMeta};
use spl_token::instruction::{AuthorityType, TokenInstruction};
use std::time::Duration;

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

// TEMP UNEFFICIENT SOLUTION
// pub struct TokenProgramInstructionDecoder;
// impl InstructionDecoder<'_> for TokenProgramInstructionDecoder {
//     type InstructionType = TokenInstruction<'static>;

//     fn decode(
//         &self,
//         instruction: solana_sdk::instruction::Instruction,
//     ) -> Option<DecodedInstruction<Self::InstructionType>> {
//         let data = instruction.data.clone();

//         // using Box::leak to make the data live for 'static
//         // CHECK: I'm afraid this will cause crashes and increase memory usage with time since "leaked"
//         // memory remains allocated indefinitely, so gotta find a better solution later
//         let boxed_data = Box::leak(Box::new(data));

//         let unpacked_instruction = TokenInstruction::unpack(boxed_data).ok()?;
//         Some(DecodedInstruction {
//             program_id: instruction.program_id,
//             data: unpacked_instruction,
//         })
//     }
// }

#[derive(Debug, Clone)]
pub enum OwnedTokenInstruction {
    InitializeMint {
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>,
    },
    InitializeAccount,
    InitializeMultisig {
        m: u8,
    },
    Transfer {
        amount: u64,
    },
    Approve {
        amount: u64,
    },
    Revoke,
    SetAuthority {
        authority_type: AuthorityType,
        new_authority: COption<Pubkey>,
    },
    MintTo {
        amount: u64,
    },
    Burn {
        amount: u64,
    },
    CloseAccount,
    FreezeAccount,
    ThawAccount,
    TransferChecked {
        amount: u64,
        decimals: u8,
    },
    ApproveChecked {
        amount: u64,
        decimals: u8,
    },
    MintToChecked {
        amount: u64,
        decimals: u8,
    },
    BurnChecked {
        amount: u64,
        decimals: u8,
    },
    InitializeAccount2 {
        owner: Pubkey,
    },
    SyncNative,
    InitializeAccount3 {
        owner: Pubkey,
    },
    InitializeMultisig2 {
        m: u8,
    },
    InitializeMint2 {
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>,
    },
    GetAccountDataSize,
    InitializeImmutableOwner,
    AmountToUiAmount {
        amount: u64,
    },
    UiAmountToAmount {
        ui_amount: String,
    },
}
impl OwnedTokenInstruction {
    pub fn from_token_instruction(instruction: TokenInstruction<'_>) -> Self {
        match instruction {
            TokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            } => OwnedTokenInstruction::InitializeMint {
                decimals,
                mint_authority,
                freeze_authority,
            },
            TokenInstruction::InitializeAccount => OwnedTokenInstruction::InitializeAccount,
            TokenInstruction::InitializeMultisig { m } => {
                OwnedTokenInstruction::InitializeMultisig { m }
            }
            TokenInstruction::Transfer { amount } => OwnedTokenInstruction::Transfer { amount },
            TokenInstruction::Approve { amount } => OwnedTokenInstruction::Approve { amount },
            TokenInstruction::Revoke => OwnedTokenInstruction::Revoke,
            TokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            } => OwnedTokenInstruction::SetAuthority {
                authority_type,
                new_authority,
            },
            TokenInstruction::MintTo { amount } => OwnedTokenInstruction::MintTo { amount },
            TokenInstruction::Burn { amount } => OwnedTokenInstruction::Burn { amount },
            TokenInstruction::CloseAccount => OwnedTokenInstruction::CloseAccount,
            TokenInstruction::FreezeAccount => OwnedTokenInstruction::FreezeAccount,
            TokenInstruction::ThawAccount => OwnedTokenInstruction::ThawAccount,
            TokenInstruction::TransferChecked { amount, decimals } => {
                OwnedTokenInstruction::TransferChecked { amount, decimals }
            }
            TokenInstruction::ApproveChecked { amount, decimals } => {
                OwnedTokenInstruction::ApproveChecked { amount, decimals }
            }
            TokenInstruction::MintToChecked { amount, decimals } => {
                OwnedTokenInstruction::MintToChecked { amount, decimals }
            }
            TokenInstruction::BurnChecked { amount, decimals } => {
                OwnedTokenInstruction::BurnChecked { amount, decimals }
            }
            TokenInstruction::InitializeAccount2 { owner } => {
                OwnedTokenInstruction::InitializeAccount2 { owner }
            }
            TokenInstruction::SyncNative => OwnedTokenInstruction::SyncNative,
            TokenInstruction::InitializeAccount3 { owner } => {
                OwnedTokenInstruction::InitializeAccount3 { owner }
            }
            TokenInstruction::InitializeMultisig2 { m } => {
                OwnedTokenInstruction::InitializeMultisig2 { m }
            }
            TokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            } => OwnedTokenInstruction::InitializeMint2 {
                decimals,
                mint_authority,
                freeze_authority,
            },
            TokenInstruction::GetAccountDataSize => OwnedTokenInstruction::GetAccountDataSize,
            TokenInstruction::InitializeImmutableOwner => {
                OwnedTokenInstruction::InitializeImmutableOwner
            }
            TokenInstruction::AmountToUiAmount { amount } => {
                OwnedTokenInstruction::AmountToUiAmount { amount }
            }
            TokenInstruction::UiAmountToAmount { ui_amount } => {
                OwnedTokenInstruction::UiAmountToAmount {
                    ui_amount: ui_amount.to_string(),
                }
            }
        }
    }
}
pub struct TokenProgramInstructionDecoder;
impl InstructionDecoder for TokenProgramInstructionDecoder {
    type InstructionType = OwnedTokenInstruction;

    fn decode(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        let unpacked_instruction = TokenInstruction::unpack(&instruction.data).ok()?;
        let owned_instruction = OwnedTokenInstruction::from_token_instruction(unpacked_instruction);

        Some(DecodedInstruction {
            program_id: instruction.program_id,
            data: owned_instruction,
        })
    }
}

pub struct TokenProgramInstructionProcessor;
#[async_trait]
impl Processor for TokenProgramInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<TokenInstruction<'static>>,
    );

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Instruction: {:?}", data.1.data);
        log::info!("Instruction metadata: {:?}", data.0);

        Ok(())
    }
}

instruction_decoder_collection!(
    AllInstructions, AllInstructionTypes,
    Token => TokenProgramInstructionDecoder => OwnedTokenInstruction
);

pub struct TokenProgramTransactionProcessor;
#[async_trait]
impl Processor for TokenProgramTransactionProcessor {
    type InputType = ParsedTransaction<AllInstructions>;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Transaction: {:?}", data);

        Ok(())
    }
}

pub struct ConfirmedBlockDecoder;
impl BlockDecoder for ConfirmedBlockDecoder {
    type BlockType = DecodedBlock;

    fn decode(&self, block: &BlockUpdate) -> Option<Self::BlockType> {
        let versioned_txs: Vec<_> = block
            .transactions
            .iter()
            .map(|tx| tx.get_transaction())
            .collect();

        Some(DecodedBlock {
            slot: block.parent_slot - 1,
            block_hash: block.blockhash.clone(),
            block_timestamp: block.block_time.unwrap_or_default(),
            previous_block_hash: block.previous_blockhash.clone(),
            transaction_count: block.transactions.len() as u64,
            transactions: Some(versioned_txs),
        })
    }
}

pub struct BlockProcessor;
#[async_trait]
impl Processor for BlockProcessor {
    type InputType = DecodedBlock;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Processing Decoded Block: {:?}", data);
        Ok(())
    }
}

pub struct SlotProcessor;

#[async_trait]
impl Processor for SlotProcessor {
    // TODO: upd
    type InputType = SlotUpdate;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        log::info!("Processing Slot: {:?}", data);
        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    carbon_core::pipeline::Pipeline::builder()
        .datasource(MockDatasource)
        .account(TokenProgramAccountDecoder, TokenProgramAccountProcessor)
        .transaction::<AllInstructions>(transaction_schema!(any), TokenProgramTransactionProcessor)
        .block(ConfirmedBlockDecoder, BlockProcessor)
        .slot(SlotProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}
