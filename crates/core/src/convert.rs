use {
    crate::transformers::transaction_metadata_from_original_meta,
    base64::Engine,
    serde::{Deserialize, Serialize},
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_status::{
        option_serializer::OptionSerializer, TransactionStatusMeta, UiCompiledInstruction,
        UiInnerInstructions, UiInstruction, UiLoadedAddresses, UiReturnDataEncoding,
        UiTransactionReturnData, UiTransactionStatusMeta, UiTransactionTokenBalance,
    },
};

pub fn to_ui_from_transaction_status_meta(
    value: &TransactionStatusMeta,
) -> UiTransactionStatusMeta {
    UiTransactionStatusMeta {
        err: None,
        status: Ok(()),
        fee: value.fee,
        pre_balances: value.pre_balances.clone(),
        post_balances: value.post_balances.clone(),
        inner_instructions: value.inner_instructions.as_ref().map_or(
            OptionSerializer::None,
            |inner_instructions| {
                OptionSerializer::Some(
                    inner_instructions
                        .iter()
                        .map(|inner_instruction| UiInnerInstructions {
                            index: inner_instruction.index,
                            instructions: inner_instruction
                                .instructions
                                .iter()
                                .map(|instruction| {
                                    UiInstruction::Compiled(UiCompiledInstruction::from(
                                        &instruction.instruction,
                                        instruction.stack_height,
                                    ))
                                })
                                .collect(),
                        })
                        .collect(),
                )
            },
        ),
        log_messages: value
            .log_messages
            .clone()
            .map_or(OptionSerializer::None, OptionSerializer::Some),
        pre_token_balances: value.pre_token_balances.as_ref().map_or(
            OptionSerializer::None,
            |pre_token_balances| {
                OptionSerializer::Some(
                    pre_token_balances
                        .iter()
                        .map(|pre_token_balance| UiTransactionTokenBalance {
                            account_index: pre_token_balance.account_index,
                            mint: pre_token_balance.mint.clone(),
                            ui_token_amount: pre_token_balance.ui_token_amount.clone(),
                            owner: OptionSerializer::Some(pre_token_balance.owner.clone()),
                            program_id: OptionSerializer::Some(
                                pre_token_balance.program_id.clone(),
                            ),
                        })
                        .collect(),
                )
            },
        ),
        post_token_balances: value.post_token_balances.as_ref().map_or(
            OptionSerializer::None,
            |post_token_balances| {
                OptionSerializer::Some(
                    post_token_balances
                        .iter()
                        .map(|post_token_balance| UiTransactionTokenBalance {
                            account_index: post_token_balance.account_index,
                            mint: post_token_balance.mint.clone(),
                            ui_token_amount: post_token_balance.ui_token_amount.clone(),
                            owner: OptionSerializer::Some(post_token_balance.owner.clone()),
                            program_id: OptionSerializer::Some(
                                post_token_balance.program_id.clone(),
                            ),
                        })
                        .collect(),
                )
            },
        ),
        rewards: value
            .rewards
            .clone()
            .map_or(OptionSerializer::None, OptionSerializer::Some),
        loaded_addresses: OptionSerializer::Some(UiLoadedAddresses {
            writable: value
                .loaded_addresses
                .writable
                .iter()
                .map(|writable| writable.to_string())
                .collect(),
            readonly: value
                .loaded_addresses
                .readonly
                .iter()
                .map(|readonly| readonly.to_string())
                .collect(),
        }),
        return_data: value
            .return_data
            .as_ref()
            .map_or(OptionSerializer::None, |return_data| {
                OptionSerializer::Some(UiTransactionReturnData {
                    program_id: return_data.program_id.to_string(),
                    data: (
                        base64::engine::general_purpose::STANDARD.encode(&return_data.data),
                        UiReturnDataEncoding::Base64,
                    ),
                })
            }),
        compute_units_consumed: value
            .compute_units_consumed
            .map_or(OptionSerializer::None, OptionSerializer::Some),
        cost_units: value
            .cost_units
            .map_or(OptionSerializer::None, OptionSerializer::Some),
    }
}

pub fn serialize_meta<S>(meta: &TransactionStatusMeta, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    to_ui_from_transaction_status_meta(meta).serialize(serializer)
}

pub fn deserialize_meta<'de, D>(deserializer: D) -> Result<TransactionStatusMeta, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let meta_original = UiTransactionStatusMeta::deserialize(deserializer)?;
    transaction_metadata_from_original_meta(meta_original).map_err(serde::de::Error::custom)
}

pub fn serialize_transaction<S>(tx: &VersionedTransaction, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes = bincode::serialize(tx).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(&bytes))
}

pub fn deserialize_transaction<'de, D>(deserializer: D) -> Result<VersionedTransaction, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(s)
        .map_err(serde::de::Error::custom)?;
    bincode::deserialize(&bytes).map_err(serde::de::Error::custom)
}
