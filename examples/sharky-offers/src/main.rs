use async_trait::async_trait;
use carbon_core::account::{AccountMetadata, DecodedAccount};
use carbon_core::datasource::{AccountUpdate, Datasource, Update, UpdateType};
use carbon_core::error::CarbonResult;
use carbon_core::metrics::Metrics;
use carbon_core::processor::Processor;
use sharky_decoder::accounts::SharkyAccount;
use sharky_decoder::SharkyDecoder;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use carbon_core::pipeline::{Pipeline, ShutdownStrategy};
use carbon_rpc_program_subscribe_datasource::{Filters, RpcProgramSubscribe};
use log_metrics::LogMetrics;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub mod sharky_decoder;

pub const SHARKY_PROGRAM_ID: Pubkey = pubkey!("SHARKobtfF1bHhxD2eqftjHBdVSCbKo9JtgK71FhELP");

pub struct GpaBackfillDatasource {
    pub rpc_url: String,
    pub program_id: Pubkey,
    pub config: Option<RpcProgramAccountsConfig>,
}

impl GpaBackfillDatasource {
    pub fn new(
        rpc_url: String,
        program_id: Pubkey,
        config: Option<RpcProgramAccountsConfig>,
    ) -> Self {
        Self {
            rpc_url,
            program_id,
            config,
        }
    }
}

#[async_trait]
impl Datasource for GpaBackfillDatasource {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
        _cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        let rpc_client = RpcClient::new(self.rpc_url.clone());

        let Ok(slot) = rpc_client.get_slot().await else {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Failed to fetch slot".to_string(),
            ));
        };

        let program_accounts = match &self.config {
            Some(config) => {
                rpc_client
                    .get_program_accounts_with_config(&self.program_id, config.clone())
                    .await
            }
            None => rpc_client.get_program_accounts(&self.program_id).await,
        };

        let Ok(program_accounts) = program_accounts else {
            return Err(carbon_core::error::Error::FailedToConsumeDatasource(
                "Failed to fetch program accounts".to_string(),
            ));
        };

        for (pubkey, account) in program_accounts {
            _ = sender.send(Update::Account(AccountUpdate {
                pubkey,
                account,
                slot,
            }));
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<carbon_core::datasource::UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}

pub struct SharkyAccountProcessor;

#[async_trait]
impl Processor for SharkyAccountProcessor {
    type InputType = (AccountMetadata, DecodedAccount<SharkyAccount>);

    async fn process(
        &mut self,
        update: Self::InputType,
        _metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()> {
        let (_metadata, account) = update;

        match account.data {
            SharkyAccount::OrderBook(_order_book) => {} // store each collection/order book in a database
            SharkyAccount::Loan(_loan) => {}            // store each loan in a database
            _ => {}                                     // uninteresting account type(s)
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    Pipeline::builder()
        .datasource(GpaBackfillDatasource::new(
            env::var("RPC_URL")?,
            SHARKY_PROGRAM_ID,
            None,
        ))
        .datasource(RpcProgramSubscribe::new(
            env::var("RPC_URL")?,
            Filters::new(SHARKY_PROGRAM_ID, None),
            Arc::new(RwLock::new(HashSet::new())),
        ))
        .account(SharkyDecoder, SharkyAccountProcessor)
        .metrics(Arc::new(LogMetrics::new()))
        .shutdown_strategy(ShutdownStrategy::ProcessPending)
        .build()?
        .run()
        .await?;

    Ok(())
}
