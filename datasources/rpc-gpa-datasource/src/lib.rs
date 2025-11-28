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
    },
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    std::sync::Arc,
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

#[derive(Debug, Clone, Default)]
pub struct GpaDatasourceConfig {
    pub program_accounts_config: Option<RpcProgramAccountsConfig>,
    pub commitment: Option<CommitmentConfig>,
}

impl GpaDatasourceConfig {
    pub const fn new(
        program_accounts_config: Option<RpcProgramAccountsConfig>,
        commitment: Option<CommitmentConfig>,
    ) -> Self {
        Self {
            program_accounts_config,
            commitment,
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
        let rpc_client = RpcClient::new_with_commitment(
            self.rpc_url.clone(),
            self.config
                .commitment
                .unwrap_or(CommitmentConfig::confirmed()),
        );

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

        let rpc_config = self
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

        let program_accounts_result = rpc_client
            .get_program_ui_accounts_with_config(&self.program_id, rpc_config.clone())
            .await;

        let program_accounts = match program_accounts_result {
            Ok(accounts) => accounts,
            Err(e) => {
                return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                    format!("Failed to fetch program accounts: {e}"),
                ));
            }
        };

        log::info!(
            "Fetched {} accounts for program {}",
            program_accounts.len(),
            self.program_id
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

            let account_slot = current_slot;

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
