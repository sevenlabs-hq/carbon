use {
    solana_message::v0::LoadedAddresses,
    solana_pubkey::Pubkey,
    solana_transaction_context::TransactionReturnData,
    solana_transaction_status::{
        option_serializer::OptionSerializer, Reward, TransactionStatusMeta,
        TransactionTokenBalance, UiLoadedAddresses, UiTransactionTokenBalance,
    },
    std::str::FromStr,
};

pub fn to_transaction_status_meta(
    legacy: legacy_solana_transaction_status::UiTransactionStatusMeta,
) -> TransactionStatusMeta {
    TransactionStatusMeta {
        status: legacy.status.map_err(|error| {
            super::transaction_error::convert_ui_transaction_error(error.to_string())
        }),
        fee: legacy.fee,
        pre_balances: legacy.pre_balances,
        post_balances: legacy.post_balances,
        inner_instructions: Some(super::compiled_instruction::convert_inner_instructions(
            legacy.inner_instructions.into(),
        )),
        log_messages: Some(legacy.log_messages.unwrap_or_else(std::vec::Vec::new)),
        pre_token_balances: Some(
            legacy
                .pre_token_balances
                .unwrap_or_else(std::vec::Vec::new)
                .iter()
                .filter_map(|transaction_token_balance| {
                    let transaction_token_balance =
                        to_transaction_token_balance(transaction_token_balance);

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
            legacy
                .post_token_balances
                .unwrap_or_else(std::vec::Vec::new)
                .iter()
                .filter_map(|transaction_token_balance| {
                    let transaction_token_balance =
                        to_transaction_token_balance(transaction_token_balance);

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
            legacy
                .rewards
                .unwrap_or_else(std::vec::Vec::new)
                .iter()
                .map(to_reward)
                .collect::<Vec<Reward>>(),
        ),

        loaded_addresses: {
            let loaded = legacy.loaded_addresses.unwrap_or_else(|| {
                legacy_solana_transaction_status::UiLoadedAddresses {
                    writable: vec![],
                    readonly: vec![],
                }
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
        return_data: legacy.return_data.map(|return_data| TransactionReturnData {
            program_id: return_data.program_id.parse().unwrap_or_default(),
            data: return_data.data.0.as_bytes().to_vec(),
        }),
        compute_units_consumed: legacy
            .compute_units_consumed
            .map(|compute_unit_consumed| compute_unit_consumed)
            .or(None),
        cost_units: legacy.cost_units.into(),
    }
}

pub fn to_ui_loaded_addresses(
    legacy: legacy_solana_transaction_status::UiLoadedAddresses,
) -> UiLoadedAddresses {
    let bytes = bincode::serialize(&legacy).expect("serialize v2 loaded addresses");
    bincode::deserialize::<UiLoadedAddresses>(&bytes).expect("deserialize v3 loaded addresses")
}

pub fn to_reward(legacy: &legacy_solana_transaction_status::Reward) -> Reward {
    let bytes = bincode::serialize(&legacy).expect("serialize v2 reward");
    bincode::deserialize::<Reward>(&bytes).expect("deserialize v3 reward")
}

pub fn to_transaction_token_balance(
    legacy: &legacy_solana_transaction_status::UiTransactionTokenBalance,
) -> UiTransactionTokenBalance {
    let bytes = bincode::serialize(&legacy).expect("serialize v2 transaction token balance");
    bincode::deserialize::<UiTransactionTokenBalance>(&bytes)
        .expect("deserialize v3 transaction token balance")
}
