use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{AccountUpdate, Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
        metrics::MetricsCollection,
    },
    solana_account::Account,
    solana_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_request::RpcRequest,
        rpc_response::{OptionalContext, RpcKeyedAccount},
    },
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    std::{str::FromStr, sync::Arc},
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

#[derive(Debug, Clone, Default)]
pub struct GpaDatasourceConfig {
    pub program_accounts_config: Option<RpcProgramAccountsConfig>,
}

impl GpaDatasourceConfig {
    pub const fn new(program_accounts_config: Option<RpcProgramAccountsConfig>) -> Self {
        Self {
            program_accounts_config,
        }
    }
}

pub struct GpaDatasource {
    pub rpc_url: String,
    pub program_id: Pubkey,
    pub config: GpaDatasourceConfig,
}

impl GpaDatasource {
    pub const fn new(rpc_url: String, program_id: Pubkey, config: GpaDatasourceConfig) -> Self {
        Self {
            rpc_url,
            program_id,
            config,
        }
    }

    pub fn new_with_defaults(rpc_url: String, program_id: Pubkey) -> Self {
        Self::new(rpc_url, program_id, GpaDatasourceConfig::default())
    }
}

#[async_trait]
impl Datasource for GpaDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let commitment = self
            .config
            .program_accounts_config
            .as_ref()
            .and_then(|config| config.account_config.commitment)
            .unwrap_or(CommitmentConfig::confirmed());

        let rpc_client = RpcClient::new_with_commitment(self.rpc_url.clone(), commitment);

        let start_time = std::time::Instant::now();

        let current_slot = match rpc_client.get_slot().await {
            Ok(slot) => slot,
            Err(e) => {
                return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                    format!("Failed to fetch current slot: {e}"),
                ));
            }
        };

        log::info!(
            "Starting account indexing for program {} (current_slot: {})",
            self.program_id,
            current_slot
        );

        let mut rpc_config = self
            .config
            .program_accounts_config
            .clone()
            .unwrap_or_else(|| RpcProgramAccountsConfig {
                account_config: RpcAccountInfoConfig {
                    encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
                    ..Default::default()
                },
                filters: None,
                with_context: None,
                sort_results: None,
            });

        rpc_config.with_context = Some(true);

        let params = serde_json::json!([self.program_id.to_string(), rpc_config]);

        let response = rpc_client
            .send::<OptionalContext<Vec<RpcKeyedAccount>>>(RpcRequest::GetProgramAccounts, params)
            .await;

        let (program_accounts, context_slot) = match response {
            Ok(OptionalContext::Context(rpc_response)) => {
                let slot = rpc_response.context.slot;
                let keyed_accounts = rpc_response.value;

                let accounts: Result<Vec<(Pubkey, solana_account_decoder::UiAccount)>, String> =
                    keyed_accounts
                        .into_iter()
                        .map(|keyed_account| {
                            let pubkey = Pubkey::from_str(&keyed_account.pubkey)
                                .map_err(|e| format!("Failed to parse pubkey: {e}"))?;
                            Ok((pubkey, keyed_account.account))
                        })
                        .collect();

                match accounts {
                    Ok(accs) => (accs, slot),
                    Err(e) => {
                        return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                            format!("Failed to parse program accounts: {e}"),
                        ));
                    }
                }
            }
            Ok(_) => {
                return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                    "Did not receive context in RPC response".to_string(),
                ));
            }
            Err(e) => {
                return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                    format!("Failed to fetch program accounts: {e}"),
                ));
            }
        };

        log::info!(
            "Fetched {} accounts for program {} (slot: {})",
            program_accounts.len(),
            self.program_id,
            context_slot
        );

        let id_for_loop = id.clone();
        let mut accounts_sent = 0u64;
        let mut accounts_failed = 0u64;

        for (pubkey, account) in program_accounts {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested during account processing");
                break;
            }

            let decoded_account: Account = match account.decode() {
                Some(acc) => acc,
                None => {
                    log::warn!("Failed to decode account: {pubkey}");
                    accounts_failed += 1;
                    continue;
                }
            };

            let account_slot = context_slot;

            let update = Update::Account(AccountUpdate {
                pubkey,
                account: decoded_account,
                slot: account_slot,
                transaction_signature: None,
            });

            match sender.try_send((update, id_for_loop.clone())) {
                Ok(_) => {
                    accounts_sent += 1;
                }
                Err(e) => {
                    log::error!("Failed to send account update: {e:?}");
                    accounts_failed += 1;
                }
            }
        }

        let elapsed = start_time.elapsed();

        metrics
            .increment_counter("gpa_datasource_accounts_processed", accounts_sent)
            .await
            .unwrap_or_else(|e| log::error!("Error recording counter: {e}"));

        if accounts_failed > 0 {
            metrics
                .increment_counter("gpa_datasource_accounts_failed", accounts_failed)
                .await
                .unwrap_or_else(|e| log::error!("Error recording counter: {e}"));
        }

        metrics
            .record_histogram("gpa_datasource_duration_seconds", elapsed.as_secs_f64())
            .await
            .unwrap_or_else(|e| log::error!("Error recording histogram: {e}"));

        log::info!(
            "Account indexing completed: {} accounts sent, {} failed, took {:.2}s",
            accounts_sent,
            accounts_failed,
            elapsed.as_secs_f64()
        );

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
