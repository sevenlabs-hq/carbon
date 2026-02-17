use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{AccountUpdate, Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
        metrics::{Counter, Histogram, MetricsRegistry},
    },
    solana_account_decoder::UiAccountEncoding,
    solana_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_config::RpcProgramAccountsConfig,
        rpc_request::RpcRequest,
        rpc_response::{OptionalContext, RpcKeyedAccount},
    },
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    std::str::FromStr,
    std::sync::{Arc, LazyLock},
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

static FETCH_DURATION_MILLIS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "rpc_gpa_fetch_duration_milliseconds",
        "Time to fetch program accounts from RPC in milliseconds",
        vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0],
    )
});
static ACCOUNTS_PROCESSED: Counter = Counter::new(
    "rpc_gpa_accounts_processed_total",
    "Account updates processed (sent) by RPC gPA datasource",
);

fn register_rpc_gpa_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&ACCOUNTS_PROCESSED);
    registry.register_histogram(&FETCH_DURATION_MILLIS);
}

pub struct GpaDatasource {
    pub rpc_url: String,
    pub program_id: Pubkey,
    pub config: RpcProgramAccountsConfig,
}

impl GpaDatasource {
    pub fn new(rpc_url: String, program_id: Pubkey) -> Self {
        Self::new_with_config(rpc_url, program_id, RpcProgramAccountsConfig::default())
    }

    pub const fn new_with_config(
        rpc_url: String,
        program_id: Pubkey,
        config: RpcProgramAccountsConfig,
    ) -> Self {
        Self {
            rpc_url,
            program_id,
            config,
        }
    }
}

#[async_trait]
impl Datasource for GpaDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        register_rpc_gpa_metrics();

        let commitment = self
            .config
            .account_config
            .commitment
            .unwrap_or(CommitmentConfig::confirmed());

        let rpc_client = RpcClient::new_with_commitment(self.rpc_url.clone(), commitment);
        let mut rpc_config = self.config.clone();
        rpc_config.with_context = Some(true);
        rpc_config.account_config.encoding = Some(UiAccountEncoding::Base64);

        let params = serde_json::json!([self.program_id.to_string(), rpc_config]);

        let fetch_start = std::time::Instant::now();
        let Ok(OptionalContext::Context(rpc_response)) = rpc_client
            .send::<OptionalContext<Vec<RpcKeyedAccount>>>(RpcRequest::GetProgramAccounts, params)
            .await
        else {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Failed to fetch program accounts".to_string(),
            ));
        };
        let fetch_elapsed = fetch_start.elapsed();
        FETCH_DURATION_MILLIS.record(fetch_elapsed.as_millis() as f64);

        log::info!(
            "Fetched {} accounts for program {} (slot: {})",
            rpc_response.value.len(),
            self.program_id,
            rpc_response.context.slot
        );

        let id_for_loop = id.clone();

        for account in rpc_response.value {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested during account processing");
                break;
            }

            let update = Update::Account(AccountUpdate {
                pubkey: Pubkey::from_str(&account.pubkey).map_err(|e| {
                    carbon_core::error::Error::FailedToConsumeDatasource(format!(
                        "Failed to parse pubkey: {e}"
                    ))
                })?,
                account: account.account.decode().ok_or(
                    carbon_core::error::Error::FailedToConsumeDatasource(
                        "Failed to decode account".to_string(),
                    ),
                )?,
                slot: rpc_response.context.slot,
                transaction_signature: None,
            });

            if let Err(e) = sender.send((update, id_for_loop.clone())).await {
                log::error!("Failed to send account update: {e:?}");
            } else {
                ACCOUNTS_PROCESSED.inc();
            }
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
