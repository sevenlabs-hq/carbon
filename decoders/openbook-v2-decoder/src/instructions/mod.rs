use {super::OpenbookV2Decoder, crate::PROGRAM_ID};
pub mod cancel_all_and_place_orders;
pub mod cancel_all_orders;
pub mod cancel_order;
pub mod cancel_order_by_client_order_id;
pub mod close_market;
pub mod close_open_orders_account;
pub mod close_open_orders_indexer;
pub mod consume_events;
pub mod consume_given_events;
pub mod create_market;
pub mod create_open_orders_account;
pub mod create_open_orders_indexer;
pub mod deposit;
pub mod deposit_log_event;
pub mod edit_order;
pub mod edit_order_pegged;
pub mod fill_log_event;
pub mod market_meta_data_log_event;
pub mod open_orders_position_log_event;
pub mod place_order;
pub mod place_order_pegged;
pub mod place_orders;
pub mod place_take_order;
pub mod prune_orders;
pub mod refill;
pub mod set_delegate;
pub mod set_delegate_log_event;
pub mod set_market_expired;
pub mod settle_funds;
pub mod settle_funds_expired;
pub mod settle_funds_log_event;
pub mod stub_oracle_close;
pub mod stub_oracle_create;
pub mod stub_oracle_set;
pub mod sweep_fees;
pub mod sweep_fees_log_event;
pub mod total_order_fill_event;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum OpenbookV2Instruction {
    CreateMarket(create_market::CreateMarket),
    CloseMarket(close_market::CloseMarket),
    CreateOpenOrdersIndexer(create_open_orders_indexer::CreateOpenOrdersIndexer),
    CloseOpenOrdersIndexer(close_open_orders_indexer::CloseOpenOrdersIndexer),
    CreateOpenOrdersAccount(create_open_orders_account::CreateOpenOrdersAccount),
    CloseOpenOrdersAccount(close_open_orders_account::CloseOpenOrdersAccount),
    PlaceOrder(place_order::PlaceOrder),
    EditOrder(edit_order::EditOrder),
    EditOrderPegged(edit_order_pegged::EditOrderPegged),
    PlaceOrders(place_orders::PlaceOrders),
    CancelAllAndPlaceOrders(cancel_all_and_place_orders::CancelAllAndPlaceOrders),
    PlaceOrderPegged(place_order_pegged::PlaceOrderPegged),
    PlaceTakeOrder(place_take_order::PlaceTakeOrder),
    ConsumeEvents(consume_events::ConsumeEvents),
    ConsumeGivenEvents(consume_given_events::ConsumeGivenEvents),
    CancelOrder(cancel_order::CancelOrder),
    CancelOrderByClientOrderId(cancel_order_by_client_order_id::CancelOrderByClientOrderId),
    CancelAllOrders(cancel_all_orders::CancelAllOrders),
    Deposit(deposit::Deposit),
    Refill(refill::Refill),
    SettleFunds(settle_funds::SettleFunds),
    SettleFundsExpired(settle_funds_expired::SettleFundsExpired),
    SweepFees(sweep_fees::SweepFees),
    SetDelegate(set_delegate::SetDelegate),
    SetMarketExpired(set_market_expired::SetMarketExpired),
    PruneOrders(prune_orders::PruneOrders),
    StubOracleCreate(stub_oracle_create::StubOracleCreate),
    StubOracleClose(stub_oracle_close::StubOracleClose),
    StubOracleSet(stub_oracle_set::StubOracleSet),
    DepositLogEvent(deposit_log_event::DepositLogEvent),
    FillLogEvent(fill_log_event::FillLogEvent),
    MarketMetaDataLogEvent(market_meta_data_log_event::MarketMetaDataLogEvent),
    TotalOrderFillEvent(total_order_fill_event::TotalOrderFillEvent),
    SetDelegateLogEvent(set_delegate_log_event::SetDelegateLogEvent),
    SettleFundsLogEvent(settle_funds_log_event::SettleFundsLogEvent),
    SweepFeesLogEvent(sweep_fees_log_event::SweepFeesLogEvent),
    OpenOrdersPositionLogEvent(open_orders_position_log_event::OpenOrdersPositionLogEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for OpenbookV2Decoder {
    type InstructionType = OpenbookV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            OpenbookV2Instruction::CreateMarket => create_market::CreateMarket,
            OpenbookV2Instruction::CloseMarket => close_market::CloseMarket,
            OpenbookV2Instruction::CreateOpenOrdersIndexer => create_open_orders_indexer::CreateOpenOrdersIndexer,
            OpenbookV2Instruction::CloseOpenOrdersIndexer => close_open_orders_indexer::CloseOpenOrdersIndexer,
            OpenbookV2Instruction::CreateOpenOrdersAccount => create_open_orders_account::CreateOpenOrdersAccount,
            OpenbookV2Instruction::CloseOpenOrdersAccount => close_open_orders_account::CloseOpenOrdersAccount,
            OpenbookV2Instruction::PlaceOrder => place_order::PlaceOrder,
            OpenbookV2Instruction::EditOrder => edit_order::EditOrder,
            OpenbookV2Instruction::EditOrderPegged => edit_order_pegged::EditOrderPegged,
            OpenbookV2Instruction::PlaceOrders => place_orders::PlaceOrders,
            OpenbookV2Instruction::CancelAllAndPlaceOrders => cancel_all_and_place_orders::CancelAllAndPlaceOrders,
            OpenbookV2Instruction::PlaceOrderPegged => place_order_pegged::PlaceOrderPegged,
            OpenbookV2Instruction::PlaceTakeOrder => place_take_order::PlaceTakeOrder,
            OpenbookV2Instruction::ConsumeEvents => consume_events::ConsumeEvents,
            OpenbookV2Instruction::ConsumeGivenEvents => consume_given_events::ConsumeGivenEvents,
            OpenbookV2Instruction::CancelOrder => cancel_order::CancelOrder,
            OpenbookV2Instruction::CancelOrderByClientOrderId => cancel_order_by_client_order_id::CancelOrderByClientOrderId,
            OpenbookV2Instruction::CancelAllOrders => cancel_all_orders::CancelAllOrders,
            OpenbookV2Instruction::Deposit => deposit::Deposit,
            OpenbookV2Instruction::Refill => refill::Refill,
            OpenbookV2Instruction::SettleFunds => settle_funds::SettleFunds,
            OpenbookV2Instruction::SettleFundsExpired => settle_funds_expired::SettleFundsExpired,
            OpenbookV2Instruction::SweepFees => sweep_fees::SweepFees,
            OpenbookV2Instruction::SetDelegate => set_delegate::SetDelegate,
            OpenbookV2Instruction::SetMarketExpired => set_market_expired::SetMarketExpired,
            OpenbookV2Instruction::PruneOrders => prune_orders::PruneOrders,
            OpenbookV2Instruction::StubOracleCreate => stub_oracle_create::StubOracleCreate,
            OpenbookV2Instruction::StubOracleClose => stub_oracle_close::StubOracleClose,
            OpenbookV2Instruction::StubOracleSet => stub_oracle_set::StubOracleSet,
            OpenbookV2Instruction::DepositLogEvent => deposit_log_event::DepositLogEvent,
            OpenbookV2Instruction::FillLogEvent => fill_log_event::FillLogEvent,
            OpenbookV2Instruction::MarketMetaDataLogEvent => market_meta_data_log_event::MarketMetaDataLogEvent,
            OpenbookV2Instruction::TotalOrderFillEvent => total_order_fill_event::TotalOrderFillEvent,
            OpenbookV2Instruction::SetDelegateLogEvent => set_delegate_log_event::SetDelegateLogEvent,
            OpenbookV2Instruction::SettleFundsLogEvent => settle_funds_log_event::SettleFundsLogEvent,
            OpenbookV2Instruction::SweepFeesLogEvent => sweep_fees_log_event::SweepFeesLogEvent,
            OpenbookV2Instruction::OpenOrdersPositionLogEvent => open_orders_position_log_event::OpenOrdersPositionLogEvent,
        )
    }
}
