use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
    },
    carbon_helius_laserstream_datasource::{LaserStreamClientConfig, LaserStreamGeyserClient},
    carbon_jito_shredstream_grpc_datasource::JitoShredstreamGrpcClient,
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
        time::Duration,
    },
    tokio::sync::mpsc::Sender,
    tokio::sync::RwLock,
    tokio_util::sync::CancellationToken,
    yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterTransactions},
};

pub enum RealtimeVariant {
    Yellowstone(YellowstoneGrpcGeyserClient),
    Laserstream(LaserStreamGeyserClient),
    JitoShredstream(JitoShredstreamGrpcClient),
}

pub fn from_env(
    transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
) -> RealtimeVariant {
    match env::var("REALTIME_SOURCE")
        .unwrap_or_else(|_| "yellowstone".to_string())
        .as_str()
    {
        "laserstream" => RealtimeVariant::Laserstream(laserstream(transaction_filters)),
        "jito-shredstream" => RealtimeVariant::JitoShredstream(jito_shredstream()),
        "yellowstone" => RealtimeVariant::Yellowstone(yellowstone(transaction_filters)),
        other => {
            panic!(
                "unsupported REALTIME_SOURCE={other}; use yellowstone, laserstream, or jito-shredstream"
            )
        }
    }
}

fn yellowstone(
    transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
) -> YellowstoneGrpcGeyserClient {
    YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").expect("GEYSER_URL must be set"),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        HashMap::default(),
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        YellowstoneGrpcClientConfig::default(),
        None,
        None,
    )
}

fn laserstream(
    transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
) -> LaserStreamGeyserClient {
    let config = LaserStreamClientConfig::new(
        None,
        Some(Duration::from_secs(15)),
        Some(Duration::from_secs(15)),
        None,
        None,
        None,
        true,
    );

    LaserStreamGeyserClient::new(
        env::var("LASERSTREAM_URL").expect("LASERSTREAM_URL must be set"),
        env::var("API_KEY").ok(),
        Some(CommitmentLevel::Confirmed),
        HashMap::default(),
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        config,
    )
}

fn jito_shredstream() -> JitoShredstreamGrpcClient {
    JitoShredstreamGrpcClient::new(
        env::var("JITO_SHREDSTREAM_URL").expect("JITO_SHREDSTREAM_URL must be set"),
    )
}

#[async_trait]
impl Datasource for RealtimeVariant {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        match self {
            Self::Yellowstone(datasource) => {
                datasource.consume(id, sender, cancellation_token).await
            }
            Self::Laserstream(datasource) => {
                datasource.consume(id, sender, cancellation_token).await
            }
            Self::JitoShredstream(datasource) => {
                datasource.consume(id, sender, cancellation_token).await
            }
        }
    }

    fn update_types(&self) -> Vec<UpdateType> {
        match self {
            Self::Yellowstone(datasource) => datasource.update_types(),
            Self::Laserstream(datasource) => datasource.update_types(),
            Self::JitoShredstream(datasource) => datasource.update_types(),
        }
    }
}
