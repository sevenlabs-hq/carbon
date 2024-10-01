use std::str::FromStr;

use crate::error::CarbonResult;
use solana_sdk::{instruction::CompiledInstruction, message::v0::LoadedAddresses, pubkey::Pubkey};
use solana_transaction_status::{
    option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
    TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiTransactionStatusMeta,
};

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
                .map(|tok| {
                    let owner = tok.owner.clone().ok_or_else(|| {
                        format!(
                            "Couldn't find the owner for account index {}",
                            tok.account_index
                        )
                    })?;

                    let program_id = tok.program_id.clone().ok_or_else(|| {
                        format!(
                            "Couldn't find a program id for account index {}",
                            tok.account_index
                        )
                    })?;

                    Ok(TransactionTokenBalance {
                        account_index: tok.account_index,
                        mint: tok.mint.clone(),
                        ui_token_amount: tok.ui_token_amount.clone(),
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
                                Ok(InnerInstruction {
                                    instruction: CompiledInstruction {
                                        program_id_index: ui_compiled_instruction.program_id_index,
                                        accounts: ui_compiled_instruction.accounts.clone(),
                                        data: bs58::decode(ui_compiled_instruction.data.clone())
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
                .map(|ptb| {
                    let owner = ptb.owner.clone().ok_or_else(|| {
                        format!(
                            "Couldn't find the owner for account index {}",
                            ptb.account_index
                        )
                    })?;

                    let program_id = ptb.program_id.clone().ok_or_else(|| {
                        format!(
                            "Couldn't find a program id for account index {}",
                            ptb.account_index
                        )
                    })?;

                    Ok(TransactionTokenBalance {
                        account_index: ptb.account_index,
                        mint: ptb.mint.clone(),
                        ui_token_amount: ptb.ui_token_amount.clone(),
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
