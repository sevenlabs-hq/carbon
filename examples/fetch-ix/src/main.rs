use serde::Serialize;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_commitment_config::CommitmentConfig;
use solana_signature::Signature;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiInstruction, UiMessage,
    UiParsedInstruction, UiParsedMessage, UiTransactionEncoding,
};
use std::{env, process};

#[derive(Serialize, Clone)]
struct AccountInfo {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
struct Output {
    accounts: Vec<AccountInfo>,
    data: String,
    program_id: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sig_str = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run -- <transaction-signature> <ix-id>");
        process::exit(1);
    });
    let signature = sig_str.parse::<Signature>()?;
    let rpc_url =
        env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    let tx = client.get_transaction_with_config(&signature, config)?;
    let EncodedConfirmedTransactionWithStatusMeta {
        transaction: enc_tx,
        ..
    } = tx;
    if let EncodedTransaction::Json(parsed_tx) = enc_tx.transaction {
        let UiMessage::Parsed(UiParsedMessage {
            account_keys,
            instructions,
            ..
        }) = parsed_tx.message
        else {
            return Err("Unsupported message format".into());
        };
        let accounts_infos: Vec<AccountInfo> = account_keys
            .into_iter()
            .map(|key| AccountInfo {
                pubkey: key.pubkey,
                is_signer: key.signer,
                is_writable: key.writable,
            })
            .collect();

        for ix in instructions {
            let (data_b58, program_id, accounts) = match ix {
                UiInstruction::Parsed(parsed_ix) => match parsed_ix {
                    UiParsedInstruction::Parsed(_) => {
                        continue;
                    }
                    UiParsedInstruction::PartiallyDecoded(ix) => {
                        (ix.data.clone(), ix.program_id.clone(), ix.accounts)
                    }
                },
                _ => panic!("Expected parsed"),
            };
            let data = hex::encode(bs58::decode(data_b58).into_vec()?);
            let filtered_accounts: Vec<AccountInfo> = accounts
                .iter()
                .filter_map(|acc| accounts_infos.iter().find(|ai| ai.pubkey == *acc).cloned())
                .collect();
            let output = Output {
                accounts: filtered_accounts,
                data,
                program_id,
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    } else {
        panic!("Unsupported transaction encoding");
    }
    Ok(())
}
