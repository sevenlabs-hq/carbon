//! Instruction extraction and account resolution.

use {
    super::{
        InstructionMetadata, InstructionsWithMetadata, TransformError, MAX_INSTRUCTION_STACK_DEPTH,
    },
    crate::{transaction::TransactionMetadata, update::TransactionUpdate},
    solana_instruction::{AccountMeta, Instruction},
    solana_message::{
        compiled_instruction::CompiledInstruction,
        v0::{LoadedAddresses, LoadedMessage},
        VersionedMessage,
    },
    solana_pubkey::Pubkey,
    std::{collections::HashSet, sync::Arc},
};

pub fn extract_instructions_with_metadata(
    transaction_metadata: &Arc<TransactionMetadata>,
    transaction_update: &TransactionUpdate,
) -> Result<InstructionsWithMetadata, TransformError> {
    let message = &transaction_update.transaction().message;
    let meta = transaction_update.meta();
    let accounts = resolve_accounts(message, &meta.loaded_addresses)?;
    let instructions = message.instructions();
    if instructions.len() > usize::from(u8::MAX) + 1 {
        return Err(TransformError::InstructionPathOverflow);
    }

    let mut groups = vec![None; instructions.len()];
    for group in meta.inner_instructions.iter().flatten() {
        let entry = groups.get_mut(usize::from(group.index)).ok_or(
            TransformError::InnerGroupIndexOutOfBounds {
                index: group.index,
                instruction_count: instructions.len(),
            },
        )?;
        if entry.replace(group).is_some() {
            return Err(TransformError::DuplicateInnerGroup { index: group.index });
        }
    }

    let mut result = Vec::with_capacity(instructions.len());
    for (index, instruction) in instructions.iter().enumerate() {
        let outer_index =
            u8::try_from(index).map_err(|_| TransformError::InstructionPathOverflow)?;
        result.push((
            InstructionMetadata {
                transaction_metadata: transaction_metadata.clone(),
                stack_height: 1,
                index: u32::from(outer_index),
                absolute_path: vec![outer_index],
            },
            build_instruction(&accounts, instruction)?,
        ));

        let Some(group) = groups[index] else {
            continue;
        };
        let mut path = [0u8; MAX_INSTRUCTION_STACK_DEPTH];
        path[0] = outer_index;
        let mut previous_height = 1;
        for inner in &group.instructions {
            let height = inner
                .stack_height
                .ok_or(TransformError::MissingStackHeight)?;
            if !(2..=MAX_INSTRUCTION_STACK_DEPTH as u32).contains(&height)
                || height > previous_height + 1
            {
                return Err(TransformError::InvalidStackHeight { height });
            }
            let depth = height as usize;
            if height > previous_height {
                path[depth - 1] = 0;
            } else {
                path[depth - 1] = path[depth - 1]
                    .checked_add(1)
                    .ok_or(TransformError::InstructionPathOverflow)?;
            }

            result.push((
                InstructionMetadata {
                    transaction_metadata: transaction_metadata.clone(),
                    stack_height: height,
                    index: u32::from(outer_index),
                    absolute_path: path[..depth].to_vec(),
                },
                build_instruction(&accounts, &inner.instruction)?,
            ));
            previous_height = height;
        }
    }

    Ok(result)
}

fn resolve_accounts(
    message: &VersionedMessage,
    loaded_addresses: &LoadedAddresses,
) -> Result<Vec<AccountMeta>, TransformError> {
    let header = message.header();
    let static_count = message.static_account_keys().len();
    let signer_count = usize::from(header.num_required_signatures);
    if signer_count == 0
        || signer_count > static_count
        || usize::from(header.num_readonly_signed_accounts) >= signer_count
        || usize::from(header.num_readonly_unsigned_accounts) > static_count - signer_count
    {
        return Err(TransformError::InvalidMessageHeader);
    }

    if let VersionedMessage::V0(message) = message {
        let writable_count: usize = message
            .address_table_lookups
            .iter()
            .map(|lookup| lookup.writable_indexes.len())
            .sum();
        let readonly_count: usize = message
            .address_table_lookups
            .iter()
            .map(|lookup| lookup.readonly_indexes.len())
            .sum();
        if loaded_addresses.writable.len() != writable_count
            || loaded_addresses.readonly.len() != readonly_count
        {
            return Err(TransformError::LoadedAddressCountMismatch);
        }

        let loaded = LoadedMessage::new_borrowed(message, loaded_addresses, &HashSet::new());
        return Ok(loaded
            .account_keys()
            .iter()
            .enumerate()
            .map(|(index, key)| AccountMeta {
                pubkey: *key,
                is_signer: loaded.is_signer(index),
                is_writable: loaded.is_writable(index),
            })
            .collect());
    }

    Ok(message
        .static_account_keys()
        .iter()
        .enumerate()
        .map(|(index, key)| AccountMeta {
            pubkey: *key,
            is_signer: message.is_signer(index),
            is_writable: message
                .is_maybe_writable_with_reserved_addresses(index, None::<&HashSet<Pubkey>>),
        })
        .collect())
}

fn build_instruction(
    accounts: &[AccountMeta],
    instruction: &CompiledInstruction,
) -> Result<Instruction, TransformError> {
    let program_id = accounts
        .get(usize::from(instruction.program_id_index))
        .ok_or(TransformError::ProgramIndexOutOfBounds {
            index: instruction.program_id_index,
            account_count: accounts.len(),
        })?
        .pubkey;

    Ok(Instruction {
        program_id,
        accounts: select_accounts(accounts, instruction)?,
        data: instruction.data.clone(),
    })
}

fn select_accounts(
    accounts: &[AccountMeta],
    instruction: &CompiledInstruction,
) -> Result<Vec<AccountMeta>, TransformError> {
    instruction
        .accounts
        .iter()
        .map(|&index| {
            accounts.get(usize::from(index)).cloned().ok_or(
                TransformError::AccountIndexOutOfBounds {
                    index,
                    account_count: accounts.len(),
                },
            )
        })
        .collect()
}

/// Resolves instruction accounts using the message and runtime-loaded addresses.
pub fn extract_account_metas(
    compiled_instruction: &CompiledInstruction,
    message: &VersionedMessage,
    loaded_addresses: &LoadedAddresses,
) -> Result<Vec<AccountMeta>, TransformError> {
    select_accounts(
        &resolve_accounts(message, loaded_addresses)?,
        compiled_instruction,
    )
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
        solana_transaction_status::{InnerInstruction, InnerInstructions, TransactionStatusMeta},
    };

    #[test]
    fn invalid_indices_are_errors_for_outer_and_inner_instructions() {
        for inner in [false, true] {
            for program in [false, true] {
                let mut message = legacy_message();
                let mut invalid = message.instructions[0].clone();
                if program {
                    invalid.program_id_index = 2;
                } else {
                    invalid.accounts = vec![0, 2, 0];
                }
                let mut meta = TransactionStatusMeta::default();
                if inner {
                    meta.inner_instructions = Some(vec![InnerInstructions {
                        index: 0,
                        instructions: vec![InnerInstruction {
                            instruction: invalid,
                            stack_height: Some(2),
                        }],
                    }]);
                } else {
                    message.instructions[0] = invalid;
                }
                let expected = if program {
                    TransformError::ProgramIndexOutOfBounds {
                        index: 2,
                        account_count: 2,
                    }
                } else {
                    TransformError::AccountIndexOutOfBounds {
                        index: 2,
                        account_count: 2,
                    }
                };
                assert_eq!(
                    extract(VersionedMessage::Legacy(message), meta).unwrap_err(),
                    expected
                );
            }
        }
    }

    #[test]
    fn extraction_rejects_invalid_inner_groups() {
        for (indices, expected) in [
            (
                vec![1],
                TransformError::InnerGroupIndexOutOfBounds {
                    index: 1,
                    instruction_count: 1,
                },
            ),
            (vec![0, 0], TransformError::DuplicateInnerGroup { index: 0 }),
        ] {
            let meta = TransactionStatusMeta {
                inner_instructions: Some(
                    indices
                        .into_iter()
                        .map(|index| InnerInstructions {
                            index,
                            instructions: vec![],
                        })
                        .collect(),
                ),
                ..Default::default()
            };
            assert_eq!(
                extract(VersionedMessage::Legacy(legacy_message()), meta).unwrap_err(),
                expected
            );
        }
    }

    #[test]
    fn absent_inner_instructions_and_failed_transactions_are_valid() {
        for inner_instructions in [None, Some(vec![])] {
            let meta = TransactionStatusMeta {
                inner_instructions,
                status: serde_json::from_str(r#"{"Err":"AccountNotFound"}"#).unwrap(),
                ..Default::default()
            };
            assert_eq!(
                extract(VersionedMessage::Legacy(legacy_message()), meta)
                    .unwrap()
                    .len(),
                1
            );
        }
    }

    #[test]
    fn instruction_paths_reject_overflow() {
        for count in [256, 257] {
            let mut message = legacy_message();
            message.instructions = vec![message.instructions[0].clone(); count];
            let outer = extract(
                VersionedMessage::Legacy(message),
                TransactionStatusMeta::default(),
            );
            let message = legacy_message();
            let meta = TransactionStatusMeta {
                inner_instructions: Some(vec![InnerInstructions {
                    index: 0,
                    instructions: vec![
                        InnerInstruction {
                            instruction: message.instructions[0].clone(),
                            stack_height: Some(2),
                        };
                        count
                    ],
                }]),
                ..Default::default()
            };
            let inner = extract(VersionedMessage::Legacy(message), meta);
            if count == 256 {
                assert_eq!(outer.unwrap().last().unwrap().0.absolute_path, vec![255]);
                assert_eq!(inner.unwrap().last().unwrap().0.absolute_path, vec![0, 255]);
            } else {
                assert_eq!(outer.unwrap_err(), TransformError::InstructionPathOverflow);
                assert_eq!(inner.unwrap_err(), TransformError::InstructionPathOverflow);
            }
        }
    }

    #[test]
    fn paths_follow_runtime_order_and_reset_for_new_parents() {
        let mut message = legacy_message();
        message.instructions.push(message.instructions[0].clone());
        let meta = TransactionStatusMeta {
            inner_instructions: Some(
                vec![1, 0]
                    .into_iter()
                    .map(|index| InnerInstructions {
                        index,
                        instructions: [2, 3, 3, 2, 3, 4, 2]
                            .into_iter()
                            .map(|height| InnerInstruction {
                                instruction: message.instructions[0].clone(),
                                stack_height: Some(height),
                            })
                            .collect(),
                    })
                    .collect(),
            ),
            ..Default::default()
        };
        let instructions = extract(VersionedMessage::Legacy(message), meta).unwrap();
        let paths: Vec<_> = instructions
            .iter()
            .map(|(metadata, _)| metadata.absolute_path.clone())
            .collect();
        assert_eq!(
            paths,
            vec![
                vec![0],
                vec![0, 0],
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1],
                vec![0, 1, 0],
                vec![0, 1, 0, 0],
                vec![0, 2],
                vec![1],
                vec![1, 0],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 1],
                vec![1, 1, 0],
                vec![1, 1, 0, 0],
                vec![1, 2],
            ]
        );
        let tree = crate::instruction::NestedInstructions::try_from(instructions).unwrap();
        assert_eq!(tree.len(), 2);
        assert_eq!(tree[0].inner_instructions.len(), 3);
        assert_eq!(tree[0].inner_instructions[0].inner_instructions.len(), 2);
        assert_eq!(
            tree[1].inner_instructions[1].inner_instructions[0]
                .inner_instructions
                .len(),
            1
        );
    }

    #[test]
    fn account_helper_preserves_writable_signers_and_account_order() {
        let mut message = legacy_message();
        message.instructions[0].accounts = vec![0, 1, 0];
        let expected = vec![
            AccountMeta::new(message.account_keys[0], true),
            AccountMeta::new_readonly(message.account_keys[1], false),
            AccountMeta::new(message.account_keys[0], true),
        ];
        let message = VersionedMessage::Legacy(message);
        assert_eq!(
            extract_account_metas(
                &message.instructions()[0],
                &message,
                &LoadedAddresses::default()
            )
            .unwrap(),
            expected
        );
        assert_eq!(
            extract(message, TransactionStatusMeta::default()).unwrap()[0]
                .1
                .accounts,
            expected
        );
    }

    #[test]
    fn v0_resolves_loaded_accounts_and_programs() {
        let legacy = legacy_message();
        let loaded_addresses = LoadedAddresses {
            writable: vec![Pubkey::new_unique(), Pubkey::new_unique()],
            readonly: vec![Pubkey::new_unique()],
        };
        let message = VersionedMessage::V0(solana_message::v0::Message {
            header: legacy.header,
            account_keys: legacy.account_keys.clone(),
            recent_blockhash: Hash::default(),
            instructions: vec![CompiledInstruction {
                program_id_index: 3,
                accounts: vec![4, 0, 2, 3, 1, 2],
                data: vec![7],
            }],
            address_table_lookups: vec![solana_message::v0::MessageAddressTableLookup {
                account_key: Pubkey::new_unique(),
                writable_indexes: vec![8, 9],
                readonly_indexes: vec![3],
            }],
        });
        let expected = vec![
            AccountMeta::new_readonly(loaded_addresses.readonly[0], false),
            AccountMeta::new(legacy.account_keys[0], true),
            AccountMeta::new(loaded_addresses.writable[0], false),
            AccountMeta::new_readonly(loaded_addresses.writable[1], false),
            AccountMeta::new_readonly(legacy.account_keys[1], false),
            AccountMeta::new(loaded_addresses.writable[0], false),
        ];
        assert_eq!(
            extract_account_metas(&message.instructions()[0], &message, &loaded_addresses).unwrap(),
            expected
        );
        let meta = TransactionStatusMeta {
            loaded_addresses: loaded_addresses.clone(),
            inner_instructions: Some(vec![InnerInstructions {
                index: 0,
                instructions: vec![InnerInstruction {
                    instruction: message.instructions()[0].clone(),
                    stack_height: Some(2),
                }],
            }]),
            ..Default::default()
        };
        for (_, instruction) in extract(message.clone(), meta).unwrap() {
            assert_eq!(instruction.program_id, loaded_addresses.writable[1]);
            assert_eq!(instruction.accounts, expected);
        }
        assert_eq!(
            extract(message, TransactionStatusMeta::default()).unwrap_err(),
            TransformError::LoadedAddressCountMismatch
        );
    }

    #[test]
    fn invalid_headers_are_errors_for_each_message_version() {
        for header in [
            MessageHeader::default(),
            MessageHeader {
                num_required_signatures: 3,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 0,
            },
            MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 2,
                num_readonly_unsigned_accounts: 0,
            },
            MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 1,
                num_readonly_unsigned_accounts: 0,
            },
            MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 2,
            },
        ] {
            let mut legacy = legacy_message();
            legacy.header = header;
            let v0 = solana_message::v0::Message {
                header,
                account_keys: legacy.account_keys.clone(),
                recent_blockhash: legacy.recent_blockhash,
                instructions: legacy.instructions.clone(),
                address_table_lookups: vec![],
            };
            let v1 = v1::Message::new(
                header,
                TransactionConfig::empty(),
                Hash::default(),
                legacy.account_keys.clone(),
                legacy.instructions.clone(),
            );
            for message in [
                VersionedMessage::Legacy(legacy),
                VersionedMessage::V0(v0),
                VersionedMessage::V1(v1),
            ] {
                assert_eq!(
                    extract(message, TransactionStatusMeta::default()).unwrap_err(),
                    TransformError::InvalidMessageHeader
                );
            }
        }
    }

    fn legacy_message() -> Message {
        Message {
            header: MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 1,
            },
            account_keys: vec![Pubkey::new_unique(), Pubkey::new_unique()],
            recent_blockhash: Hash::default(),
            instructions: vec![CompiledInstruction {
                program_id_index: 1,
                accounts: vec![0],
                data: vec![],
            }],
        }
    }

    fn extract(
        message: VersionedMessage,
        meta: TransactionStatusMeta,
    ) -> Result<InstructionsWithMetadata, TransformError> {
        let update = TransactionUpdate::new(
            VersionedTransaction {
                signatures: vec![Signature::default()],
                message,
            },
            meta,
            1,
        )
        .unwrap();
        let metadata = Arc::new(update.clone().try_into().unwrap());
        extract_instructions_with_metadata(&metadata, &update)
    }

    #[test]
    fn extraction_rejects_invalid_stack_heights() {
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
                        header: MessageHeader {
                            num_required_signatures: 1,
                            num_readonly_signed_accounts: 0,
                            num_readonly_unsigned_accounts: 1,
                        },
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
        };

        assert_eq!(
            extract(vec![None]).unwrap_err(),
            TransformError::MissingStackHeight
        );
        for height in [0, 1, 3, MAX_INSTRUCTION_STACK_DEPTH as u32 + 1, u32::MAX] {
            assert_eq!(
                extract(vec![Some(height)]).unwrap_err(),
                TransformError::InvalidStackHeight { height }
            );
        }
        assert_eq!(
            extract(vec![Some(2), Some(4)]).unwrap_err(),
            TransformError::InvalidStackHeight { height: 4 }
        );
        assert_eq!(extract(vec![Some(2)]).unwrap().len(), 2);
        assert_eq!(
            extract(vec![Some(2), None, Some(2)]).unwrap_err(),
            TransformError::MissingStackHeight
        );
        assert_eq!(
            extract((2..=MAX_INSTRUCTION_STACK_DEPTH as u32).map(Some).collect())
                .unwrap()
                .len(),
            MAX_INSTRUCTION_STACK_DEPTH
        );
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
