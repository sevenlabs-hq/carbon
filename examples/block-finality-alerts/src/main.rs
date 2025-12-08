use carbon_core::datasource::BlockDetails;
use solana_commitment_config::CommitmentConfig;
use solana_transaction_status::TransactionDetails;
use {
    async_trait::async_trait,
    carbon_core::{error::CarbonResult, metrics::MetricsCollection, processor::Processor},
    carbon_log_metrics::LogMetrics,
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let filters = Filters::new(
        RpcBlockSubscribeFilter::All,
        Some(RpcBlockSubscribeConfig {
            commitment: Some(CommitmentConfig::finalized()),
            transaction_details: Some(TransactionDetails::None),
            max_supported_transaction_version: Some(0),
            ..RpcBlockSubscribeConfig::default()
        }),
    );

    let rpc_ws_url =
        env::var("RPC_WS_URL").unwrap_or("wss://api.mainnet-beta.solana.com/".to_string());

    log::info!("Starting with RPC: {rpc_ws_url}");
    let block_subscribe = RpcBlockSubscribe::new(rpc_ws_url, filters);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(block_subscribe)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .block_details(BlockProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct BlockProcessor;

#[async_trait]
impl Processor for BlockProcessor {
    type InputType = BlockDetails;

    async fn process(
        &mut self,
        block_details: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::info!("Final block: {:?}", &block_details);

        Ok(())
    }
}
