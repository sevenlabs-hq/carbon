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
        instruction::{DecodedInstruction, InstructionMetadata, MAX_INSTRUCTION_STACK_DEPTH},
        schema::ParsedInstruction,
        transaction::TransactionMetadata,
    },
    solana_instruction::AccountMeta,
    solana_message::{
        compiled_instruction::CompiledInstruction, v0::LoadedAddresses, VersionedMessage,
    },
    solana_pubkey::Pubkey,
    solana_transaction_context::TransactionReturnData,
    solana_transaction_status::{
        option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
        TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiLoadedAddresses,
        UiTransactionStatusMeta,
    },
    std::{collections::HashSet, str::FromStr, sync::Arc},
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
    transaction_metadata: &Arc<TransactionMetadata>,
    transaction_update: &TransactionUpdate,
) -> CarbonResult<Vec<(InstructionMetadata, solana_instruction::Instruction)>> {
    log::trace!(
        "extract_instructions_with_metadata(transaction_metadata: {transaction_metadata:?}, transaction_update: {transaction_update:?})"
    );

    let message = &transaction_update.transaction.message;
    let meta = &transaction_update.meta;
    let mut instructions_with_metadata = Vec::with_capacity(32);

    match message {
        VersionedMessage::Legacy(legacy) => {
            process_instructions(
                &legacy.account_keys,
                &legacy.instructions,
                &meta.inner_instructions,
                transaction_metadata,
                &mut instructions_with_metadata,
                |_, idx| legacy.is_maybe_writable(idx, None),
                |_, idx| legacy.is_signer(idx),
            );
        }
        VersionedMessage::V0(v0) => {
            let mut account_keys: Vec<Pubkey> = Vec::with_capacity(
                v0.account_keys.len()
                    + meta.loaded_addresses.writable.len()
                    + meta.loaded_addresses.readonly.len(),
            );

            account_keys.extend_from_slice(&v0.account_keys);
            account_keys.extend_from_slice(&meta.loaded_addresses.writable);
            account_keys.extend_from_slice(&meta.loaded_addresses.readonly);

            process_instructions(
                &account_keys,
                &v0.instructions,
                &meta.inner_instructions,
                transaction_metadata,
                &mut instructions_with_metadata,
                |key, _| meta.loaded_addresses.writable.contains(key),
                |_, idx| idx < v0.header.num_required_signatures as usize,
            );
        }
    }

    Ok(instructions_with_metadata)
}

fn process_instructions<F1, F2>(
    account_keys: &[Pubkey],
    instructions: &[CompiledInstruction],
    inner: &Option<Vec<InnerInstructions>>,
    transaction_metadata: &Arc<TransactionMetadata>,
    result: &mut Vec<(InstructionMetadata, solana_instruction::Instruction)>,
    is_writable: F1,
    is_signer: F2,
) where
    F1: Fn(&Pubkey, usize) -> bool,
    F2: Fn(&Pubkey, usize) -> bool,
{
    for (i, compiled_instruction) in instructions.iter().enumerate() {
        result.push((
            InstructionMetadata {
                transaction_metadata: transaction_metadata.clone(),
                stack_height: 1,
                index: i as u32,
                absolute_path: vec![i as u8],
            },
            build_instruction(account_keys, compiled_instruction, &is_writable, &is_signer),
        ));

        if let Some(inner_instructions) = inner {
            for inner_tx in inner_instructions {
                if inner_tx.index as usize == i {
                    let mut path_stack = [0; MAX_INSTRUCTION_STACK_DEPTH];
                    path_stack[0] = inner_tx.index;
                    let mut prev_height = 0;

                    for inner_inst in &inner_tx.instructions {
                        let stack_height = inner_inst.stack_height.unwrap_or(1) as usize;
                        if stack_height > prev_height {
                            path_stack[stack_height - 1] = 0;
                        } else {
                            path_stack[stack_height - 1] += 1;
                        }

                        result.push((
                            InstructionMetadata {
                                transaction_metadata: transaction_metadata.clone(),
                                stack_height: stack_height as u32,
                                index: inner_tx.index as u32,
                                absolute_path: path_stack[..stack_height].into(),
                            },
                            build_instruction(
                                account_keys,
                                &inner_inst.instruction,
                                &is_writable,
                                &is_signer,
                            ),
                        ));

                        prev_height = stack_height;
                    }
                }
            }
        }
    }
}

fn build_instruction<F1, F2>(
    account_keys: &[Pubkey],
    instruction: &CompiledInstruction,
    is_writable: &F1,
    is_signer: &F2,
) -> solana_instruction::Instruction
where
    F1: Fn(&Pubkey, usize) -> bool,
    F2: Fn(&Pubkey, usize) -> bool,
{
    let program_id = *account_keys
        .get(instruction.program_id_index as usize)
        .unwrap_or(&Pubkey::default());

    let accounts = instruction
        .accounts
        .iter()
        .filter_map(|account_idx| {
            account_keys
                .get(*account_idx as usize)
                .map(|key| AccountMeta {
                    pubkey: *key,
                    is_writable: is_writable(key, *account_idx as usize),
                    is_signer: is_signer(key, *account_idx as usize),
                })
        })
        .collect();

    solana_instruction::Instruction {
        program_id,
        accounts,
        data: instruction.data.clone(),
    }
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
        "extract_account_metas(compiled_instruction: {compiled_instruction:?}, message: {message:?})"
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
    transaction_metadata: Arc<TransactionMetadata>,
    instructions: Vec<ParsedInstruction<T>>,
    stack_height: u32,
) -> Vec<(InstructionMetadata, DecodedInstruction<T>)> {
    log::trace!("unnest_parsed_instructions(instructions: {instructions:?})");

    let mut result = Vec::new();

    for (ix_idx, parsed_instruction) in instructions.into_iter().enumerate() {
        result.push((
            InstructionMetadata {
                transaction_metadata: transaction_metadata.clone(),
                stack_height,
                index: ix_idx as u32 + 1,
                absolute_path: vec![],
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
    log::trace!("transaction_metadata_from_original_meta(meta_original: {meta_original:?})");
    Ok(TransactionStatusMeta {
        status: meta_original.status.map_err(Into::into),
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
        cost_units: meta_original.cost_units.into(),
    })
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::instruction::{InstructionsWithMetadata, NestedInstructions},
        carbon_test_utils::base58_deserialize,
        solana_account_decoder_client_types::token::UiTokenAmount,
        solana_hash::Hash,
        solana_message::{
            legacy::Message,
            v0::{self, MessageAddressTableLookup},
            MessageHeader,
        },
        solana_signature::Signature,
        solana_transaction::versioned::VersionedTransaction,
        std::vec,
    };

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
            cost_units: None,
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
                message: VersionedMessage::Legacy(Message {
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
                        CompiledInstruction {
                            program_id_index: 9,
                            accounts: vec![],
                            data: base58_deserialize::ix_data("3GAG5eogvTjV"),
                        },
                        CompiledInstruction {
                            program_id_index: 8,
                            accounts: vec![0, 6, 3, 1, 2, 4, 5, 12, 11, 10, 7],
                            data: base58_deserialize::ix_data(
                                "XJqfG9ATWCDptdf7vx8UxGEDKxSPzetbnXg1wZsUpasa7",
                            ),
                        },
                        CompiledInstruction {
                            program_id_index: 7,
                            accounts: vec![],
                            data: base58_deserialize::ix_data("3GAG5eogvTjV"),
                        },
                    ],
                }),
            },
            meta: original_tx_meta.clone(),
            is_vote: false,
            slot: 123,
            block_time: Some(123),
            block_hash: Hash::from_str("9bit9vXNX9HyHwL89aGDNmk3vbyAM96nvb6F4SaoM1CU").ok(),
        };
        let transaction_metadata = transaction_update
            .clone()
            .try_into()
            .expect("transaction metadata");
        let instructions_with_metadata: InstructionsWithMetadata =
            extract_instructions_with_metadata(
                &Arc::new(transaction_metadata),
                &transaction_update,
            )
            .expect("extract instructions with metadata");

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
        let expected_tx_meta = TransactionStatusMeta {
            status: Ok(()),
            fee: 80000,
            pre_balances: vec![
                64472129,
                2039280,
                2039280,
                71437440,
                2039280,
                1,
                1141440,
                7775404600,
                117416465239,
                731913600,
                71437440,
                23385600,
                71437440,
                7182750,
                2039280,
                2039280,
                1141440,
                1,
                934087680,
                4000000
            ],
            post_balances: vec![
                64392129,
                2039280,
                2039280,
                71437440,
                2039280,
                1,
                1141440,
                7775404600,
                117416465239,
                731913600,
                71437440,
                23385600,
                71437440,
                7182750,
                2039280,
                2039280,
                1141440,
                1,
                934087680,
                4000000
            ],
            cost_units: None,
            inner_instructions: Some(vec![
                InnerInstructions {
                    index: 3,
                    instructions: vec![
                        InnerInstruction{
                            instruction: CompiledInstruction{
                                program_id_index: 16,
                                accounts: vec![
                                    13,
                                    16,
                                    14,
                                    15,
                                    1,
                                    2,
                                    7,
                                    8,
                                    11,
                                    16,
                                    0,
                                    18,
                                    18,
                                    19,
                                    16,
                                    3,
                                    10,
                                    12,
                                ],
                                data: base58_deserialize::ix_data("PgQWtn8oziwqoZL8sWNwT7LtzLzAUp8MM") 
                            },
                            stack_height: Some(2),
                        },
                        InnerInstruction{
                            instruction: CompiledInstruction{
                                program_id_index: 18,
                                accounts: vec![
                                    1,
                                    8,
                                    15,
                                    0,
                                ],
                                data: base58_deserialize::ix_data("gD28Qcm8qkpHv") 
                            },
                            stack_height: Some(3),
                        },
                        InnerInstruction{
                            instruction: CompiledInstruction{
                                program_id_index: 18,
                                accounts: vec![
                                    14,
                                    7,
                                    2,
                                    13,
                                ],
                                data: base58_deserialize::ix_data("hLZYKissEeFUU") 
                            },
                            stack_height: Some(3),
                        },
                        InnerInstruction{
                            instruction: CompiledInstruction{
                                program_id_index: 16,
                                accounts: vec![
                                    19,
                                ],
                                data: base58_deserialize::ix_data("yCGxBopjnVNQkNP5usq1PonMQAFjN4WpP7MXQHZjf7XRFvuZeLCkVHy966UtS1VyTsN9u6oGPC5aaYqLj5UXxLj8FaCJccaibatRPgkX95PDrzwLBhZE43gcsTwwccBuEd67YuWJsM1j7tXo5ntSaTWRsfuaqkkoaCDDeidPunPSTBRUY68Hw5oFnYhUcG5CUEPWmM") 
                            },
                            stack_height: Some(3),
                        },
                        InnerInstruction{
                            instruction: CompiledInstruction{
                                program_id_index: 18,
                                accounts: vec![
                                    1,
                                    8,
                                    4,
                                    0,
                                ],
                                data: base58_deserialize::ix_data("heASn5ozzjZrp") 
                            },
                            stack_height: Some(2),
                        },
                    ],
                }
            ]),
            log_messages: Some(vec![
                "Program ComputeBudget111111111111111111111111111111 invoke [1]",
                "Program ComputeBudget111111111111111111111111111111 success",
                "Program ComputeBudget111111111111111111111111111111 invoke [1]",
                "Program ComputeBudget111111111111111111111111111111 success",
                "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
                "Program log: CreateIdempotent",
                "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 4338 of 191700 compute units",
                "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
                "Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma invoke [1]",
                "Program log: Instruction: CommissionSplSwap2",
                "Program log: order_id: 105213",
                "Program log: AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB",
                "Program log: 7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx",
                "Program log: 7Z2QzVa3q7r7m84nuez9eRn2u3oCUeg9D1bzdRvNFdxN",
                "Program log: 7Z2QzVa3q7r7m84nuez9eRn2u3oCUeg9D1bzdRvNFdxN",
                "Program log: before_source_balance: 9452950000000, before_destination_balance: 573930799366, amount_in: 9372599925000, expect_amount_out: 1700985700449, min_return: 1680573872044",
                "Program log: Dex::MeteoraDlmm amount_in: 9372599925000, offset: 0",
                "Program log: BMCheVSdKZ6rxsoJ1MChA5HQRtk5pz4QkCLs7MXFkvZJ",
                "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo invoke [2]",
                "Program log: Instruction: Swap",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
                "Program log: Instruction: TransferChecked",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6173 of 112532 compute units",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
                "Program log: Instruction: TransferChecked",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6147 of 102925 compute units",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
                "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo invoke [3]",
                "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo consumed 2134 of 93344 compute units",
                "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo success",
                "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo consumed 63904 of 153546 compute units",
                "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo success",
                "Program data: QMbN6CYIceINCDl9OoYIAABhAKYKjAEAAA==",
                "Program log: SwapEvent { dex: MeteoraDlmm, amount_in: 9372599925000, amount_out: 1700985700449 }",
                "Program log: 6VRWsRGxnJFg7y4ck3NBsBPQ5SLkCtkH3tTioJkEby3b",
                "Program log: AADrz4o64xxynMr8tochpqAYevmySvsEhjAgKEXNcjnd",
                "Program log: after_source_balance: 80350075000, after_destination_balance: 2274916499815, source_token_change: 9372599925000, destination_token_change: 1700985700449",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
                "Program log: Instruction: TransferChecked",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6173 of 76109 compute units",
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
                "Program log: commission_direction: true, commission_amount: 80350075000",
                "Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma consumed 118873 of 187362 compute units",
                "Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma success"
            ].into_iter().map(|s| s.to_string()).collect()),
            pre_token_balances: Some(vec![
                TransactionTokenBalance {
                    account_index:1,
                    mint:"AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(9452.95),
                        decimals: 9,
                        amount: "9452950000000".to_owned(),
                        ui_amount_string: "9452.95".to_owned(),
                    },
                    owner: "7Z2QzVa3q7r7m84nuez9eRn2u3oCUeg9D1bzdRvNFdxN".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:2,
                    mint:"7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(573.930799366),
                        decimals: 9,
                        amount: "573930799366".to_owned(),
                        ui_amount_string: "573.930799366".to_owned(),
                    },
                    owner: "7Z2QzVa3q7r7m84nuez9eRn2u3oCUeg9D1bzdRvNFdxN".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:4,
                    mint:"AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(357.387320573),
                        decimals: 9,
                        amount: "357387320573".to_owned(),
                        ui_amount_string: "357.387320573".to_owned(),
                    },
                    owner: "8psNvWTrdNTiVRNzAgsou9kETXNJm2SXZyaKuJraVRtf".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:14,
                    mint:"7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(108469.853556668),
                        decimals: 9,
                        amount: "108469853556668".to_owned(),
                        ui_amount_string: "108469.853556668".to_owned(),
                    },
                    owner: "BMCheVSdKZ6rxsoJ1MChA5HQRtk5pz4QkCLs7MXFkvZJ".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:15,
                    mint:"AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(176698.438078034),
                        decimals: 9,
                        amount: "176698438078034".to_owned(),
                        ui_amount_string: "176698.438078034".to_owned(),
                    },
                    owner: "BMCheVSdKZ6rxsoJ1MChA5HQRtk5pz4QkCLs7MXFkvZJ".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
            ]),
            post_token_balances: Some(vec![
                TransactionTokenBalance {
                    account_index:1,
                    mint:"AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: None,
                        decimals: 9,
                        amount: "0".to_owned(),
                        ui_amount_string: "0".to_owned(),
                    },
                    owner: "7Z2QzVa3q7r7m84nuez9eRn2u3oCUeg9D1bzdRvNFdxN".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:2,
                    mint:"7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some( 2274.916499815),
                        decimals: 9,
                        amount: "2274916499815".to_owned(),
                        ui_amount_string: "2274.916499815".to_owned(),
                    },
                    owner: "7Z2QzVa3q7r7m84nuez9eRn2u3oCUeg9D1bzdRvNFdxN".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:4,
                    mint:"AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(437.737395573),
                        decimals: 9,
                        amount: "437737395573".to_owned(),
                        ui_amount_string: "437.737395573".to_owned(),
                    },
                    owner: "8psNvWTrdNTiVRNzAgsou9kETXNJm2SXZyaKuJraVRtf".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:14,
                    mint:"7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(106768.867856219),
                        decimals: 9,
                        amount: "106768867856219".to_owned(),
                        ui_amount_string: "106768.867856219".to_owned(),
                    },
                    owner: "BMCheVSdKZ6rxsoJ1MChA5HQRtk5pz4QkCLs7MXFkvZJ".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
                TransactionTokenBalance {
                    account_index:15,
                    mint:"AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB".to_owned(),
                    ui_token_amount: UiTokenAmount {
                        ui_amount: Some(186071.038003034),
                        decimals: 9,
                        amount: "186071038003034".to_owned(),
                        ui_amount_string: "186071.038003034".to_owned(),
                    },
                    owner: "BMCheVSdKZ6rxsoJ1MChA5HQRtk5pz4QkCLs7MXFkvZJ".to_owned(),
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_owned(),
                },
            ]),
            rewards: Some(vec![]),
            loaded_addresses: LoadedAddresses {
                writable: vec![
                    Pubkey::from_str_const("12QvTU4Z7XdxT16mSYU8rE2n9CpXpvunHiZrfySaf7h8"),
                    Pubkey::from_str_const("76TaSYC4LuopNGf5apJUXMG2MDfcQMHiw6SMX93VYQGp"),
                    Pubkey::from_str_const("7q4JPakWqK7UrjRsTNoYXMNeBYKP3WKawNcHYzidTaav"),
                    Pubkey::from_str_const("BMCheVSdKZ6rxsoJ1MChA5HQRtk5pz4QkCLs7MXFkvZJ"),
                    Pubkey::from_str_const("GqXFYwijuNaKRQgtrVrJkDGXorPcZwX7Vyd4jDsuxW9J"),
                    Pubkey::from_str_const("JBbKXBC4yBco9BfChDaf5GHd8hyLbjASeLxFCwGCH99a"),
                    Pubkey::from_str_const("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo")
                ],
                readonly: vec![
                    Pubkey::from_str_const("11111111111111111111111111111111"),
                    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                    Pubkey::from_str_const("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                ],
            },
            return_data: None,
            compute_units_consumed: Some(123511),
        };

        // Act
        let tx_meta_status = carbon_test_utils::read_transaction_meta("tests/fixtures/cpi_tx.json")
            .expect("read fixture");
        let original_tx_meta = transaction_metadata_from_original_meta(tx_meta_status)
            .expect("transaction metadata from original meta");
        let transaction_update = TransactionUpdate {
            signature: Signature::default(),
            transaction: VersionedTransaction {
                signatures: vec![Signature::default()],
                message: VersionedMessage::V0(v0::Message {
                    header: MessageHeader::default(),
                    account_keys: vec![
                        Pubkey::from_str_const("7Z2QzVa3q7r7m84nuez9eRn2u3oCUeg9D1bzdRvNFdxN"),
                        Pubkey::from_str_const("6VRWsRGxnJFg7y4ck3NBsBPQ5SLkCtkH3tTioJkEby3b"),
                        Pubkey::from_str_const("AADrz4o64xxynMr8tochpqAYevmySvsEhjAgKEXNcjnd"),
                        Pubkey::from_str_const("EuGVLjHv1K1YVxmcukLYMBjGB7YXy5hxbJ2z4LeDLUfQ"),
                        Pubkey::from_str_const("FDxb6WnUHrSsz9zwKqneC5JXVmrRgPySBjf6WfNfFyrM"),
                        Pubkey::from_str_const("ComputeBudget111111111111111111111111111111"),
                        Pubkey::from_str_const("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma"),
                        Pubkey::from_str_const("7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx"),
                        Pubkey::from_str_const("AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB"),
                        Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                        Pubkey::from_str_const("12QvTU4Z7XdxT16mSYU8rE2n9CpXpvunHiZrfySaf7h8"),
                        Pubkey::from_str_const("76TaSYC4LuopNGf5apJUXMG2MDfcQMHiw6SMX93VYQGp"),
                        Pubkey::from_str_const("7q4JPakWqK7UrjRsTNoYXMNeBYKP3WKawNcHYzidTaav"),
                        Pubkey::from_str_const("BMCheVSdKZ6rxsoJ1MChA5HQRtk5pz4QkCLs7MXFkvZJ"),
                        Pubkey::from_str_const("GqXFYwijuNaKRQgtrVrJkDGXorPcZwX7Vyd4jDsuxW9J"),
                        Pubkey::from_str_const("JBbKXBC4yBco9BfChDaf5GHd8hyLbjASeLxFCwGCH99a"),
                        Pubkey::from_str_const("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                        Pubkey::from_str_const("11111111111111111111111111111111"),
                        Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                        Pubkey::from_str_const("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6"),
                    ],
                    recent_blockhash: Hash::default(),
                    instructions: vec![
                        CompiledInstruction {
                            program_id_index: 5,
                            accounts: vec![],
                            data: base58_deserialize::ix_data("3GAG5eogvTjV"),
                        },
                        CompiledInstruction {
                            program_id_index: 5,
                            accounts: vec![],
                            data: base58_deserialize::ix_data("3GAG5eogvTjV"),
                        },
                        CompiledInstruction {
                            program_id_index: 9,
                            accounts: vec![],
                            data: base58_deserialize::ix_data("3GAG5eogvTjV"),
                        },
                        CompiledInstruction {
                            program_id_index: 8,
                            accounts: vec![],
                            data: base58_deserialize::ix_data(
                                "XJqfG9ATWCDptdf7vx8UxGEDKxSPzetbnXg1wZsUpasa7",
                            ),
                        },
                    ],
                    address_table_lookups: vec![
                        MessageAddressTableLookup {
                            account_key: Pubkey::from_str_const(
                                "FfMiwAdZeeSZyuApu5fsCuPzvyAyKdEbNcmEVEEhgJAW",
                            ),
                            writable_indexes: vec![0, 1, 2, 3, 4, 5],
                            readonly_indexes: vec![],
                        },
                        MessageAddressTableLookup {
                            account_key: Pubkey::from_str_const(
                                "EDDSpjZHrsFKYTMJDcBqXAjkLcu9EKdvrQR4XnqsXErH",
                            ),
                            writable_indexes: vec![0],
                            readonly_indexes: vec![3, 4, 5],
                        },
                    ],
                }),
            },
            meta: original_tx_meta.clone(),
            is_vote: false,
            slot: 123,
            block_time: Some(123),
            block_hash: None,
        };
        let transaction_metadata = transaction_update
            .clone()
            .try_into()
            .expect("transaction metadata");
        let instructions_with_metadata: InstructionsWithMetadata =
            extract_instructions_with_metadata(
                &Arc::new(transaction_metadata),
                &transaction_update,
            )
            .expect("extract instructions with metadata");
        let nested_instructions: NestedInstructions = instructions_with_metadata.into();

        // Assert
        assert_eq!(original_tx_meta, expected_tx_meta);
        assert_eq!(nested_instructions.len(), 4);
        assert_eq!(nested_instructions[0].inner_instructions.len(), 0);
        assert_eq!(nested_instructions[1].inner_instructions.len(), 0);
        assert_eq!(nested_instructions[2].inner_instructions.len(), 0);
        assert_eq!(nested_instructions[3].inner_instructions.len(), 2);
    }
}
