use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
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
        .metrics_flush_interval(3)
        .instruction(OpenbookV2Decoder, OpenbookV2InstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct OpenbookV2InstructionProcessor;

#[async_trait]
impl Processor for OpenbookV2InstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<OpenbookV2Instruction>,
        NestedInstructions,
        solana_instruction::Instruction,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions, _): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;

        match instruction.data {
            OpenbookV2Instruction::CreateMarket(create_market) => {
                log::info!(
                    "CreateMarket: signature: {signature}, create_market: {create_market:?}"
                );
            }
            OpenbookV2Instruction::CloseMarket(close_market) => {
                log::info!("CloseMarket: signature: {signature}, close_market: {close_market:?}");
            }
            OpenbookV2Instruction::CreateOpenOrdersIndexer(create_open_orders_indexer) => {
                log::info!(
                    "CreateOpenOrdersIndexer: signature: {signature}, create_open_orders_indexer: {create_open_orders_indexer:?}"
                );
            }
            OpenbookV2Instruction::CloseOpenOrdersIndexer(close_open_orders_indexer) => {
                log::info!(
                    "CloseOpenOrdersIndexer: signature: {signature}, close_open_orders_indexer: {close_open_orders_indexer:?}"
                );
            }
            OpenbookV2Instruction::CreateOpenOrdersAccount(create_open_orders_account) => {
                log::info!(
                    "CreateOpenOrdersAccount: signature: {signature}, create_open_orders_account: {create_open_orders_account:?}"
                );
            }
            OpenbookV2Instruction::CloseOpenOrdersAccount(close_open_orders_account) => {
                log::info!(
                    "CloseOpenOrdersAccount: signature: {signature}, close_open_orders_account: {close_open_orders_account:?}"
                );
            }
            OpenbookV2Instruction::PlaceOrder(place_order) => {
                log::info!("PlaceOrder: signature: {signature}, place_order: {place_order:?}");
            }
            OpenbookV2Instruction::EditOrder(edit_order) => {
                log::info!("EditOrder: signature: {signature}, edit_order: {edit_order:?}");
            }
            OpenbookV2Instruction::EditOrderPegged(edit_order_pegged) => {
                log::info!(
                    "EditOrderPegged: signature: {signature}, edit_order_pegged: {edit_order_pegged:?}"
                );
            }
            OpenbookV2Instruction::PlaceOrders(place_orders) => {
                log::info!("PlaceOrders: signature: {signature}, place_orders: {place_orders:?}");
            }
            OpenbookV2Instruction::CancelAllAndPlaceOrders(cancel_all_and_place_orders) => {
                log::info!(
                    "CancelAllAndPlaceOrders: signature: {signature}, cancel_all_and_place_orders: {cancel_all_and_place_orders:?}"
                );
            }
            OpenbookV2Instruction::PlaceOrderPegged(place_order_pegged) => {
                log::info!(
                    "PlaceOrderPegged: signature: {signature}, place_order_pegged: {place_order_pegged:?}"
                );
            }
            OpenbookV2Instruction::PlaceTakeOrder(place_take_order) => {
                log::info!(
                    "PlaceTakeOrder: signature: {signature}, place_take_order: {place_take_order:?}"
                );
            }
            OpenbookV2Instruction::ConsumeEvents(consume_events) => {
                log::info!(
                    "ConsumeEvents: signature: {signature}, consume_events: {consume_events:?}"
                );
            }
            OpenbookV2Instruction::ConsumeGivenEvents(consume_given_events) => {
                log::info!(
                    "ConsumeGivenEvents: signature: {signature}, consume_given_events: {consume_given_events:?}"
                );
            }
            OpenbookV2Instruction::CancelOrder(cancel_order) => {
                log::info!("CancelOrder: signature: {signature}, cancel_order: {cancel_order:?}");
            }
            OpenbookV2Instruction::CancelOrderByClientOrderId(cancel_order_by_client_order_id) => {
                log::info!(
                    "CancelOrderByClientOrderId: signature: {signature}, cancel_order_by_client_order_id: {cancel_order_by_client_order_id:?}"
                );
            }
            OpenbookV2Instruction::CancelAllOrders(cancel_all_orders) => {
                log::info!(
                    "CancelAllOrders: signature: {signature}, cancel_all_orders: {cancel_all_orders:?}"
                );
            }
            OpenbookV2Instruction::Deposit(deposit) => {
                log::info!("Deposit: signature: {signature}, deposit: {deposit:?}");
            }
            OpenbookV2Instruction::Refill(refill) => {
                log::info!("Refill: signature: {signature}, refill: {refill:?}");
            }
            OpenbookV2Instruction::SettleFunds(settle_funds) => {
                log::info!("SettleFunds: signature: {signature}, settle_funds: {settle_funds:?}");
            }
            OpenbookV2Instruction::SettleFundsExpired(settle_funds_expired) => {
                log::info!(
                    "SettleFundsExpired: signature: {signature}, settle_funds_expired: {settle_funds_expired:?}"
                );
            }
            OpenbookV2Instruction::SweepFees(sweep_fees) => {
                log::info!("SweepFees: signature: {signature}, sweep_fees: {sweep_fees:?}");
            }
            OpenbookV2Instruction::SetDelegate(set_delegate) => {
                log::info!("SetDelegate: signature: {signature}, set_delegate: {set_delegate:?}");
            }
            OpenbookV2Instruction::SetMarketExpired(set_market_expired) => {
                log::info!(
                    "SetMarketExpired: signature: {signature}, set_market_expired: {set_market_expired:?}"
                );
            }
            OpenbookV2Instruction::PruneOrders(prune_orders) => {
                log::info!("PruneOrders: signature: {signature}, prune_orders: {prune_orders:?}");
            }
            OpenbookV2Instruction::StubOracleCreate(stub_oracle_create) => {
                log::info!(
                    "StubOracleCreate: signature: {signature}, stub_oracle_create: {stub_oracle_create:?}"
                );
            }
            OpenbookV2Instruction::StubOracleClose(stub_oracle_close) => {
                log::info!(
                    "StubOracleClose: signature: {signature}, stub_oracle_close: {stub_oracle_close:?}"
                );
            }
            OpenbookV2Instruction::StubOracleSet(stub_oracle_set) => {
                log::info!(
                    "StubOracleSet: signature: {signature}, stub_oracle_set: {stub_oracle_set:?}"
                );
            }
            OpenbookV2Instruction::DepositLogEvent(deposit_log_event) => {
                log::info!(
                    "DepositLogEvent: signature: {signature}, deposit_log_event: {deposit_log_event:?}"
                );
            }
            OpenbookV2Instruction::FillLogEvent(fill_log_event) => {
                log::info!(
                    "FillLogEvent: signature: {signature}, fill_log_event: {fill_log_event:?}"
                );
            }
            OpenbookV2Instruction::MarketMetaDataLogEvent(market_meta_data_log_event) => {
                log::info!(
                    "MarketMetaDataLogEvent: signature: {signature}, market_meta_data_log_event: {market_meta_data_log_event:?}"
                );
            }
            OpenbookV2Instruction::TotalOrderFillEvent(total_order_fill_event) => {
                log::info!(
                    "TotalOrderFillEvent: signature: {signature}, total_order_fill_event: {total_order_fill_event:?}"
                );
            }
            OpenbookV2Instruction::SetDelegateLogEvent(set_delegate_log_event) => {
                log::info!(
                    "SetDelegateLogEvent: signature: {signature}, set_delegate_log_event: {set_delegate_log_event:?}"
                );
            }
            OpenbookV2Instruction::SettleFundsLogEvent(settle_funds_log_event) => {
                log::info!(
                    "SettleFundsLogEvent: signature: {signature}, settle_funds_log_event: {settle_funds_log_event:?}"
                );
            }
            OpenbookV2Instruction::SweepFeesLogEvent(sweep_fees_log_event) => {
                log::info!(
                    "SweepFeesLogEvent: signature: {signature}, sweep_fees_log_event: {sweep_fees_log_event:?}"
                );
            }
            OpenbookV2Instruction::OpenOrdersPositionLogEvent(open_orders_position_log_event) => {
                log::info!(
                    "OpenOrdersPositionLogEvent: signature: {signature}, open_orders_position_log_event: {open_orders_position_log_event:?}"
                );
            }
        };

        Ok(())
    }
}
