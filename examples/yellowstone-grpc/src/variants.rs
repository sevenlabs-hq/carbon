#![allow(dead_code)]

use {
    carbon_helius_laserstream_datasource::{LaserStreamClientConfig, LaserStreamGeyserClient},
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    std::{collections::HashMap, env, time::Duration},
    yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterTransactions},
};

pub fn yellowstone(
    transaction_filters: HashMap<String, SubscribeRequestFilterTransactions>,
) -> YellowstoneGrpcGeyserClient {
    YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").expect("GEYSER_URL must be set"),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        HashMap::default(),
        transaction_filters,
        Default::default(),
        YellowstoneGrpcClientConfig::default(),
        None,
        None,
    )
}

pub fn laserstream(
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
        None,
    );

    LaserStreamGeyserClient::new(
        env::var("LASERSTREAM_URL").expect("LASERSTREAM_URL must be set"),
        env::var("API_KEY").ok(),
        Some(CommitmentLevel::Confirmed),
        HashMap::default(),
        transaction_filters,
        Default::default(),
        config,
    )
}
