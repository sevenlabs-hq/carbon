use {
    solana_account_decoder_client_types::token::UiTokenAmount,
    solana_message::v0::LoadedAddresses,
    solana_transaction_context::TransactionReturnData,
    solana_transaction_status::{Reward, TransactionStatusMeta, TransactionTokenBalance},
};

pub fn to_modern(
    legacy: legacy_solana_transaction_status::TransactionStatusMeta,
) -> TransactionStatusMeta {
    TransactionStatusMeta {
        status: legacy.status.map_err(|error| {
            super::transaction_error::convert_ui_transaction_error(error.to_string())
        }),
        fee: legacy.fee,
        pre_balances: legacy.pre_balances,
        post_balances: legacy.post_balances,
        inner_instructions: Some(super::compiled_instruction::convert_inner_instructions(
            legacy.inner_instructions,
        )),
        log_messages: Some(legacy.log_messages.unwrap_or_default()),
        pre_token_balances: Some(
            legacy
                .pre_token_balances
                .unwrap_or_default()
                .iter()
                .filter_map(|transaction_token_balance| {
                    Some(to_transaction_token_balance(transaction_token_balance))
                })
                .collect::<Vec<TransactionTokenBalance>>(),
        ),
        post_token_balances: Some(
            legacy
                .post_token_balances
                .unwrap_or_default()
                .iter()
                .filter_map(|transaction_token_balance| {
                    Some(to_transaction_token_balance(transaction_token_balance))
                })
                .collect::<Vec<TransactionTokenBalance>>(),
        ),
        rewards: Some(
            legacy
                .rewards
                .unwrap_or_default()
                .iter()
                .map(to_reward)
                .collect::<Vec<Reward>>(),
        ),
        loaded_addresses: to_loaded_addresses(legacy.loaded_addresses),
        return_data: legacy.return_data.map(|return_data| TransactionReturnData {
            program_id: super::pubkey::to_modern(&return_data.program_id),
            data: return_data.data,
        }),
        compute_units_consumed: legacy.compute_units_consumed.or(None),
        cost_units: legacy.cost_units,
    }
}

pub fn to_loaded_addresses(legacy: legacy_solana_message::v0::LoadedAddresses) -> LoadedAddresses {
    let bytes = bincode::serialize(&legacy).expect("serialize v2 laoded addresses");
    bincode::deserialize::<LoadedAddresses>(&bytes).expect("deserialize v3 loaded addresses")
}

pub fn to_reward(legacy: &legacy_solana_transaction_status::Reward) -> Reward {
    let bytes = bincode::serialize(&legacy).expect("serialize v2 reward");
    bincode::deserialize::<Reward>(&bytes).expect("deserialize v3 reward")
}

pub fn to_transaction_token_balance(
    legacy: &legacy_solana_transaction_status::TransactionTokenBalance,
) -> TransactionTokenBalance {
    let legacy_ui_token_amount = legacy.ui_token_amount.clone();

    TransactionTokenBalance {
        account_index: legacy.account_index,
        mint: legacy.mint.clone(),
        ui_token_amount: UiTokenAmount {
            ui_amount: legacy_ui_token_amount.ui_amount,
            ui_amount_string: legacy_ui_token_amount.ui_amount_string,
            decimals: legacy_ui_token_amount.decimals,
            amount: legacy_ui_token_amount.amount,
        },
        owner: legacy.owner.clone(),
        program_id: legacy.program_id.clone(),
    }
}
