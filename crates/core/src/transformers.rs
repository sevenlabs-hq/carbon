use std::{collections::HashSet, str::FromStr};

use solana_sdk::{
    instruction::{AccountMeta, CompiledInstruction},
    message::{
        v0::{LoadedAddresses, LoadedMessage},
        VersionedMessage,
    },
    pubkey::Pubkey,
    reserved_account_keys::ReservedAccountKeys,
};
use solana_transaction_status::{
    option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
    TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiTransactionStatusMeta,
};

use crate::{
    datasource::TransactionUpdate,
    error::{CarbonResult, Error},
    instruction::{InstructionMetadata, NestedInstruction},
    transaction::TransactionMetadata,
};

pub fn extract_transaction_metadata(
    transaction_update: &TransactionUpdate,
) -> CarbonResult<TransactionMetadata> {
    let message = transaction_update.transaction.message.clone();
    let accounts = message.static_account_keys();

    Ok(TransactionMetadata {
        slot: transaction_update.slot,
        signature: transaction_update.signature,
        fee_payer: *accounts.get(0).ok_or(Error::MissingFeePayer)?,
    })
}

pub fn extract_instructions_with_metadata(
    transaction_metadata: &TransactionMetadata,
    transaction_update: &TransactionUpdate,
) -> CarbonResult<Vec<(InstructionMetadata, solana_sdk::instruction::Instruction)>> {
    let message = transaction_update.transaction.message.clone();
    let meta = transaction_update.meta.clone();

    let mut instructions_with_metadata =
        Vec::<(InstructionMetadata, solana_sdk::instruction::Instruction)>::new();

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
                    },
                    solana_sdk::instruction::Instruction {
                        program_id,
                        accounts,
                        data: compiled_instruction.data.clone(),
                    },
                ));

                if let Some(inner_instructions) = &meta.inner_instructions {
                    for inner_instructions_per_tx in inner_instructions {
                        if inner_instructions_per_tx.index == i as u8 {
                            for inner_instruction in inner_instructions_per_tx.instructions.iter() {
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
                                        return Some(AccountMeta {
                                            pubkey: *account_pubkey,
                                            is_writable: legacy
                                                .is_maybe_writable(*account_index as usize, None),
                                            is_signer: legacy.is_signer(*account_index as usize),
                                        });
                                    })
                                    .collect();

                                instructions_with_metadata.push((
                                    InstructionMetadata {
                                        transaction_metadata: transaction_metadata.clone(),
                                        stack_height: inner_instruction.stack_height.unwrap_or(1),
                                    },
                                    solana_sdk::instruction::Instruction {
                                        program_id,
                                        accounts,
                                        data: compiled_instruction.data.clone(),
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
                writable: meta
                    .loaded_addresses
                    .writable
                    .iter()
                    .map(|key| key.clone())
                    .collect(),
                readonly: meta
                    .loaded_addresses
                    .readonly
                    .iter()
                    .map(|key| key.clone())
                    .collect(),
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
                    .filter_map(|account_index| {
                        let account_pubkey =
                            loaded_message.account_keys().get(*account_index as usize);

                        return Some(AccountMeta {
                            pubkey: account_pubkey.map(|acc| acc.clone()).unwrap_or_default(),
                            is_writable: loaded_message.is_writable(*account_index as usize),
                            is_signer: loaded_message.is_signer(*account_index as usize),
                        });
                    })
                    .collect();

                instructions_with_metadata.push((
                    InstructionMetadata {
                        transaction_metadata: transaction_metadata.clone(),
                        stack_height: 1,
                    },
                    solana_sdk::instruction::Instruction {
                        program_id,
                        accounts,
                        data: compiled_instruction.data.clone(),
                    },
                ));

                if let Some(inner_instructions) = &meta.inner_instructions {
                    for inner_instructions_per_tx in inner_instructions {
                        if inner_instructions_per_tx.index == i as u8 {
                            for inner_instruction in inner_instructions_per_tx.instructions.iter() {
                                let program_id = *loaded_message
                                    .account_keys()
                                    .get(inner_instruction.instruction.program_id_index as usize)
                                    .unwrap_or(&Pubkey::default());

                                let accounts: Vec<AccountMeta> = inner_instruction
                                    .instruction
                                    .accounts
                                    .iter()
                                    .filter_map(|account_index| {
                                        let account_pubkey = loaded_message
                                            .account_keys()
                                            .get(*account_index as usize)
                                            .map(|pubkey| pubkey.clone())
                                            .unwrap_or_default();
                                        return Some(AccountMeta {
                                            pubkey: account_pubkey.clone(),
                                            is_writable: loaded_message
                                                .is_writable(*account_index as usize),
                                            is_signer: loaded_message
                                                .is_signer(*account_index as usize),
                                        });
                                    })
                                    .collect();

                                instructions_with_metadata.push((
                                    InstructionMetadata {
                                        transaction_metadata: transaction_metadata.clone(),
                                        stack_height: inner_instruction.stack_height.unwrap_or(1),
                                    },
                                    solana_sdk::instruction::Instruction {
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

pub fn extract_account_metas(
    compiled_instruction: &solana_sdk::instruction::CompiledInstruction,
    message: &solana_sdk::message::VersionedMessage,
) -> CarbonResult<Vec<solana_sdk::instruction::AccountMeta>> {
    let mut accounts = Vec::<solana_sdk::instruction::AccountMeta>::new();

    for account_index in compiled_instruction.accounts.iter() {
        accounts.push(solana_sdk::instruction::AccountMeta {
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
                        .into_iter()
                        .map(|pubkey| pubkey.clone())
                        .collect::<HashSet<_>>(),
                ),
            ),
        });
    }

    Ok(accounts)
}

pub fn nest_instructions(
    instructions: Vec<(InstructionMetadata, solana_sdk::instruction::Instruction)>,
) -> Vec<NestedInstruction> {
    let mut result = Vec::<NestedInstruction>::new();
    let mut stack = Vec::<(Vec<usize>, usize)>::new();

    for (metadata, instruction) in instructions {
        let nested_instruction = NestedInstruction {
            metadata: metadata.clone(),
            instruction,
            inner_instructions: Vec::new(),
        };

        while let Some((_, parent_stack_height)) = stack.last() {
            if metadata.stack_height as usize > *parent_stack_height {
                break;
            }
            stack.pop();
        }

        if let Some((path_to_parent, _)) = stack.last() {
            let mut current_instructions = &mut result;
            for &index in path_to_parent {
                current_instructions = &mut current_instructions[index].inner_instructions;
            }
            current_instructions.push(nested_instruction);
            let mut new_path = path_to_parent.clone();
            new_path.push(current_instructions.len() - 1);
            stack.push((new_path, metadata.stack_height as usize));
        } else {
            result.push(nested_instruction);
            let new_path = vec![result.len() - 1];
            stack.push((new_path, metadata.stack_height as usize));
        }
    }

    result
}

pub fn build_tx_status_meta(
    tx_meta: UiTransactionStatusMeta,
) -> CarbonResult<TransactionStatusMeta> {
    let OptionSerializer::Some(meta_log_messages) = tx_meta.log_messages else {
        return CarbonResult::Err(crate::error::Error::Custom(
            "Couldn't get transaction log messages".to_string(),
        ));
    };

    let pre_token_balances = match tx_meta.pre_token_balances {
        OptionSerializer::Some(balances) => {
            let balances_result = balances
                .iter()
                .map(|pre_token_balance| {
                    let pre_token_balance_cloned = pre_token_balance.clone();
                    let owner = pre_token_balance_cloned.owner.ok_or_else(|| {
                        format!(
                            "Couldn't find the owner for account index {}",
                            pre_token_balance.account_index
                        )
                    })?;

                    let program_id = pre_token_balance_cloned.program_id.ok_or_else(|| {
                        format!(
                            "Couldn't find a program id for account index {}",
                            pre_token_balance.account_index
                        )
                    })?;

                    Ok(TransactionTokenBalance {
                        account_index: pre_token_balance_cloned.account_index,
                        mint: pre_token_balance_cloned.mint,
                        ui_token_amount: pre_token_balance_cloned.ui_token_amount,
                        owner,
                        program_id,
                    })
                })
                .collect::<Result<Vec<TransactionTokenBalance>, String>>();

            match balances_result {
                Ok(token_balances) => Some(token_balances),
                Err(_err) => None,
            }
        }
        _ => None,
    };

    let inner_instructions = match tx_meta.inner_instructions {
        OptionSerializer::Some(inner_ixs) => {
            let result: Result<Vec<InnerInstructions>, String> = inner_ixs
                .iter()
                .map(|iixs| {
                    let instructions = iixs
                        .instructions
                        .iter()
                        .map(|iix| match iix {
                            UiInstruction::Compiled(ui_compiled_instruction) => {
                                let ui_compiled_instruction_cloned =
                                    ui_compiled_instruction.clone();
                                Ok(InnerInstruction {
                                    instruction: CompiledInstruction {
                                        program_id_index: ui_compiled_instruction.program_id_index,
                                        accounts: ui_compiled_instruction_cloned.accounts,
                                        data: bs58::decode(ui_compiled_instruction_cloned.data)
                                            .into_vec()
                                            .map_err(|_| {
                                                format!(
                                                    "Failed to decode data for account index {}",
                                                    ui_compiled_instruction.program_id_index
                                                )
                                            })?,
                                    },
                                    stack_height: ui_compiled_instruction.stack_height,
                                })
                            }
                            _ => Err("Unimplemented instruction type".to_string()),
                        })
                        .collect::<Result<Vec<InnerInstruction>, String>>()?;

                    Ok(InnerInstructions {
                        index: iixs.index,
                        instructions,
                    })
                })
                .collect();

            match result {
                Ok(iixs) => Some(iixs),
                Err(_err) => None,
            }
        }
        _ => None,
    };

    let post_token_balances = match tx_meta.post_token_balances {
        OptionSerializer::Some(balances) => {
            let balances_result = balances
                .iter()
                .map(|post_token_balance| {
                    let post_token_balance_cloned = post_token_balance.clone();
                    let owner = post_token_balance_cloned.owner.ok_or_else(|| {
                        format!(
                            "Couldn't find the owner for account index {}",
                            post_token_balance.account_index
                        )
                    })?;

                    let program_id = post_token_balance_cloned.program_id.ok_or_else(|| {
                        format!(
                            "Couldn't find a program id for account index {}",
                            post_token_balance.account_index
                        )
                    })?;

                    Ok(TransactionTokenBalance {
                        account_index: post_token_balance.account_index,
                        mint: post_token_balance_cloned.mint,
                        ui_token_amount: post_token_balance_cloned.ui_token_amount,
                        owner,
                        program_id,
                    })
                })
                .collect::<Result<Vec<TransactionTokenBalance>, String>>();

            match balances_result {
                Ok(token_balances) => Some(token_balances),
                Err(_err) => None,
            }
        }
        _ => None,
    };

    let rewards = match tx_meta.rewards {
        OptionSerializer::Some(rewards_list) => {
            let rewards_result = rewards_list
                .iter()
                .map(|rewards| {
                    Ok(Reward {
                        pubkey: rewards.pubkey.clone(),
                        lamports: rewards.lamports,
                        post_balance: rewards.post_balance,
                        reward_type: rewards.reward_type,
                        commission: rewards.commission,
                    })
                })
                .collect::<Result<Vec<Reward>, String>>();

            match rewards_result {
                Ok(rewards) => Some(rewards),
                Err(_err) => None,
            }
        }
        _ => None,
    };

    let loaded_addresses = match tx_meta.loaded_addresses {
        OptionSerializer::Some(loaded) => {
            let writable_result = loaded
                .writable
                .iter()
                .map(|w| {
                    Pubkey::from_str(&w).map_err(|_| format!("Invalid writable pubkey: {}", w))
                })
                .collect::<Result<Vec<Pubkey>, String>>();

            let readonly_result = loaded
                .readonly
                .iter()
                .map(|r| {
                    Pubkey::from_str(&r).map_err(|_| format!("Invalid readonly pubkey: {}", r))
                })
                .collect::<Result<Vec<Pubkey>, String>>();

            match (writable_result, readonly_result) {
                (Ok(writable), Ok(readonly)) => LoadedAddresses { writable, readonly },
                (Err(_), _) | (_, Err(_)) => LoadedAddresses {
                    writable: vec![],
                    readonly: vec![],
                },
            }
        }
        _ => LoadedAddresses {
            writable: vec![],
            readonly: vec![],
        },
    };

    let compute_units_consumed = match tx_meta.compute_units_consumed {
        OptionSerializer::Some(cus) => Some(cus),
        _ => None,
    };

    CarbonResult::Ok(TransactionStatusMeta {
        status: tx_meta.status,
        fee: tx_meta.fee,
        pre_balances: tx_meta.pre_balances,
        post_balances: tx_meta.post_balances,
        log_messages: Some(meta_log_messages),
        pre_token_balances,
        inner_instructions,
        post_token_balances,
        rewards,
        loaded_addresses,
        return_data: None,
        compute_units_consumed,
    })
}
