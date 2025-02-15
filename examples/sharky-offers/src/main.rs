use async_trait::async_trait;
use carbon_core::account::AccountProcessorInputType;
use carbon_core::datasource::{AccountUpdate, Datasource, Update, UpdateType};
use carbon_core::error::CarbonResult;
use carbon_core::instruction::InstructionDecoder;
use carbon_core::instruction_decoder_collection;
use carbon_core::metrics::MetricsCollection;
use carbon_core::processor::Processor;
use carbon_sharky_decoder::accounts::SharkyAccount;
use carbon_sharky_decoder::instructions::SharkyInstruction;
use carbon_sharky_decoder::instructions::SharkyInstructionType;
use carbon_sharky_decoder::SharkyDecoder;
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

use carbon_core::pipeline::{Pipeline, ShutdownStrategy};
use carbon_log_metrics::LogMetrics;
use carbon_rpc_program_subscribe_datasource::{Filters, RpcProgramSubscribe};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

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
        _metrics: Arc<MetricsCollection>,
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
            if let Err(e) = sender.send(Update::Account(AccountUpdate {
                pubkey,
                account,
                slot,
            })) {
                println!("\nFailed to send account update: {:?}", e);
            }
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
    type InputType = AccountProcessorInputType<SharkyAccount>;

    async fn process(
        &mut self,
        update: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (_metadata, account) = update;

        match account.data {
            SharkyAccount::OrderBook(order_book) => {
                println!("Orderbook: {:?}", &order_book);
            }
            SharkyAccount::Loan(loan) => {
                println!("Loan: {:?}", &loan);
            }
            _ => {}
        }

        Ok(())
    }
}

instruction_decoder_collection!(
    AllInstructions, AllInstructionsType, AllPrograms,
    Sharky => SharkyDecoder => SharkyInstruction
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    Pipeline::builder()
        .datasource(GpaBackfillDatasource::new(
            env::var("RPC_URL").unwrap_or_default(),
            SHARKY_PROGRAM_ID,
            None,
        ))
        .datasource(RpcProgramSubscribe::new(
            // Websocket RPC url, usually starts with "wss://"
            env::var("RPC_WS_URL").unwrap_or_default(),
            Filters::new(
                SHARKY_PROGRAM_ID,
                Some(RpcProgramAccountsConfig {
                    filters: None,
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ),
        ))
        .account(SharkyDecoder, SharkyAccountProcessor)
        .metrics(Arc::new(LogMetrics::new()))
        .shutdown_strategy(ShutdownStrategy::ProcessPending)
        .build()?
        .run()
        .await?;

    Ok(())
}
