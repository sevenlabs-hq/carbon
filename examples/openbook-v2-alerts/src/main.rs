use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_openbook_v2_decoder::{
        instructions::OpenbookV2Instruction, OpenbookV2Decoder,
        PROGRAM_ID as OPENBOOK_V2_PROGRAM_ID,
    },
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let filters = Filters::new(
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(OPENBOOK_V2_PROGRAM_ID.to_string()),
        Some(RpcBlockSubscribeConfig {
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
        .instruction(OpenbookV2Decoder, OpenbookV2InstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct OpenbookV2TransactionProcessor;

pub struct OpenbookV2InstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, OpenbookV2Instruction>>
    for OpenbookV2InstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, OpenbookV2Instruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        match input.decoded_instruction {
            OpenbookV2Instruction::CreateMarket { data, .. } => {
                log::info!("CreateMarket: signature: {signature}, create_market: {data:?}");
            }
            OpenbookV2Instruction::CloseMarket { data, .. } => {
                log::info!("CloseMarket: signature: {signature}, close_market: {data:?}");
            }
            OpenbookV2Instruction::CreateOpenOrdersIndexer { data, .. } => {
                log::info!(
                    "CreateOpenOrdersIndexer: signature: {signature}, create_open_orders_indexer: {data:?}"
                );
            }
            OpenbookV2Instruction::CloseOpenOrdersIndexer { data, .. } => {
                log::info!(
                    "CloseOpenOrdersIndexer: signature: {signature}, close_open_orders_indexer: {data:?}"
                );
            }
            OpenbookV2Instruction::CreateOpenOrdersAccount { data, .. } => {
                log::info!(
                    "CreateOpenOrdersAccount: signature: {signature}, create_open_orders_account: {data:?}"
                );
            }
            OpenbookV2Instruction::CloseOpenOrdersAccount { data, .. } => {
                log::info!(
                    "CloseOpenOrdersAccount: signature: {signature}, close_open_orders_account: {data:?}"
                );
            }
            OpenbookV2Instruction::PlaceOrder { data, .. } => {
                log::info!("PlaceOrder: signature: {signature}, place_order: {data:?}");
            }
            OpenbookV2Instruction::EditOrder { data, .. } => {
                log::info!("EditOrder: signature: {signature}, edit_order: {data:?}");
            }
            OpenbookV2Instruction::EditOrderPegged { data, .. } => {
                log::info!("EditOrderPegged: signature: {signature}, edit_order_pegged: {data:?}");
            }
            OpenbookV2Instruction::PlaceOrders { data, .. } => {
                log::info!("PlaceOrders: signature: {signature}, place_orders: {data:?}");
            }
            OpenbookV2Instruction::CancelAllAndPlaceOrders { data, .. } => {
                log::info!(
                    "CancelAllAndPlaceOrders: signature: {signature}, cancel_all_and_place_orders: {data:?}"
                );
            }
            OpenbookV2Instruction::PlaceOrderPegged { data, .. } => {
                log::info!(
                    "PlaceOrderPegged: signature: {signature}, place_order_pegged: {data:?}"
                );
            }
            OpenbookV2Instruction::PlaceTakeOrder { data, .. } => {
                log::info!("PlaceTakeOrder: signature: {signature}, place_take_order: {data:?}");
            }
            OpenbookV2Instruction::ConsumeEvents { data, .. } => {
                log::info!("ConsumeEvents: signature: {signature}, consume_events: {data:?}");
            }
            OpenbookV2Instruction::ConsumeGivenEvents { data, .. } => {
                log::info!(
                    "ConsumeGivenEvents: signature: {signature}, consume_given_events: {data:?}"
                );
            }
            OpenbookV2Instruction::CancelOrder { data, .. } => {
                log::info!("CancelOrder: signature: {signature}, cancel_order: {data:?}");
            }
            OpenbookV2Instruction::CancelOrderByClientOrderId { data, .. } => {
                log::info!(
                    "CancelOrderByClientOrderId: signature: {signature}, cancel_order_by_client_order_id: {data:?}"
                );
            }
            OpenbookV2Instruction::CancelAllOrders { data, .. } => {
                log::info!("CancelAllOrders: signature: {signature}, cancel_all_orders: {data:?}");
            }
            OpenbookV2Instruction::Deposit { data, .. } => {
                log::info!("Deposit: signature: {signature}, deposit: {data:?}");
            }
            OpenbookV2Instruction::Refill { data, .. } => {
                log::info!("Refill: signature: {signature}, refill: {data:?}");
            }
            OpenbookV2Instruction::SettleFunds { data, .. } => {
                log::info!("SettleFunds: signature: {signature}, settle_funds: {data:?}");
            }
            OpenbookV2Instruction::SettleFundsExpired { data, .. } => {
                log::info!(
                    "SettleFundsExpired: signature: {signature}, settle_funds_expired: {data:?}"
                );
            }
            OpenbookV2Instruction::SweepFees { data, .. } => {
                log::info!("SweepFees: signature: {signature}, sweep_fees: {data:?}");
            }
            OpenbookV2Instruction::SetDelegate { data, .. } => {
                log::info!("SetDelegate: signature: {signature}, set_delegate: {data:?}");
            }
            OpenbookV2Instruction::SetMarketExpired { data, .. } => {
                log::info!(
                    "SetMarketExpired: signature: {signature}, set_market_expired: {data:?}"
                );
            }
            OpenbookV2Instruction::PruneOrders { data, .. } => {
                log::info!("PruneOrders: signature: {signature}, prune_orders: {data:?}");
            }
            OpenbookV2Instruction::StubOracleCreate { data, .. } => {
                log::info!(
                    "StubOracleCreate: signature: {signature}, stub_oracle_create: {data:?}"
                );
            }
            OpenbookV2Instruction::StubOracleClose { data, .. } => {
                log::info!("StubOracleClose: signature: {signature}, stub_oracle_close: {data:?}");
            }
            OpenbookV2Instruction::StubOracleSet { data, .. } => {
                log::info!("StubOracleSet: signature: {signature}, stub_oracle_set: {data:?}");
            }
            OpenbookV2Instruction::CpiEvent { data, .. } => {
                log::info!("CpiEvent: signature: {signature}, cpi_event: {data:?}");
            }
        }

        Ok(())
    }
}
