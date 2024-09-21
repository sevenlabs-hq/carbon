use std::collections::HashSet;

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

    for (i, compiled_instruction) in message.instructions().iter().enumerate() {
        let program_id = *compiled_instruction.program_id(&message.static_account_keys());
        let accounts = extract_account_metas(&compiled_instruction, &message)?;

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
                        let program_id = *inner_instruction
                            .instruction
                            .program_id(&message.static_account_keys());
                        let accounts =
                            extract_account_metas(&inner_instruction.instruction, &message)?;

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
