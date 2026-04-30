#![allow(dead_code)]

use {
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
};

pub fn rpc(program: Pubkey) -> RpcTransactionCrawler {
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

pub fn helius_gtfa(program: Pubkey) -> HeliusGtfaDatasource {
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
