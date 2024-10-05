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
use std::{str::FromStr, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;

const REQUEST_TIMEOUT_MS: i32 = 10 * 60 * 1000; // 10 mins

pub struct BigQueryDatasource {
    pub credentials_path: Option<Arc<String>>,
    pub project_id: Arc<String>,
    pub rpc_client: Arc<RpcClient>,
    pub program_ids: Arc<Vec<String>>,
    pub start_slot: u64,
    pub end_slot: u64,
}

impl BigQueryDatasource {
    /// #### Create a new BigQuery datasource intstance.
    /// **Note**: the framework simplifies authentication with Google Cloud BigQuery by using Application Default Credentials (ADC).
    /// If you do not provide a path to a service account key file, the framework will attempt to use ADC.
    ///
    pub fn new(
        credentials_path: Option<String>,
        project_id: String,
        rpc_url: String,
        program_ids: Vec<String>,
        start_slot: u64,
        end_slot: u64,
    ) -> Self {
        Self {
            credentials_path: if let Some(credentials_path) = credentials_path {
                Some(Arc::new(credentials_path))
            } else {
                None
            },
            project_id: Arc::new(project_id),
            rpc_client: Arc::new(RpcClient::new(rpc_url)),
            program_ids: Arc::new(program_ids),
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
    ) -> CarbonResult<Vec<tokio::task::AbortHandle>> {
        log::info!("Consuming BigQuery Datasource...");
        let client = if let Some(credentials_path) = self.credentials_path.clone() {
            Arc::new(
                Client::from_service_account_key_file(&credentials_path)
                    .await
                    .map_err(|err| {
                        carbon_core::error::Error::Custom(format!(
                            "Failed to create BigQuery client from the credentials path: {}",
                            err.to_string()
                        ))
                    })?,
            )
        } else {
            Arc::new(
                Client::from_application_default_credentials()
                    .await
                    .map_err(|err| {
                        carbon_core::error::Error::Custom(format!(
                            "Failed to create BigQuery client from application default credentials, user needs to authenticate with Google Cloud: {}",
                            err.to_string()
                        ))
                    })?,
            )
        };

        let sender = Arc::new(sender.clone());
        let project_id = Arc::clone(&self.project_id);
        let program_ids = Arc::clone(&self.program_ids);
        let rpc_client = Arc::clone(&self.rpc_client);
        let start_slot = self.start_slot;
        let end_slot = self.end_slot;

        let accounts_abort_handle = tokio::spawn({
            let client = Arc::clone(&client);
            let sender = Arc::clone(&sender);
            let project_id = Arc::clone(&project_id);
            let program_ids = Arc::clone(&program_ids);

            async move {
                fetch_and_process_accounts(
                    &client,
                    &project_id,
                    program_ids.as_slice(),
                    start_slot,
                    end_slot,
                    &sender,
                )
                .await;
            }
        })
        .abort_handle();

        let transactions_abort_handle = tokio::spawn({
            let client = Arc::clone(&client);
            let sender = Arc::clone(&sender);
            let project_id = Arc::clone(&project_id);
            let program_ids = Arc::clone(&program_ids);
            let rpc_client = Arc::clone(&rpc_client);

            async move {
                fetch_and_process_transactions(
                    &client,
                    &project_id,
                    &rpc_client,
                    program_ids.as_slice(),
                    start_slot,
                    end_slot,
                    &sender,
                )
                .await;
            }
        })
        .abort_handle();

        Ok(vec![accounts_abort_handle, transactions_abort_handle])
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate, UpdateType::Transaction]
    }
}

async fn fetch_and_process_accounts(
    client: &Client,
    project_id: &str,
    program_ids: &[String],
    start_slot: u64,
    end_slot: u64,
    sender: &UnboundedSender<Update>,
) {
    log::info!("Fetching accounts...");
    println!("Fetching accounts...");
    let program_ids_list = program_ids
        .iter()
        .map(|id| format!("'{}'", id))
        .collect::<Vec<String>>()
        .join(", ");

    let base_account_query = QueryRequest::new(format!(
        "SELECT pubkey, block_slot, owner, lamports, data, executable, rent_epoch
         FROM `bigquery-public-data.crypto_solana_mainnet_us.Accounts`
         WHERE block_slot BETWEEN {start_slot} AND {end_slot}
         AND owner IN ({program_ids_list})"
    ));

    let account_query = QueryRequest {
        timeout_ms: Some(REQUEST_TIMEOUT_MS),
        ..base_account_query
    };

    println!("Running query..");
    match client.job().query(project_id, account_query).await {
        Ok(mut account_rows) => {
            println!("Total rows: {}", account_rows.row_count());
            while account_rows.next_row() {
                match parse_account_row(&account_rows) {
                    Ok(account_update) => {
                        println!("Got account update: {:?}", account_update);
                        _ = sender.send(Update::Account(account_update));
                    }
                    Err(err) => {
                        log::error!("Error parsing account row: {}", err);
                        println!("Error parsing account row: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Error fetching accounts: {}", err.to_string());
            return;
        }
    }
}

async fn fetch_and_process_transactions(
    client: &Client,
    project_id: &str,
    rpc_client: &RpcClient,
    program_ids: &[String],
    start_slot: u64,
    end_slot: u64,
    sender: &UnboundedSender<Update>,
) {
    log::info!("Fetching transactions...");
    println!("Fetching transactions...");

    let program_ids_list = program_ids
        .iter()
        .map(|id| format!("'{}'", id))
        .collect::<Vec<String>>()
        .join(", ");
    let base_transaction_query = QueryRequest::new(format!(
        "SELECT signature
         FROM `bigquery-public-data.crypto_solana_mainnet_us.Transactions`
         WHERE block_slot BETWEEN {start_slot} AND {end_slot}
         AND EXISTS (
             SELECT 1 FROM UNNEST(accounts) AS account
             WHERE account.pubkey IN ({program_ids_list})
         )",
    ));

    let transaction_query = QueryRequest {
        timeout_ms: Some(REQUEST_TIMEOUT_MS),
        ..base_transaction_query
    };

    match client.job().query(project_id, transaction_query).await {
        Ok(mut transaction_rows) => {
            while transaction_rows.next_row() {
                match parse_transaction_row(&transaction_rows, rpc_client).await {
                    Ok(transaction_update) => {
                        _ = sender.send(Update::Transaction(transaction_update));
                    }
                    Err(err) => {
                        log::error!("Error parsing transaction row: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Error fetching transactions: {}", err.to_string());
            return;
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

    let lamports = match result_set.get_i64_by_name("lamports") {
        Ok(Some(lamports)) => lamports,
        _ => 0,
    };

    let account_data = match result_set.get_json_value_by_name("data") {
        Ok(Some(value)) => {
            let data_array: Vec<AccountDataJson> = match serde_json::from_value(value) {
                Ok(array) => array,
                Err(err) => {
                    log::error!("Couldn't deserialize account data array: {}", err);
                    vec![]
                }
            };

            if let Some(data_object) = data_array.get(0) {
                if let (Some(raw_str), Some(encoding_str)) =
                    (&data_object.raw, &data_object.encoding)
                {
                    match encoding_str.as_str() {
                        "base64" => {
                            match base64::engine::general_purpose::STANDARD.decode(&raw_str) {
                                Ok(data_vec) => data_vec,
                                Err(err) => {
                                    log::error!("Couldn't decode base64 account data: {}", err);
                                    vec![]
                                }
                            }
                        }
                        "json" => {
                            if let Ok(json_data) = serde_json::from_str::<Value>(&raw_str) {
                                match serde_json::to_vec(&json_data) {
                                    Ok(data_vec) => data_vec,
                                    Err(err) => {
                                        log::error!(
                                            "Couldn't serialize JSON account data: {}",
                                            err
                                        );
                                        vec![]
                                    }
                                }
                            } else {
                                log::error!("Couldn't parse JSON account data");
                                vec![]
                            }
                        }
                        _ => {
                            log::error!("Unknown account data encoding: {}", encoding_str);
                            vec![]
                        }
                    }
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        }
        _ => {
            log::error!("Failed to get 'data' from ResultSet");
            vec![]
        }
    };

    let executable = match result_set.get_bool_by_name("executable") {
        Ok(Some(value)) => value,
        _ => false,
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
