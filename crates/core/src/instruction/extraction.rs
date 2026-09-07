//! Instruction extraction and account resolution.

use {
    super::{InstructionMetadata, MAX_INSTRUCTION_STACK_DEPTH},
    crate::{
        error::{CarbonResult, Error},
        transaction::TransactionMetadata,
        update::TransactionUpdate,
    },
    solana_instruction::AccountMeta,
    solana_message::{compiled_instruction::CompiledInstruction, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_transaction_status::InnerInstructions,
    std::{collections::HashSet, sync::Arc},
};

pub fn extract_instructions_with_metadata(
    transaction_metadata: &Arc<TransactionMetadata>,
    transaction_update: &TransactionUpdate,
) -> CarbonResult<Vec<(InstructionMetadata, solana_instruction::Instruction)>> {
    let message = &transaction_update.transaction().message;
    let meta = transaction_update.meta();
    let mut instructions_with_metadata = Vec::with_capacity(32);

    match message {
        VersionedMessage::Legacy(legacy) => {
            process_instructions(
                &legacy.account_keys,
                &legacy.instructions,
                &meta.inner_instructions,
                transaction_metadata,
                &mut instructions_with_metadata,
                |_, idx| {
                    legacy.is_maybe_writable_with_reserved_addresses(idx, None::<&HashSet<Pubkey>>)
                },
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
                |key, idx| {
                    let num_static = v0.account_keys.len();
                    if idx < num_static {
                        let num_signers = v0.header.num_required_signatures as usize;
                        let num_readonly_signed = v0.header.num_readonly_signed_accounts as usize;
                        let num_readonly_unsigned =
                            v0.header.num_readonly_unsigned_accounts as usize;
                        if idx < num_signers {
                            idx < num_signers - num_readonly_signed
                        } else {
                            idx < num_static - num_readonly_unsigned
                        }
                    } else {
                        meta.loaded_addresses.writable.contains(key)
                    }
                },
                |_, idx| idx < v0.header.num_required_signatures as usize,
            );
        }
        VersionedMessage::V1(v1) => {
            process_instructions(
                &v1.account_keys,
                &v1.instructions,
                &meta.inner_instructions,
                transaction_metadata,
                &mut instructions_with_metadata,
                |_, idx| {
                    v1.is_maybe_writable_with_reserved_addresses(idx, None::<&HashSet<Pubkey>>)
                },
                |_, idx| v1.is_signer(idx),
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
                        let Some(stack_height) = validated_stack_height(inner_inst.stack_height)
                        else {
                            log::warn!(
                                "invalid inner instruction stack height ({:?}) in transaction {} at instruction {}, dropping the remaining inner instructions of this group",
                                inner_inst.stack_height,
                                transaction_metadata.signature,
                                inner_tx.index,
                            );
                            break;
                        };
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
                                absolute_path: path_stack[..stack_height].to_vec(),
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

fn validated_stack_height(stack_height: Option<u32>) -> Option<usize> {
    match stack_height {
        Some(height) if (2..=MAX_INSTRUCTION_STACK_DEPTH as u32).contains(&height) => {
            Some(height as usize)
        }
        _ => None,
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

    let mut accounts = Vec::with_capacity(instruction.accounts.len());
    for account_idx in &instruction.accounts {
        if let Some(key) = account_keys.get(*account_idx as usize) {
            accounts.push(AccountMeta {
                pubkey: *key,
                is_writable: is_writable(key, *account_idx as usize),
                is_signer: is_signer(key, *account_idx as usize),
            });
        }
    }

    solana_instruction::Instruction {
        program_id,
        accounts,
        data: instruction.data.clone(),
    }
}

pub fn extract_account_metas(
    compiled_instruction: &CompiledInstruction,
    message: &VersionedMessage,
) -> CarbonResult<Vec<AccountMeta>> {
    let mut accounts = Vec::<AccountMeta>::with_capacity(compiled_instruction.accounts.len());

    for account_index in compiled_instruction.accounts.iter() {
        accounts.push(AccountMeta {
            pubkey: *message
                .static_account_keys()
                .get(*account_index as usize)
                .ok_or(Error::MissingAccountInTransaction)?,
            is_signer: message.is_signer(*account_index as usize),
            is_writable: message.is_maybe_writable_with_reserved_addresses(
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        solana_hash::Hash,
        solana_message::{
            legacy::Message,
            v1::{self, TransactionConfig},
            MessageHeader,
        },
        solana_signature::Signature,
        solana_transaction::versioned::VersionedTransaction,
        solana_transaction_status::{InnerInstruction, TransactionStatusMeta},
    };

    #[test]
    fn test_extract_instructions_skips_inner_group_after_invalid_stack_height() {
        let build_update = |stack_heights: Vec<Option<u32>>| {
            let payer = Pubkey::new_unique();
            let program = Pubkey::new_unique();
            let instruction = CompiledInstruction {
                program_id_index: 1,
                accounts: vec![0],
                data: vec![],
            };
            TransactionUpdate::new(
                VersionedTransaction {
                    signatures: vec![Signature::default()],
                    message: VersionedMessage::Legacy(Message {
                        header: MessageHeader::default(),
                        account_keys: vec![payer, program],
                        recent_blockhash: Hash::default(),
                        instructions: vec![instruction.clone()],
                    }),
                },
                TransactionStatusMeta {
                    inner_instructions: Some(vec![InnerInstructions {
                        index: 0,
                        instructions: stack_heights
                            .into_iter()
                            .map(|stack_height| InnerInstruction {
                                instruction: instruction.clone(),
                                stack_height,
                            })
                            .collect(),
                    }]),
                    ..Default::default()
                },
                1,
            )
            .expect("transaction update")
            .with_is_vote(false)
        };
        let extract = |stack_heights: Vec<Option<u32>>| {
            extract_instructions_with_metadata(
                &Arc::new(TransactionMetadata::default()),
                &build_update(stack_heights),
            )
            .expect("extract instructions with metadata")
        };

        assert_eq!(extract(vec![None]).len(), 1);
        assert_eq!(extract(vec![Some(1)]).len(), 1);
        assert_eq!(extract(vec![Some(6)]).len(), 1);
        assert_eq!(extract(vec![Some(2)]).len(), 2);

        let partial = extract(vec![Some(2), None, Some(2)]);
        assert_eq!(partial.len(), 2);
        assert_eq!(partial[0].0.absolute_path, vec![0]);
        assert_eq!(partial[1].0.absolute_path, vec![0, 0]);
    }

    #[test]
    fn test_extract_instructions_v1_preserves_config_and_resolves_inline_accounts() {
        let payer = Pubkey::new_unique();
        let readonly_signer = Pubkey::new_unique();
        let writable_account = Pubkey::new_unique();
        let readonly_account = Pubkey::new_unique();
        let program = Pubkey::new_unique();
        let config = TransactionConfig::empty()
            .with_priority_fee(42)
            .with_compute_unit_limit(500_000)
            .with_loaded_accounts_data_size_limit(64 * 1024)
            .with_heap_size(64 * 1024);
        let top_level_instruction = CompiledInstruction {
            program_id_index: 4,
            accounts: vec![0, 1, 2, 3],
            data: vec![1, 2, 3],
        };
        let inner_instruction = CompiledInstruction {
            program_id_index: 4,
            accounts: vec![2, 3, 1],
            data: vec![4, 5, 6],
        };
        let transaction_update = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default(), Signature::default()],
                message: VersionedMessage::V1(v1::Message::new(
                    MessageHeader {
                        num_required_signatures: 2,
                        num_readonly_signed_accounts: 1,
                        num_readonly_unsigned_accounts: 2,
                    },
                    config,
                    Hash::default(),
                    vec![
                        payer,
                        readonly_signer,
                        writable_account,
                        readonly_account,
                        program,
                    ],
                    vec![top_level_instruction],
                )),
            },
            TransactionStatusMeta {
                inner_instructions: Some(vec![InnerInstructions {
                    index: 0,
                    instructions: vec![InnerInstruction {
                        instruction: inner_instruction,
                        stack_height: Some(2),
                    }],
                }]),
                ..Default::default()
            },
            1,
        )
        .expect("transaction update")
        .with_is_vote(false)
        .with_index(0);
        let transaction_metadata: TransactionMetadata = transaction_update
            .clone()
            .try_into()
            .expect("transaction metadata");

        assert_eq!(transaction_metadata.fee_payer, payer);
        let VersionedMessage::V1(metadata_message) = &transaction_metadata.message else {
            panic!("expected V1 transaction metadata");
        };
        assert_eq!(metadata_message.config, config);

        let instructions = extract_instructions_with_metadata(
            &Arc::new(transaction_metadata),
            &transaction_update,
        )
        .expect("extract V1 instructions with metadata");

        assert_eq!(instructions.len(), 2);

        let (top_level_metadata, top_level) = &instructions[0];
        assert_eq!(top_level_metadata.stack_height, 1);
        assert_eq!(top_level_metadata.absolute_path, vec![0]);
        assert_eq!(top_level.program_id, program);
        assert_eq!(top_level.data, vec![1, 2, 3]);
        assert_eq!(
            top_level.accounts,
            vec![
                AccountMeta {
                    pubkey: payer,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: readonly_signer,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: writable_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: readonly_account,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        );

        let (inner_metadata, inner) = &instructions[1];
        assert_eq!(inner_metadata.stack_height, 2);
        assert_eq!(inner_metadata.absolute_path, vec![0, 0]);
        assert_eq!(inner.program_id, program);
        assert_eq!(inner.data, vec![4, 5, 6]);
        assert_eq!(
            inner.accounts,
            vec![
                AccountMeta {
                    pubkey: writable_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: readonly_account,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: readonly_signer,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        );
    }
}
