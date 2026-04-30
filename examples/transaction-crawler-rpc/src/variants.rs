use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
    },
    carbon_helius_gtfa_datasource::{
        HeliusGtfaConfig, HeliusGtfaDatasource, HeliusGtfaFilters, SlotFilter, SortOrder,
        TransactionStatusFilter,
    },
    carbon_rpc_transaction_crawler_datasource::{
        ConnectionConfig, Filters, RetryConfig, RpcTransactionCrawler,
    },
    solana_commitment_config::CommitmentConfig,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::{env, str::FromStr, time::Duration},
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

pub enum TransactionCrawlerVariant {
    Rpc(RpcTransactionCrawler),
    HeliusGtfa(HeliusGtfaDatasource),
}

pub fn from_env(program: Pubkey) -> TransactionCrawlerVariant {
    match env::var("TRANSACTION_SOURCE")
        .unwrap_or_else(|_| "rpc".to_string())
        .as_str()
    {
        "helius-gtfa" => TransactionCrawlerVariant::HeliusGtfa(helius_gtfa(program)),
        "rpc" => TransactionCrawlerVariant::Rpc(rpc(program)),
        other => panic!("unsupported TRANSACTION_SOURCE={other}; use rpc or helius-gtfa"),
    }
}

fn rpc(program: Pubkey) -> RpcTransactionCrawler {
    let connection_config = ConnectionConfig::new(
        100,
        Duration::from_secs(5),
        5,
        RetryConfig::no_retry(),
        None,
        None,
        true,
    );

    let until_signature = env::var("UNTIL_SIGNATURE")
        .ok()
        .and_then(|s| Signature::from_str(&s).ok());

    let filters = Filters::new(None, None, until_signature);

    RpcTransactionCrawler::new(
        env::var("RPC_URL").expect("RPC_URL must be set"),
        program,
        connection_config,
        filters,
        Some(CommitmentConfig::finalized()),
    )
}

fn helius_gtfa(program: Pubkey) -> HeliusGtfaDatasource {
    let filters = HeliusGtfaFilters {
        slot: Some(SlotFilter {
            gte: Some(416_009_000),
            lte: Some(416_010_000),
            gt: None,
            lt: None,
        }),
        block_time: None, // alt: BlockTimeFilter for unix-time bounds
        signature: None,
        status: Some(TransactionStatusFilter::Succeeded), // alt: ::Any to keep failed
    };

    let config = HeliusGtfaConfig::new(
        Some(SortOrder::Asc),
        Some(100),
        Some(CommitmentConfig::confirmed()),
        Some(filters),
        None,
    );

    HeliusGtfaDatasource::new_with_config(
        env::var("HELIUS_RPC_URL").expect("HELIUS_RPC_URL must be set"),
        program,
        config,
    )
}

#[async_trait]
impl Datasource for TransactionCrawlerVariant {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        match self {
            Self::Rpc(datasource) => datasource.consume(id, sender, cancellation_token).await,
            Self::HeliusGtfa(datasource) => {
                datasource.consume(id, sender, cancellation_token).await
            }
        }
    }

    fn update_types(&self) -> Vec<UpdateType> {
        match self {
            Self::Rpc(datasource) => datasource.update_types(),
            Self::HeliusGtfa(datasource) => datasource.update_types(),
        }
    }
}
