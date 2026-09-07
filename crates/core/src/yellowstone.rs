//! Yellowstone protobuf-to-Solana transaction conversion.

use {
    solana_account_decoder_client_types::token::UiTokenAmount,
    solana_hash::{Hash, HASH_BYTES},
    solana_message::{
        compiled_instruction::CompiledInstruction,
        v0::{LoadedAddresses, Message as MessageV0, MessageAddressTableLookup},
        v1::{Message as MessageV1, TransactionConfig},
        Message as LegacyMessage, MessageHeader, VersionedMessage,
    },
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
    solana_signature::{Signature, SIGNATURE_BYTES},
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_context::transaction::TransactionReturnData,
    solana_transaction_error::TransactionError,
    solana_transaction_status::{
        InnerInstruction, InnerInstructions, Reward, RewardType, TransactionStatusMeta,
        TransactionTokenBalance,
    },
    thiserror::Error,
    yellowstone_grpc_proto::prelude as proto,
};

pub type ConversionResult<T> = Result<T, ConversionError>;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("missing required protobuf field `{0}`")]
    MissingField(&'static str),

    #[error("invalid byte length for `{field}`: expected {expected}, got {actual}")]
    InvalidByteLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("value {value} for `{field}` does not fit in a u8")]
    IntegerOutOfRange { field: &'static str, value: u32 },

    #[error("invalid enum value {value} for `{field}`")]
    InvalidEnum { field: &'static str, value: i32 },

    #[error("invalid numeric value `{value}` for `{field}`")]
    InvalidNumber { field: &'static str, value: String },

    #[error("failed to decode the protobuf transaction error with wincode")]
    InvalidTransactionError(#[source] wincode::ReadError),
}

/// Convert a protobuf transaction, including Legacy, V0, and V1 messages.
pub fn create_tx_versioned(tx: proto::Transaction) -> ConversionResult<VersionedTransaction> {
    let signatures = tx
        .signatures
        .into_iter()
        .map(|signature| {
            let actual = signature.len();
            Signature::try_from(signature).map_err(|_| ConversionError::InvalidByteLength {
                field: "transaction.signatures",
                expected: SIGNATURE_BYTES,
                actual,
            })
        })
        .collect::<ConversionResult<Vec<_>>>()?;

    Ok(VersionedTransaction {
        signatures,
        message: create_message(
            tx.message
                .ok_or(ConversionError::MissingField("transaction.message"))?,
        )?,
    })
}

fn create_message(message: proto::Message) -> ConversionResult<VersionedMessage> {
    let proto::Message {
        header,
        account_keys,
        recent_blockhash,
        instructions,
        versioned,
        address_table_lookups,
        config,
    } = message;

    let header = header.ok_or(ConversionError::MissingField("message.header"))?;
    let header = MessageHeader {
        num_required_signatures: to_u8(
            header.num_required_signatures,
            "message.header.num_required_signatures",
        )?,
        num_readonly_signed_accounts: to_u8(
            header.num_readonly_signed_accounts,
            "message.header.num_readonly_signed_accounts",
        )?,
        num_readonly_unsigned_accounts: to_u8(
            header.num_readonly_unsigned_accounts,
            "message.header.num_readonly_unsigned_accounts",
        )?,
    };
    let recent_blockhash = create_hash(recent_blockhash)?;
    let account_keys = create_pubkey_vec(account_keys, "message.account_keys")?;
    let instructions = create_message_instructions(instructions)?;

    // `config` presence distinguishes V1 from V0.
    if let Some(config) = config {
        return Ok(VersionedMessage::V1(MessageV1 {
            header,
            config: TransactionConfig {
                priority_fee: config.priority_fee,
                compute_unit_limit: config.compute_unit_limit,
                loaded_accounts_data_size_limit: config.loaded_accounts_data_size_limit,
                heap_size: config.heap_size,
            },
            lifetime_specifier: recent_blockhash,
            account_keys,
            instructions,
        }));
    }

    if versioned {
        let address_table_lookups = address_table_lookups
            .into_iter()
            .map(|lookup| {
                Ok(MessageAddressTableLookup {
                    account_key: create_pubkey(
                        lookup.account_key,
                        "message.address_table_lookups.account_key",
                    )?,
                    writable_indexes: lookup.writable_indexes,
                    readonly_indexes: lookup.readonly_indexes,
                })
            })
            .collect::<ConversionResult<Vec<_>>>()?;

        Ok(VersionedMessage::V0(MessageV0 {
            header,
            account_keys,
            recent_blockhash,
            instructions,
            address_table_lookups,
        }))
    } else {
        Ok(VersionedMessage::Legacy(LegacyMessage {
            header,
            account_keys,
            recent_blockhash,
            instructions,
        }))
    }
}

fn create_message_instructions(
    instructions: Vec<proto::CompiledInstruction>,
) -> ConversionResult<Vec<CompiledInstruction>> {
    instructions
        .into_iter()
        .map(create_message_instruction)
        .collect()
}

fn create_message_instruction(
    instruction: proto::CompiledInstruction,
) -> ConversionResult<CompiledInstruction> {
    Ok(CompiledInstruction {
        program_id_index: to_u8(
            instruction.program_id_index,
            "message.instructions.program_id_index",
        )?,
        accounts: instruction.accounts,
        data: instruction.data,
    })
}

/// Convert all transaction execution metadata emitted by Yellowstone.
pub fn create_tx_meta(
    meta: proto::TransactionStatusMeta,
) -> ConversionResult<TransactionStatusMeta> {
    let proto::TransactionStatusMeta {
        err,
        fee,
        pre_balances,
        post_balances,
        inner_instructions,
        inner_instructions_none,
        log_messages,
        log_messages_none,
        pre_token_balances,
        post_token_balances,
        rewards,
        loaded_writable_addresses,
        loaded_readonly_addresses,
        return_data,
        return_data_none,
        compute_units_consumed,
        cost_units,
    } = meta;

    let status = match create_tx_error(err.as_ref())? {
        Some(error) => Err(error),
        None => Ok(()),
    };
    let inner_instructions = if inner_instructions_none {
        None
    } else {
        Some(create_meta_inner_instructions(inner_instructions)?)
    };
    let log_messages = (!log_messages_none).then_some(log_messages);
    let return_data = if return_data_none {
        None
    } else {
        let data = return_data.ok_or(ConversionError::MissingField("meta.return_data"))?;
        Some(TransactionReturnData {
            program_id: create_pubkey(data.program_id, "meta.return_data.program_id")?,
            data: data.data,
        })
    };

    Ok(TransactionStatusMeta {
        status,
        fee,
        pre_balances,
        post_balances,
        inner_instructions,
        log_messages,
        pre_token_balances: Some(create_token_balances(pre_token_balances)?),
        post_token_balances: Some(create_token_balances(post_token_balances)?),
        rewards: Some(
            rewards
                .into_iter()
                .map(create_reward)
                .collect::<ConversionResult<Vec<_>>>()?,
        ),
        loaded_addresses: create_loaded_addresses(
            loaded_writable_addresses,
            loaded_readonly_addresses,
        )?,
        return_data,
        compute_units_consumed,
        cost_units,
    })
}

fn create_tx_error(
    error: Option<&proto::TransactionError>,
) -> ConversionResult<Option<TransactionError>> {
    error
        .map(|error| {
            wincode::deserialize::<TransactionError>(&error.err)
                .map_err(ConversionError::InvalidTransactionError)
        })
        .transpose()
}

fn create_meta_inner_instructions(
    instructions: Vec<proto::InnerInstructions>,
) -> ConversionResult<Vec<InnerInstructions>> {
    instructions
        .into_iter()
        .map(create_meta_inner_instruction)
        .collect()
}

fn create_meta_inner_instruction(
    instructions: proto::InnerInstructions,
) -> ConversionResult<InnerInstructions> {
    Ok(InnerInstructions {
        index: to_u8(instructions.index, "meta.inner_instructions.index")?,
        instructions: instructions
            .instructions
            .into_iter()
            .map(|instruction| {
                Ok(InnerInstruction {
                    instruction: CompiledInstruction {
                        program_id_index: to_u8(
                            instruction.program_id_index,
                            "meta.inner_instructions.program_id_index",
                        )?,
                        accounts: instruction.accounts,
                        data: instruction.data,
                    },
                    stack_height: instruction.stack_height,
                })
            })
            .collect::<ConversionResult<Vec<_>>>()?,
    })
}

fn create_reward(reward: proto::Reward) -> ConversionResult<Reward> {
    let reward_type = match proto::RewardType::try_from(reward.reward_type).map_err(|_| {
        ConversionError::InvalidEnum {
            field: "meta.rewards.reward_type",
            value: reward.reward_type,
        }
    })? {
        proto::RewardType::Unspecified => None,
        proto::RewardType::Fee => Some(RewardType::Fee),
        proto::RewardType::Rent => Some(RewardType::Rent),
        proto::RewardType::Staking => Some(RewardType::Staking),
        proto::RewardType::Voting => Some(RewardType::Voting),
        proto::RewardType::DeactivatedStake => Some(RewardType::DeactivatedStake),
    };
    let commission = parse_optional_number(reward.commission, "meta.rewards.commission")?;
    let commission_bps =
        parse_optional_number(reward.commission_bps, "meta.rewards.commission_bps")?;

    Ok(Reward {
        pubkey: reward.pubkey,
        lamports: reward.lamports,
        post_balance: reward.post_balance,
        reward_type,
        commission,
        commission_bps,
    })
}

fn create_token_balances(
    balances: Vec<proto::TokenBalance>,
) -> ConversionResult<Vec<TransactionTokenBalance>> {
    balances
        .into_iter()
        .map(|balance| {
            let amount = balance
                .ui_token_amount
                .ok_or(ConversionError::MissingField(
                    "meta.token_balances.ui_token_amount",
                ))?;

            Ok(TransactionTokenBalance {
                account_index: to_u8(balance.account_index, "meta.token_balances.account_index")?,
                mint: balance.mint,
                ui_token_amount: UiTokenAmount {
                    ui_amount: Some(amount.ui_amount),
                    decimals: to_u8(amount.decimals, "meta.token_balances.decimals")?,
                    amount: amount.amount,
                    ui_amount_string: amount.ui_amount_string,
                },
                owner: balance.owner,
                program_id: balance.program_id,
            })
        })
        .collect()
}

fn create_loaded_addresses(
    writable: Vec<Vec<u8>>,
    readonly: Vec<Vec<u8>>,
) -> ConversionResult<LoadedAddresses> {
    Ok(LoadedAddresses {
        writable: create_pubkey_vec(writable, "meta.loaded_writable_addresses")?,
        readonly: create_pubkey_vec(readonly, "meta.loaded_readonly_addresses")?,
    })
}

fn create_hash(bytes: Vec<u8>) -> ConversionResult<Hash> {
    let actual = bytes.len();
    let bytes =
        <[u8; HASH_BYTES]>::try_from(bytes).map_err(|_| ConversionError::InvalidByteLength {
            field: "message.recent_blockhash",
            expected: HASH_BYTES,
            actual,
        })?;
    Ok(Hash::new_from_array(bytes))
}

fn create_pubkey_vec(pubkeys: Vec<Vec<u8>>, field: &'static str) -> ConversionResult<Vec<Pubkey>> {
    pubkeys
        .into_iter()
        .map(|pubkey| create_pubkey(pubkey, field))
        .collect()
}

fn create_pubkey(bytes: Vec<u8>, field: &'static str) -> ConversionResult<Pubkey> {
    let actual = bytes.len();
    Pubkey::try_from(bytes).map_err(|_| ConversionError::InvalidByteLength {
        field,
        expected: PUBKEY_BYTES,
        actual,
    })
}

fn to_u8(value: u32, field: &'static str) -> ConversionResult<u8> {
    value
        .try_into()
        .map_err(|_| ConversionError::IntegerOutOfRange { field, value })
}

fn parse_optional_number<T>(value: String, field: &'static str) -> ConversionResult<Option<T>>
where
    T: std::str::FromStr,
{
    if value.is_empty() {
        Ok(None)
    } else {
        value
            .parse()
            .map(Some)
            .map_err(|_| ConversionError::InvalidNumber { field, value })
    }
}

#[cfg(test)]
mod tests {
    use {super::*, yellowstone_grpc_proto::prost::Message as ProstMessage};

    fn proto_header() -> proto::MessageHeader {
        proto::MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        }
    }

    fn proto_instruction() -> proto::CompiledInstruction {
        proto::CompiledInstruction {
            program_id_index: 1,
            accounts: vec![0],
            data: vec![4, 5, 6],
        }
    }

    fn proto_message() -> proto::Message {
        proto::Message {
            header: Some(proto_header()),
            account_keys: vec![vec![1; PUBKEY_BYTES], vec![2; PUBKEY_BYTES]],
            recent_blockhash: vec![3; HASH_BYTES],
            instructions: vec![proto_instruction()],
            versioned: false,
            address_table_lookups: vec![],
            config: None,
        }
    }

    #[test]
    fn converts_legacy_and_v0_messages() {
        let legacy = create_message(proto_message()).unwrap();
        assert!(matches!(legacy, VersionedMessage::Legacy(_)));

        let mut v0 = proto_message();
        v0.versioned = true;
        v0.address_table_lookups = vec![proto::MessageAddressTableLookup {
            account_key: vec![8; PUBKEY_BYTES],
            writable_indexes: vec![1, 2],
            readonly_indexes: vec![3],
        }];

        let VersionedMessage::V0(v0) = create_message(v0).unwrap() else {
            panic!("expected a V0 message");
        };
        assert_eq!(v0.address_table_lookups.len(), 1);
        assert_eq!(
            v0.address_table_lookups[0].account_key,
            Pubkey::from([8; 32])
        );
        assert_eq!(v0.address_table_lookups[0].writable_indexes, vec![1, 2]);
        assert_eq!(v0.address_table_lookups[0].readonly_indexes, vec![3]);
    }

    #[test]
    fn protobuf_v1_round_trip_preserves_config_and_sdk_wire_format() {
        let config = proto::TransactionConfig {
            priority_fee: Some(1_234),
            compute_unit_limit: Some(500_000),
            loaded_accounts_data_size_limit: Some(65_536),
            heap_size: Some(64 * 1_024),
        };
        let mut message = proto_message();
        message.versioned = true;
        message.config = Some(config);
        message.address_table_lookups = vec![proto::MessageAddressTableLookup {
            account_key: vec![8; PUBKEY_BYTES],
            writable_indexes: vec![1],
            readonly_indexes: vec![],
        }];
        let proto_transaction = proto::Transaction {
            signatures: vec![vec![7; SIGNATURE_BYTES]],
            message: Some(message),
        };

        let protobuf_bytes = proto_transaction.encode_to_vec();
        let decoded = proto::Transaction::decode(protobuf_bytes.as_slice()).unwrap();
        let transaction = create_tx_versioned(decoded).unwrap();

        let expected = VersionedTransaction {
            signatures: vec![Signature::from([7; SIGNATURE_BYTES])],
            message: VersionedMessage::V1(MessageV1 {
                header: MessageHeader {
                    num_required_signatures: 1,
                    num_readonly_signed_accounts: 0,
                    num_readonly_unsigned_accounts: 1,
                },
                config: TransactionConfig {
                    priority_fee: Some(1_234),
                    compute_unit_limit: Some(500_000),
                    loaded_accounts_data_size_limit: Some(65_536),
                    heap_size: Some(64 * 1_024),
                },
                lifetime_specifier: Hash::new_from_array([3; HASH_BYTES]),
                account_keys: vec![Pubkey::from([1; 32]), Pubkey::from([2; 32])],
                instructions: vec![CompiledInstruction {
                    program_id_index: 1,
                    accounts: vec![0],
                    data: vec![4, 5, 6],
                }],
            }),
        };
        assert_eq!(transaction, expected);

        let sdk_wire = wincode::serialize(&transaction).unwrap();
        assert_eq!(sdk_wire.first(), Some(&0x81));
        let sdk_round_trip: VersionedTransaction = wincode::deserialize(&sdk_wire).unwrap();
        assert_eq!(sdk_round_trip, transaction);
    }

    #[test]
    fn empty_config_presence_still_identifies_v1() {
        let mut message = proto_message();
        message.versioned = false;
        message.config = Some(proto::TransactionConfig::default());

        let protobuf_bytes = message.encode_to_vec();
        let decoded = proto::Message::decode(protobuf_bytes.as_slice()).unwrap();
        let VersionedMessage::V1(message) = create_message(decoded).unwrap() else {
            panic!("config presence must take precedence over the versioned flag");
        };

        assert_eq!(message.config, TransactionConfig::default());
    }

    #[test]
    fn protobuf_status_meta_round_trip_preserves_fields() {
        let transaction_error = TransactionError::AccountNotFound;
        let proto_meta = proto::TransactionStatusMeta {
            err: Some(proto::TransactionError {
                err: wincode::serialize(&transaction_error).unwrap(),
            }),
            fee: 5_000,
            pre_balances: vec![100, 200],
            post_balances: vec![90, 205],
            inner_instructions: vec![proto::InnerInstructions {
                index: 0,
                instructions: vec![proto::InnerInstruction {
                    program_id_index: 1,
                    accounts: vec![0],
                    data: vec![9],
                    stack_height: Some(2),
                }],
            }],
            inner_instructions_none: false,
            log_messages: vec!["program log".to_owned()],
            log_messages_none: false,
            pre_token_balances: vec![proto::TokenBalance {
                account_index: 0,
                mint: "mint".to_owned(),
                ui_token_amount: Some(proto::UiTokenAmount {
                    ui_amount: 1.25,
                    decimals: 6,
                    amount: "1250000".to_owned(),
                    ui_amount_string: "1.25".to_owned(),
                }),
                owner: "owner".to_owned(),
                program_id: "token-program".to_owned(),
            }],
            post_token_balances: vec![],
            rewards: vec![proto::Reward {
                pubkey: "vote-account".to_owned(),
                lamports: 25,
                post_balance: 1_025,
                reward_type: proto::RewardType::DeactivatedStake as i32,
                commission: "7".to_owned(),
                commission_bps: "750".to_owned(),
            }],
            loaded_writable_addresses: vec![vec![4; PUBKEY_BYTES]],
            loaded_readonly_addresses: vec![vec![5; PUBKEY_BYTES]],
            return_data: Some(proto::ReturnData {
                program_id: vec![6; PUBKEY_BYTES],
                data: vec![7, 8],
            }),
            return_data_none: false,
            compute_units_consumed: Some(12_345),
            cost_units: Some(12_500),
        };

        let protobuf_bytes = proto_meta.encode_to_vec();
        let decoded = proto::TransactionStatusMeta::decode(protobuf_bytes.as_slice()).unwrap();
        let meta = create_tx_meta(decoded).unwrap();

        assert_eq!(meta.status, Err(transaction_error));
        assert_eq!(meta.fee, 5_000);
        assert_eq!(meta.pre_balances, vec![100, 200]);
        assert_eq!(meta.post_balances, vec![90, 205]);
        assert_eq!(
            meta.inner_instructions,
            Some(vec![InnerInstructions {
                index: 0,
                instructions: vec![InnerInstruction {
                    instruction: CompiledInstruction {
                        program_id_index: 1,
                        accounts: vec![0],
                        data: vec![9],
                    },
                    stack_height: Some(2),
                }],
            }])
        );
        assert_eq!(meta.log_messages, Some(vec!["program log".to_owned()]));
        assert_eq!(meta.pre_token_balances.as_ref().unwrap().len(), 1);
        assert_eq!(meta.post_token_balances, Some(vec![]));
        assert_eq!(
            meta.rewards.as_ref().unwrap()[0].reward_type,
            Some(RewardType::DeactivatedStake)
        );
        assert_eq!(meta.rewards.as_ref().unwrap()[0].commission, Some(7));
        assert_eq!(meta.rewards.as_ref().unwrap()[0].commission_bps, Some(750));
        assert_eq!(meta.loaded_addresses.writable, vec![Pubkey::from([4; 32])]);
        assert_eq!(meta.loaded_addresses.readonly, vec![Pubkey::from([5; 32])]);
        assert_eq!(
            meta.return_data,
            Some(TransactionReturnData {
                program_id: Pubkey::from([6; 32]),
                data: vec![7, 8],
            })
        );
        assert_eq!(meta.compute_units_consumed, Some(12_345));
        assert_eq!(meta.cost_units, Some(12_500));
    }

    #[test]
    fn rejects_invalid_signature_hash_and_pubkey_lengths() {
        let error = create_tx_versioned(proto::Transaction {
            signatures: vec![vec![0; SIGNATURE_BYTES - 1]],
            message: Some(proto_message()),
        })
        .unwrap_err();
        assert!(matches!(
            error,
            ConversionError::InvalidByteLength {
                field: "transaction.signatures",
                expected: SIGNATURE_BYTES,
                actual: 63,
            }
        ));

        let mut invalid_hash = proto_message();
        invalid_hash.recent_blockhash = vec![0; HASH_BYTES - 1];
        let error = create_message(invalid_hash).unwrap_err();
        assert!(matches!(
            error,
            ConversionError::InvalidByteLength {
                field: "message.recent_blockhash",
                expected: HASH_BYTES,
                actual: 31,
            }
        ));

        let mut invalid_pubkey = proto_message();
        invalid_pubkey.account_keys[0] = vec![0; PUBKEY_BYTES - 1];
        let error = create_message(invalid_pubkey).unwrap_err();
        assert!(matches!(
            error,
            ConversionError::InvalidByteLength {
                field: "message.account_keys",
                expected: PUBKEY_BYTES,
                actual: 31,
            }
        ));
    }

    #[test]
    fn rejects_values_that_do_not_fit_sdk_indices() {
        let mut invalid_header = proto_message();
        invalid_header
            .header
            .as_mut()
            .unwrap()
            .num_required_signatures = u32::from(u8::MAX) + 1;
        let error = create_message(invalid_header).unwrap_err();
        assert!(matches!(
            error,
            ConversionError::IntegerOutOfRange {
                field: "message.header.num_required_signatures",
                value: 256,
            }
        ));

        let mut invalid_instruction = proto_message();
        invalid_instruction.instructions[0].program_id_index = u32::from(u8::MAX) + 1;
        let error = create_message(invalid_instruction).unwrap_err();
        assert!(matches!(
            error,
            ConversionError::IntegerOutOfRange {
                field: "message.instructions.program_id_index",
                value: 256,
            }
        ));
    }

    #[test]
    fn rejects_malformed_wincode_error_and_missing_return_data() {
        let error = create_tx_error(Some(&proto::TransactionError { err: vec![] })).unwrap_err();
        let ConversionError::InvalidTransactionError(source) = error else {
            panic!("expected a wincode decoding error");
        };
        assert!(!source.to_string().is_empty());

        let error = create_tx_meta(proto::TransactionStatusMeta {
            return_data: None,
            return_data_none: false,
            ..Default::default()
        })
        .unwrap_err();
        assert!(matches!(
            error,
            ConversionError::MissingField("meta.return_data")
        ));
    }

    #[test]
    fn status_meta_honors_none_flags() {
        let meta = create_tx_meta(proto::TransactionStatusMeta {
            inner_instructions: vec![proto::InnerInstructions {
                index: 0,
                instructions: vec![],
            }],
            inner_instructions_none: true,
            log_messages: vec!["ignored".to_owned()],
            log_messages_none: true,
            return_data: Some(proto::ReturnData {
                program_id: vec![6; PUBKEY_BYTES],
                data: vec![7, 8],
            }),
            return_data_none: true,
            ..Default::default()
        })
        .unwrap();

        assert_eq!(meta.inner_instructions, None);
        assert_eq!(meta.log_messages, None);
        assert_eq!(meta.return_data, None);
    }
}
