//! Provides utility functions to transform transaction data into various
//! representations within the `carbon-core` framework.
//!
//! This module includes functions for extracting transaction metadata, parsing
//! instructions, and nesting instructions based on stack depth. It also offers
//! transformations for Solana transaction components into suitable formats for
//! the framework, enabling flexible processing of transaction data.
//!
//! ## Key Components
//!
//! - **Metadata Extraction**: Extracts essential transaction metadata for
//!   processing.
//! - **Instruction Parsing**: Parses both top-level and nested instructions
//!   from transactions.
//! - **Account Metadata**: Converts account data into a standardized format for
//!   transactions.
//!
//! ## Notes
//!
//! - The module supports both legacy and v0 transactions, including handling of
//!   loaded addresses and inner instructions.

use {
    crate::{
        collection::InstructionDecoderCollection,
        datasource::TransactionUpdate,
        error::{CarbonResult, Error},
        instruction::{DecodedInstruction, InstructionMetadata},
        schema::ParsedInstruction,
        transaction::TransactionMetadata,
    },
    solana_instruction::AccountMeta,
    solana_program::{
        instruction::CompiledInstruction,
        message::{
            v0::{LoadedAddresses, LoadedMessage},
            VersionedMessage,
        },
    },
    solana_pubkey::Pubkey,
    solana_sdk::{
        reserved_account_keys::ReservedAccountKeys,
        transaction_context::TransactionReturnData, // TODO: replace with solana_transaction_context after release of 2.2.0
    },
    solana_transaction_status::{
        option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
        TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiLoadedAddresses,
        UiTransactionStatusMeta,
    },
    std::{collections::HashSet, str::FromStr},
};

/// Extracts instructions with metadata from a transaction update.
///
/// This function parses both top-level and inner instructions, associating them
/// with metadata such as stack height and account information. It provides a
/// detailed breakdown of each instruction, useful for further processing.
///
/// # Parameters
///
/// - `transaction_metadata`: Metadata about the transaction from which
///   instructions are extracted.
/// - `transaction_update`: The `TransactionUpdate` containing the transaction's
///   data and message.
///
/// # Returns
///
/// A `CarbonResult<Vec<(InstructionMetadata,
/// solana_instruction::Instruction)>>` containing instructions along with
/// their associated metadata.
///
/// # Errors
///
/// Returns an error if any account metadata required for instruction processing
/// is missing.
pub fn extract_instructions_with_metadata(
    transaction_metadata: &TransactionMetadata,
    transaction_update: &TransactionUpdate,
) -> CarbonResult<Vec<(InstructionMetadata, solana_instruction::Instruction)>> {
    log::trace!(
        "extract_instructions_with_metadata(transaction_metadata: {:?}, transaction_update: {:?})",
        transaction_metadata,
        transaction_update
    );
    let message = transaction_update.transaction.message.clone();
    let meta = transaction_update.meta.clone();

    let mut instructions_with_metadata =
        Vec::<(InstructionMetadata, solana_instruction::Instruction)>::new();

    match message {
        VersionedMessage::Legacy(legacy) => {
            for (i, compiled_instruction) in legacy.instructions.iter().enumerate() {
                let program_id = *legacy
                    .account_keys
                    .get(compiled_instruction.program_id_index as usize)
                    .unwrap_or(&Pubkey::default());

                let accounts: Vec<_> = compiled_instruction
                    .accounts
                    .iter()
                    .filter_map(|account_index| {
                        let account_pubkey = legacy.account_keys.get(*account_index as usize)?;
                        Some(AccountMeta {
                            pubkey: *account_pubkey,
                            is_writable: legacy.is_maybe_writable(*account_index as usize, None),
                            is_signer: legacy.is_signer(*account_index as usize),
                        })
                    })
                    .collect();

                instructions_with_metadata.push((
                    InstructionMetadata {
                        transaction_metadata: transaction_metadata.clone(),
                        stack_height: 1,
                        index: i as u32 + 1,
                    },
                    solana_instruction::Instruction {
                        program_id,
                        accounts,
                        data: compiled_instruction.data.clone(),
                    },
                ));

                if let Some(inner_instructions) = &meta.inner_instructions {
                    for inner_instructions_per_tx in inner_instructions {
                        if inner_instructions_per_tx.index == i as u8 {
                            for (inner_ix_idx, inner_instruction) in
                                inner_instructions_per_tx.instructions.iter().enumerate()
                            {
                                let program_id = *legacy
                                    .account_keys
                                    .get(inner_instruction.instruction.program_id_index as usize)
                                    .unwrap_or(&Pubkey::default());

                                let accounts: Vec<_> = inner_instruction
                                    .instruction
                                    .accounts
                                    .iter()
                                    .filter_map(|account_index| {
                                        let account_pubkey =
                                            legacy.account_keys.get(*account_index as usize)?;

                                        Some(AccountMeta {
                                            pubkey: *account_pubkey,
                                            is_writable: legacy
                                                .is_maybe_writable(*account_index as usize, None),
                                            is_signer: legacy.is_signer(*account_index as usize),
                                        })
                                    })
                                    .collect();

                                instructions_with_metadata.push((
                                    InstructionMetadata {
                                        transaction_metadata: transaction_metadata.clone(),
                                        stack_height: inner_instruction.stack_height.unwrap_or(1),
                                        index: inner_ix_idx as u32 + 1,
                                    },
                                    solana_instruction::Instruction {
                                        program_id,
                                        accounts,
                                        data: inner_instruction.instruction.data.clone(),
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }
        VersionedMessage::V0(v0) => {
            let loaded_addresses = LoadedAddresses {
                writable: meta.loaded_addresses.writable.to_vec(),
                readonly: meta.loaded_addresses.readonly.to_vec(),
            };

            let loaded_message = LoadedMessage::new(
                v0.clone(),
                loaded_addresses,
                &ReservedAccountKeys::empty_key_set(),
            );

            for (i, compiled_instruction) in v0.instructions.iter().enumerate() {
                let program_id = *loaded_message
                    .account_keys()
                    .get(compiled_instruction.program_id_index as usize)
                    .unwrap_or(&Pubkey::default());

                let accounts: Vec<AccountMeta> = compiled_instruction
                    .accounts
                    .iter()
                    .map(|account_index| {
                        let account_pubkey =
                            loaded_message.account_keys().get(*account_index as usize);

                        AccountMeta {
                            pubkey: account_pubkey.copied().unwrap_or_default(),
                            is_writable: loaded_message.is_writable(*account_index as usize),
                            is_signer: loaded_message.is_signer(*account_index as usize),
                        }
                    })
                    .collect();

                instructions_with_metadata.push((
                    InstructionMetadata {
                        transaction_metadata: transaction_metadata.clone(),
                        stack_height: 1,
                        index: i as u32 + 1,
                    },
                    solana_instruction::Instruction {
                        program_id,
                        accounts,
                        data: compiled_instruction.data.clone(),
                    },
                ));

                if let Some(inner_instructions) = &meta.inner_instructions {
                    for inner_instructions_per_tx in inner_instructions {
                        if inner_instructions_per_tx.index == i as u8 {
                            for (inner_ix_idx, inner_instruction) in
                                inner_instructions_per_tx.instructions.iter().enumerate()
                            {
                                let program_id = *loaded_message
                                    .account_keys()
                                    .get(inner_instruction.instruction.program_id_index as usize)
                                    .unwrap_or(&Pubkey::default());

                                let accounts: Vec<AccountMeta> = inner_instruction
                                    .instruction
                                    .accounts
                                    .iter()
                                    .map(|account_index| {
                                        let account_pubkey = loaded_message
                                            .account_keys()
                                            .get(*account_index as usize)
                                            .copied()
                                            .unwrap_or_default();

                                        AccountMeta {
                                            pubkey: account_pubkey,
                                            is_writable: loaded_message
                                                .is_writable(*account_index as usize),
                                            is_signer: loaded_message
                                                .is_signer(*account_index as usize),
                                        }
                                    })
                                    .collect();

                                instructions_with_metadata.push((
                                    InstructionMetadata {
                                        transaction_metadata: transaction_metadata.clone(),
                                        stack_height: inner_instruction.stack_height.unwrap_or(1),
                                        index: inner_ix_idx as u32 + 1,
                                    },
                                    solana_instruction::Instruction {
                                        program_id,
                                        accounts,
                                        data: inner_instruction.instruction.data.clone(),
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(instructions_with_metadata)
}

/// Extracts account metadata from a compiled instruction and transaction
/// message.
///
/// This function converts each account index within the instruction into an
/// `AccountMeta` struct, providing details on account keys, signer status, and
/// write permissions.
///
/// # Parameters
///
/// - `compiled_instruction`: The compiled instruction to extract accounts from.
/// - `message`: The transaction message containing the account keys.
///
/// # Returns
///
/// A `CarbonResult<&[AccountMeta]>` containing
/// metadata for each account involved in the instruction.
///
/// # Errors
///
/// Returns an error if any referenced account key is missing from the
/// transaction.
pub fn extract_account_metas(
    compiled_instruction: &CompiledInstruction,
    message: &VersionedMessage,
) -> CarbonResult<Vec<AccountMeta>> {
    log::trace!(
        "extract_account_metas(compiled_instruction: {:?}, message: {:?})",
        compiled_instruction,
        message
    );
    let mut accounts = Vec::<AccountMeta>::with_capacity(compiled_instruction.accounts.len());

    for account_index in compiled_instruction.accounts.iter() {
        accounts.push(AccountMeta {
            pubkey: *message
                .static_account_keys()
                .get(*account_index as usize)
                .ok_or(Error::MissingAccountInTransaction)?,
            is_signer: message.is_signer(*account_index as usize),
            is_writable: message.is_maybe_writable(
                *account_index as usize,
                Some(
                    &message
                        .static_account_keys()
                        .iter()
                        .copied()
                        .collect::<HashSet<_>>(),
                ),
            ),
        });
    }

    Ok(accounts)
}

/// Unnests parsed instructions, producing an array of `(InstructionMetadata,
/// DecodedInstruction<T>)` tuple
///
/// This function takes a vector of `ParsedInstruction` and unnests them into a
/// vector of `(InstructionMetadata, DecodedInstruction<T>)` tuples.
/// It recursively processes nested instructions, increasing the stack height
/// for each level of nesting.
///
/// # Parameters
///
/// - `transaction_metadata`: The metadata of the transaction containing the
///   instructions.
/// - `instructions`: The vector of `ParsedInstruction` to be unnested.
/// - `stack_height`: The current stack height.
///
/// # Returns
///
/// A vector of `(InstructionMetadata, DecodedInstruction<T>)` tuples
/// representing the unnested instructions.
pub fn unnest_parsed_instructions<T: InstructionDecoderCollection>(
    transaction_metadata: TransactionMetadata,
    instructions: Vec<ParsedInstruction<T>>,
    stack_height: u32,
) -> Vec<(InstructionMetadata, DecodedInstruction<T>)> {
    log::trace!(
        "unnest_parsed_instructions(instructions: {:?})",
        instructions
    );

    let mut result = Vec::new();

    for (ix_idx, parsed_instruction) in instructions.into_iter().enumerate() {
        result.push((
            InstructionMetadata {
                transaction_metadata: transaction_metadata.clone(),
                stack_height,
                index: ix_idx as u32 + 1,
            },
            parsed_instruction.instruction,
        ));
        result.extend(unnest_parsed_instructions(
            transaction_metadata.clone(),
            parsed_instruction.inner_instructions,
            stack_height + 1,
        ));
    }

    result
}

/// Converts UI transaction metadata into `TransactionStatusMeta`.
///
/// This function transforms the user interface format of transaction metadata
/// into a more comprehensive `TransactionStatusMeta` structure suitable for
/// backend processing.
///
/// # Parameters
///
/// - `meta_original`: The original UI format of transaction status metadata.
///
/// # Returns
///
/// A `CarbonResult<TransactionStatusMeta>` representing the full transaction
/// status with nested instructions, token balances, and rewards.
///
/// # Notes
///
/// This function handles various metadata fields, including inner instructions,
/// token balances, and rewards, providing a complete view of the transaction's
/// effects.
pub fn transaction_metadata_from_original_meta(
    meta_original: UiTransactionStatusMeta,
) -> CarbonResult<TransactionStatusMeta> {
    log::trace!(
        "transaction_metadata_from_original_meta(meta_original: {:?})",
        meta_original
    );
    Ok(TransactionStatusMeta {
        status: meta_original.status,
        fee: meta_original.fee,
        pre_balances: meta_original.pre_balances,
        post_balances: meta_original.post_balances,
        inner_instructions: Some(
            meta_original
                .inner_instructions
                .unwrap_or_else(std::vec::Vec::new)
                .iter()
                .map(|inner_instruction_group| InnerInstructions {
                    index: inner_instruction_group.index,
                    instructions: inner_instruction_group
                        .instructions
                        .iter()
                        .map(|ui_instruction| match ui_instruction {
                            UiInstruction::Compiled(compiled_ui_instruction) => {
                                let decoded_data =
                                    bs58::decode(compiled_ui_instruction.data.clone())
                                        .into_vec()
                                        .unwrap_or_else(|_| vec![]);
                                InnerInstruction {
                                    instruction: CompiledInstruction {
                                        program_id_index: compiled_ui_instruction.program_id_index,
                                        accounts: compiled_ui_instruction.accounts.clone(),
                                        data: decoded_data,
                                    },
                                    stack_height: compiled_ui_instruction.stack_height,
                                }
                            }
                            _ => {
                                log::error!("Unsupported instruction type encountered");
                                InnerInstruction {
                                    instruction: CompiledInstruction {
                                        program_id_index: 0,
                                        accounts: vec![],
                                        data: vec![],
                                    },
                                    stack_height: None,
                                }
                            }
                        })
                        .collect::<Vec<InnerInstruction>>(),
                })
                .collect::<Vec<InnerInstructions>>(),
        ),
        log_messages: Some(
            meta_original
                .log_messages
                .unwrap_or_else(std::vec::Vec::new),
        ),
        pre_token_balances: Some(
            meta_original
                .pre_token_balances
                .unwrap_or_else(std::vec::Vec::new)
                .iter()
                .filter_map(|transaction_token_balance| {
                    if let (OptionSerializer::Some(owner), OptionSerializer::Some(program_id)) = (
                        transaction_token_balance.owner.as_ref(),
                        transaction_token_balance.program_id.as_ref(),
                    ) {
                        Some(TransactionTokenBalance {
                            account_index: transaction_token_balance.account_index,
                            mint: transaction_token_balance.mint.clone(),
                            ui_token_amount: transaction_token_balance.ui_token_amount.clone(),
                            owner: owner.to_string(),
                            program_id: program_id.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<TransactionTokenBalance>>(),
        ),
        post_token_balances: Some(
            meta_original
                .post_token_balances
                .unwrap_or_else(std::vec::Vec::new)
                .iter()
                .filter_map(|transaction_token_balance| {
                    if let (OptionSerializer::Some(owner), OptionSerializer::Some(program_id)) = (
                        transaction_token_balance.owner.as_ref(),
                        transaction_token_balance.program_id.as_ref(),
                    ) {
                        Some(TransactionTokenBalance {
                            account_index: transaction_token_balance.account_index,
                            mint: transaction_token_balance.mint.clone(),
                            ui_token_amount: transaction_token_balance.ui_token_amount.clone(),
                            owner: owner.to_string(),
                            program_id: program_id.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<TransactionTokenBalance>>(),
        ),
        rewards: Some(
            meta_original
                .rewards
                .unwrap_or_else(std::vec::Vec::new)
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
            let loaded = meta_original
                .loaded_addresses
                .unwrap_or_else(|| UiLoadedAddresses {
                    writable: vec![],
                    readonly: vec![],
                });
            LoadedAddresses {
                writable: loaded
                    .writable
                    .iter()
                    .map(|w| Pubkey::from_str(w).unwrap_or_default())
                    .collect::<Vec<Pubkey>>(),
                readonly: loaded
                    .readonly
                    .iter()
                    .map(|r| Pubkey::from_str(r).unwrap_or_default())
                    .collect::<Vec<Pubkey>>(),
            }
        },
        return_data: meta_original
            .return_data
            .map(|return_data| TransactionReturnData {
                program_id: return_data.program_id.parse().unwrap_or_default(),
                data: return_data.data.0.as_bytes().to_vec(),
            }),
        compute_units_consumed: meta_original
            .compute_units_consumed
            .map(|compute_unit_consumed| compute_unit_consumed)
            .or(None),
    })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use carbon_test_utils::base58_deserialize;
    use solana_account_decoder_client_types::token::UiTokenAmount;
    use solana_sdk::{hash::Hash, message::{legacy,  MessageHeader}, transaction::VersionedTransaction};
    use solana_signature::Signature;

    use crate::instruction::{InstructionsWithMetadata, NestedInstructions};

    use super::*;

    #[test]
    fn test_transaction_metadata_from_original_meta_simple() {
        // Arrange
        let expected_tx_meta = TransactionStatusMeta {
            status: Ok(()),
            fee: 9000,
            pre_balances: vec![
                323078186,
                2039280,
                3320301592555,
                68596164896,
                446975391774,
                1019603,
                2039280,
                1,
                1141440,
                1,
                731913600,
                934087680,
                3695760,
                1461600
            ],
            post_balances: vec![
                321402879,
                2039280,
                3320301602394,
                68597804804,
                446975398334,
                1029603,
                2039280,
                1,
                1141440,
                1,
                731913600,
                934087680,
                3695760,
                1461600,
            ],
            inner_instructions: Some(vec![
                InnerInstructions{ 
                    index: 1,
                    instructions: vec![
                        InnerInstruction{ 
                            instruction: CompiledInstruction{ 
                                program_id_index: 11, 
                                accounts: vec![
                                    1,
                                    13,
                                    6,
                                    3
                                ], 
                                data: base58_deserialize::ix_data("hDDqy4KAEGx3J") 
                            }, 
                            stack_height: Some(2),
                        },
                        InnerInstruction{ 
                            instruction: CompiledInstruction{ 
                                program_id_index: 7, 
                                accounts: vec![
                                    0,
                                    3
                                ], 
                                data: base58_deserialize::ix_data("3Bxs4ezjpW22kuoV") 
                            }, 
                            stack_height: Some(2),
                        },
                        InnerInstruction{ 
                            instruction: CompiledInstruction{ 
                                program_id_index: 7, 
                                accounts: vec![
                                    0,
                                    2
                                ], 
                                data: base58_deserialize::ix_data("3Bxs4KSwSHEiNiN3") 
                            }, 
                            stack_height: Some(2),
                        },
                        InnerInstruction{ 
                            instruction: CompiledInstruction{ 
                                program_id_index: 7, 
                                accounts: vec![
                                    0,
                                    4
                                ], 
                                data: base58_deserialize::ix_data("3Bxs4TdopiUbobUj") 
                            }, 
                            stack_height: Some(2),
                        },
                    ],
            }
            ]),
            log_messages: Some(vec![
                "Program ComputeBudget111111111111111111111111111111 invoke [1]",
                "Program ComputeBudget111111111111111111111111111111 success",
                "Program MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG invoke [1]",
                "Program log: Instruction: Buy",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
                "Program log: Instruction: TransferChecked",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 370747 compute units",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
                "Program 11111111111111111111111111111111 invoke [2]",
                "Program 11111111111111111111111111111111 success",
                "Program 11111111111111111111111111111111 invoke [2]",
                "Program 11111111111111111111111111111111 success",
                "Program 11111111111111111111111111111111 invoke [2]",
                "Program 11111111111111111111111111111111 success",
                "Program log: Transfering collateral from buyer to curve account: 1639908, Helio fee: 6560, Dex fee: 9839",
                "Program data: vdt/007mYe5XD5Rn8AQAAOQFGQAAAAAAbyYAAAAAAACgGQAAAAAAAAAGZYutIlwKL4hMgKVUfMrwNkmY1Lx+bGF8yTqY+mFm7CM5km+SaKcGm4hX/quBhPtof2NGGMA12sQ53BrrO1WYoPAAAAAAAc/9l+YGhwgMeWNsZs3HFBi8RjvPXd5tjX5Jv9YfHhgWAAUAAAB0cmFkZQ==",
                "Program MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG consumed 44550 of 399850 compute units",
                "Program MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG success",
                "Program 11111111111111111111111111111111 invoke [1]",
                "Program 11111111111111111111111111111111 success"
            ].into_iter().map(|s| s.to_string()).collect()),
            pre_token_balances: Some(vec![
                TransactionTokenBalance {
                    account_index:1, 
                    mint:"3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon".to_owned(),
                    ui_token_amount: UiTokenAmount { 
                        ui_amount: Some(253495663.57641867),
                        decimals: 9,
                        amount: "253495663576418647".to_owned(),
                        ui_amount_string: "253495663.576418647".to_owned(),
                    }, 
                    owner: "4CYhuDhT4c9ATZpJceoQG8Du4vCjf5ZKvxsyXpJoVub4".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:6, 
                    mint:"3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon".to_owned(),
                    ui_token_amount: UiTokenAmount { 
                        ui_amount: Some(2266244.543486133),
                        decimals: 9,
                        amount: "2266244543486133".to_owned(),
                        ui_amount_string: "2266244.543486133".to_owned(),
                    }, 
                    owner: "Ezug1uk7oTEULvBcXCngdZuJDmZ8Ed2TKY4oov4GmLLm".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
            ]),
            post_token_balances: Some(vec![
                TransactionTokenBalance {
                    account_index:1, 
                    mint:"3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon".to_owned(),
                    ui_token_amount: UiTokenAmount { 
                        ui_amount: Some(253490233.0),
                        decimals: 9,
                        amount: "253490233000000000".to_owned(),
                        ui_amount_string: "253490233".to_owned(),
                    }, 
                    owner: "4CYhuDhT4c9ATZpJceoQG8Du4vCjf5ZKvxsyXpJoVub4".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:6,
                    mint:"3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon".to_owned(),
                    ui_token_amount: UiTokenAmount { 
                        ui_amount: Some(2271675.11990478),
                        decimals: 9,
                        amount: "2271675119904780".to_owned(),
                        ui_amount_string: "2271675.11990478".to_owned(),
                    },
                    owner: "Ezug1uk7oTEULvBcXCngdZuJDmZ8Ed2TKY4oov4GmLLm".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
            ]),
            rewards: Some(vec![]),
            loaded_addresses: LoadedAddresses {
                writable: vec![],
                readonly: vec![],
            },
            return_data: None,
            compute_units_consumed: Some(44850),
        };
        // Act
        let tx_meta_status =
            carbon_test_utils::read_transaction_meta("tests/fixtures/simple_tx.json")
                .expect("read fixture");

        let original_tx_meta = transaction_metadata_from_original_meta(tx_meta_status)
            .expect("transaction metadata from original meta");
        let transaction_update = TransactionUpdate { 
            signature: Signature::default(), 
            transaction: VersionedTransaction {
                signatures: vec![Signature::default()],
                message: VersionedMessage::Legacy(
                    legacy::Message {
                        header: MessageHeader::default(),
                        account_keys: vec![
                            Pubkey::from_str_const("Ezug1uk7oTEULvBcXCngdZuJDmZ8Ed2TKY4oov4GmLLm"),
                            Pubkey::from_str_const("5Zg9kJdzYFKwS4hLzF1QvvNBYyUNpn9YWxYp6HVMknJt"),
                            Pubkey::from_str_const("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
                            Pubkey::from_str_const("4CYhuDhT4c9ATZpJceoQG8Du4vCjf5ZKvxsyXpJoVub4"),
                            Pubkey::from_str_const("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
                            Pubkey::from_str_const("ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49"),
                            Pubkey::from_str_const("6FqNPPA4W1nuvL1BHGhusSHjdNa4qJBoXyRKggAh9pb9"),
                            Pubkey::from_str_const("11111111111111111111111111111111"),
                            Pubkey::from_str_const("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG"),
                            Pubkey::from_str_const("ComputeBudget111111111111111111111111111111"),
                            Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                            Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                            Pubkey::from_str_const("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
                            Pubkey::from_str_const("3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon"),
                        ],
                        recent_blockhash: Hash::default(),
                        instructions: vec![
                            CompiledInstruction { program_id_index: 9, accounts: vec![],  data: base58_deserialize::ix_data("3GAG5eogvTjV")  },
                            CompiledInstruction { program_id_index: 8, accounts: vec![0, 6, 3, 1, 2, 4,5, 12, 11, 10, 7],  data: base58_deserialize::ix_data("XJqfG9ATWCDptdf7vx8UxGEDKxSPzetbnXg1wZsUpasa7")  },
                            CompiledInstruction { program_id_index: 7, accounts: vec![],  data: base58_deserialize::ix_data("3GAG5eogvTjV")  },
                        ],
                    },
                ),
            },
            meta: original_tx_meta.clone(), 
            is_vote: false,
            slot: 123,
            block_time: Some(123),
         };
        let transaction_metadata = transaction_update.clone().try_into().expect("transaction metadata");
        let instructions_with_metadata: InstructionsWithMetadata =
        extract_instructions_with_metadata(
            &transaction_metadata,
            &transaction_update,
        ).expect("extract instructions with metadata");

    let nested_instructions: NestedInstructions = instructions_with_metadata.into();
    
        // Assert
        assert_eq!(original_tx_meta, expected_tx_meta);
        assert_eq!(nested_instructions.len(), 3);
        assert_eq!(nested_instructions[0].inner_instructions.len(), 0);
        assert_eq!(nested_instructions[1].inner_instructions.len(), 4);
        assert_eq!(nested_instructions[2].inner_instructions.len(), 0);
    }

    #[test]
    fn test_transaction_metadata_from_original_meta_cpi() {
        // Arrange
        // Act
        // Assert
    }
}
