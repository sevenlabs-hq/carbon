use async_trait::async_trait;
use base64::Engine;
use carbon_core::{
    datasource::{AccountUpdate, Datasource, TransactionUpdate, Update, UpdateType},
    error::CarbonResult,
    transformers::build_tx_status_meta,
};
use gcp_bigquery_client::{
    model::{query_request::QueryRequest, query_response::ResultSet},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signature};
use std::str::FromStr;
use tokio::sync::mpsc::UnboundedSender;

pub struct BigQueryDatasource {
    pub project_id: String,
    pub dataset: String,
    pub credentials_path: String,
    pub account_table: String,
    pub transaction_table: String,
    pub rpc_url: String,
    pub program_id: String,
    pub start_slot: u64,
    pub end_slot: u64,
}

impl BigQueryDatasource {
    pub fn new(
        project_id: String,
        dataset: String,
        credentials_path: String,
        account_table: String,
        transaction_table: String,
        rpc_url: String,
        program_id: String,
        start_slot: u64,
        end_slot: u64,
    ) -> Self {
        Self {
            project_id,
            dataset,
            credentials_path,
            account_table,
            transaction_table,
            rpc_url,
            program_id,
            start_slot,
            end_slot,
        }
    }
}

#[async_trait]
impl Datasource for BigQueryDatasource {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let sender = sender.clone();
        let credentials_path = self.credentials_path.clone();
        let project_id = self.project_id.clone();
        let dataset = self.dataset.clone();
        let account_table = self.account_table.clone();
        let transaction_table = self.transaction_table.clone();
        let start_slot = self.start_slot;
        let end_slot = self.end_slot;
        let rpc_client = RpcClient::new(self.rpc_url.clone());
        let program_id = self.program_id.clone();

        let client = Client::from_service_account_key_file(&credentials_path)
            .await
            .map_err(|err| {
                carbon_core::error::Error::Custom(format!(
                    "Failed to create BigQuery client: {}",
                    err.to_string()
                ))
            })?;

        let abort_handle = tokio::spawn(async move {
            fetch_and_process_accounts(
                &client,
                &dataset,
                &project_id,
                &account_table,
                &program_id,
                start_slot,
                end_slot,
                &sender,
            )
            .await;

            fetch_and_process_transactions(
                &client,
                &rpc_client,
                &dataset,
                &project_id,
                &transaction_table,
                &program_id,
                start_slot,
                end_slot,
                &sender,
            )
            .await;
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate, UpdateType::Transaction]
    }
}

async fn fetch_and_process_accounts(
    client: &Client,
    dataset: &str,
    project_id: &str,
    account_table: &str,
    program_id: &str,
    start_slot: u64,
    end_slot: u64,
    sender: &UnboundedSender<Update>,
) {
    let account_query = QueryRequest::new(format!(
        "SELECT pubkey, block_slot, owner, lamports, data, executable, rent_epoch
         FROM `{project_id}.{dataset}.{account_table}`
         WHERE slot BETWEEN {start_slot} AND {end_slot}
         AND owner = '{program_id}'",
    ));

    if let Ok(mut account_rows) = client.job().query(project_id, account_query).await {
        while account_rows.next_row() {
            if let Ok(account_update) = parse_account_row(&account_rows) {
                _ = sender.send(Update::Account(account_update));
            }
        }
    }
}

async fn fetch_and_process_transactions(
    client: &Client,
    rpc_client: &RpcClient,
    dataset: &str,
    project_id: &str,
    transaction_table: &str,
    program_id: &str,
    start_slot: u64,
    end_slot: u64,
    sender: &UnboundedSender<Update>,
) {
    let transaction_query = QueryRequest::new(format!(
        "SELECT signature
         FROM `{project_id}.{dataset}.{transaction_table}`
         WHERE slot BETWEEN {start_slot} AND {end_slot}
         AND EXISTS (
             SELECT 1 FROM UNNEST(accounts) AS account
             WHERE account.pubkey = '{program_id}'
         )",
    ));

    if let Ok(mut transaction_rows) = client.job().query(project_id, transaction_query).await {
        while transaction_rows.next_row() {
            if let Ok(transaction_update) =
                parse_transaction_row(&transaction_rows, rpc_client).await
            {
                _ = sender.send(Update::Transaction(transaction_update));
            }
        }
    }
}

fn parse_account_row(result_set: &ResultSet) -> CarbonResult<AccountUpdate> {
    #[derive(Debug, Deserialize, Serialize)]
    struct AccountDataJson {
        raw: Option<String>,
        encoding: Option<String>,
    }

    let Some(pubkey_str) = result_set.get_string_by_name("pubkey").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account pubkey is null"
        )));
    };

    let pubkey = Pubkey::from_str(&pubkey_str).map_err(|err| {
        carbon_core::error::Error::Custom(format!("Error parsing account pubkey: {err}"))
    })?;

    let Some(slot) = result_set.get_i64_by_name("block_slot").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account pubkey is null"
        )));
    };

    let Some(owner_str) = result_set.get_string_by_name("owner").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account owner is null"
        )));
    };
    let owner = Pubkey::from_str(&owner_str).map_err(|err| {
        carbon_core::error::Error::Custom(format!("Error parsing account's owner pubkey: {err}"))
    })?;

    let Some(lamports) = result_set.get_i64_by_name("lamports").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account lamports is null"
        )));
    };

    let Some(data_value) = result_set.get_json_value_by_name("data").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account data is null"
        )));
    };

    let data_array: Vec<AccountDataJson> = serde_json::from_value(data_value).map_err(|_| {
        carbon_core::error::Error::Custom(
            "Couldn't deserialize Account Data's into AccountDataJson struct".to_string(),
        )
    })?;

    let Some(data_object) = data_array.get(0) else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account data is empty"
        )));
    };

    let Some(raw_str) = data_object.raw.clone() else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(
            "Couldn't get Account Data's Raw field".to_string(),
        ));
    };
    let Some(encoding_str) = data_object.encoding.clone() else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(
            "Couldn't get Account Data's Encoding field".to_string(),
        ));
    };

    let account_data = match encoding_str.as_str() {
        "base64" => base64::engine::general_purpose::STANDARD
            .decode(raw_str)
            .map_err(|err| {
                carbon_core::error::Error::Custom(format!(
                    "Couldn't decode raw account data: {err}"
                ))
            })?,
        "json" => {
            let json_data: Value = serde_json::from_str(&raw_str).map_err(|err| {
                carbon_core::error::Error::Custom(format!(
                    "Couldn't decode raw account data from JSON: {err}"
                ))
            })?;

            serde_json::to_vec(&json_data).map_err(|err| {
                carbon_core::error::Error::Custom(format!(
                    "Couldn't serialize account data from JSON: {err}"
                ))
            })?
        }
        _ => {
            return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
                "Couldn't process Account data encoding: {encoding_str}"
            )));
        }
    };

    let Some(executable) = result_set.get_bool_by_name("executable").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account executable is null"
        )));
    };
    let Some(rent_epoch) = result_set.get_i64_by_name("rent_epoch").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: account epoch is null"
        )));
    };

    let account = Account {
        lamports: lamports as u64,
        data: account_data,
        owner,
        executable,
        rent_epoch: rent_epoch as u64,
    };

    Ok(AccountUpdate {
        pubkey,
        account,
        slot: slot as u64,
    })
}

async fn parse_transaction_row(
    result_set: &ResultSet,
    rpc_client: &RpcClient,
) -> CarbonResult<TransactionUpdate> {
    let Some(signature_str) = result_set.get_string_by_name("signature").map_err(|err| {
        carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: {}",
            err.to_string()
        ))
    })?
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Failed to get BigQuery ResultSet field: transaction signature is null"
        )));
    };

    let signature = Signature::from_str(&signature_str).map_err(|err| {
        carbon_core::error::Error::Custom(format!("Error parsing transaction signature: {err}"))
    })?;

    let Ok(tx) = rpc_client
        .get_transaction_with_config(
            &signature,
            RpcTransactionConfig {
                encoding: Some(solana_transaction_status::UiTransactionEncoding::Base64),
                commitment: Some(solana_sdk::commitment_config::CommitmentConfig {
                    commitment: solana_sdk::commitment_config::CommitmentLevel::Confirmed,
                }),
                max_supported_transaction_version: Some(2),
            },
        )
        .await
    else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Couldn't get transaction with config for the following signature: {}",
            &signature.to_string()
        )));
    };

    let Some(decoded_tx) = tx.transaction.transaction.decode() else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(format!(
            "Couldn't decode transaction for signature: {}",
            &signature.to_string()
        )));
    };

    let Some(meta_original) = tx.transaction.meta else {
        return CarbonResult::Err(carbon_core::error::Error::Custom(
            "Couldn't get transaction meta".to_string(),
        ));
    };

    let tx_status_meta = build_tx_status_meta(meta_original).map_err(|err| {
        carbon_core::error::Error::Custom(format!("Couldn't build transaction meta: {err}"))
    })?;

    Ok(TransactionUpdate {
        signature,
        transaction: decoded_tx,
        meta: tx_status_meta,
        is_vote: false,
        slot: tx.slot,
    })
}
