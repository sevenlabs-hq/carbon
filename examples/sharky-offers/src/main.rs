use {
    async_trait::async_trait,
    carbon_core::{
        account::AccountProcessorInputType,
        datasource::{AccountUpdate, Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
        instruction::InstructionDecoder,
        instruction_decoder_collection,
        metrics::MetricsCollection,
        pipeline::{Pipeline, ShutdownStrategy},
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_rpc_program_subscribe_datasource::{Filters, RpcProgramSubscribe},
    carbon_sharky_decoder::{
        accounts::SharkyAccount,
        instructions::{SharkyInstruction, SharkyInstructionType},
        SharkyDecoder, PROGRAM_ID as SHARKY_PROGRAM_ID,
    },
    solana_account_decoder::UiAccountEncoding,
    solana_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    },
    solana_pubkey::Pubkey,
    std::{env, sync::Arc},
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

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
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
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

        let id_for_loop = id.clone();

        for (pubkey, account) in program_accounts {
            if let Err(e) = sender.try_send((
                Update::Account(AccountUpdate {
                    pubkey,
                    account,
                    slot,
                }),
                id_for_loop.clone(),
            )) {
                log::error!("Failed to send account update: {:?}", e);
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
        let (_metadata, account, _raw_account) = update;

        match account.data {
            SharkyAccount::OrderBook(order_book) => {
                log::info!("Orderbook: {:?}", &order_book);
            }
            SharkyAccount::Loan(loan) => {
                log::info!("Loan: {:?}", &loan);
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
    dotenv::dotenv().ok();
    env_logger::init();

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
