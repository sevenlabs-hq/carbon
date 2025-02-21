use super::PhoenixV1Decoder;
pub mod cancel_all_orders;
pub mod cancel_all_orders_with_free_funds;
pub mod cancel_multiple_orders_by_id;
pub mod cancel_multiple_orders_by_id_with_free_funds;
pub mod cancel_up_to;
pub mod cancel_up_to_with_free_funds;
pub mod change_fee_recipient;
pub mod change_market_status;
pub mod change_seat_status;
pub mod claim_authority;
pub mod collect_fees;
pub mod deposit_funds;
pub mod evict_seat;
pub mod force_cancel_orders;
pub mod initialize_market;
pub mod log;
pub mod name_successor;
pub mod place_limit_order;
pub mod place_limit_order_with_free_funds;
pub mod place_multiple_post_only_orders;
pub mod place_multiple_post_only_orders_with_free_funds;
pub mod reduce_order;
pub mod reduce_order_with_free_funds;
pub mod request_seat;
pub mod request_seat_authorized;
pub mod swap;
pub mod swap_with_free_funds;
pub mod withdraw_funds;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum PhoenixV1Instruction {
    Swap(swap::Swap),
    SwapWithFreeFunds(swap_with_free_funds::SwapWithFreeFunds),
    PlaceLimitOrder(place_limit_order::PlaceLimitOrder),
    PlaceLimitOrderWithFreeFunds(place_limit_order_with_free_funds::PlaceLimitOrderWithFreeFunds),
    ReduceOrder(reduce_order::ReduceOrder),
    ReduceOrderWithFreeFunds(reduce_order_with_free_funds::ReduceOrderWithFreeFunds),
    CancelAllOrders(cancel_all_orders::CancelAllOrders),
    CancelAllOrdersWithFreeFunds(cancel_all_orders_with_free_funds::CancelAllOrdersWithFreeFunds),
    CancelUpTo(cancel_up_to::CancelUpTo),
    CancelUpToWithFreeFunds(cancel_up_to_with_free_funds::CancelUpToWithFreeFunds),
    CancelMultipleOrdersById(cancel_multiple_orders_by_id::CancelMultipleOrdersById),
    CancelMultipleOrdersByIdWithFreeFunds(
        cancel_multiple_orders_by_id_with_free_funds::CancelMultipleOrdersByIdWithFreeFunds,
    ),
    WithdrawFunds(withdraw_funds::WithdrawFunds),
    DepositFunds(deposit_funds::DepositFunds),
    RequestSeat(request_seat::RequestSeat),
    Log(log::Log),
    PlaceMultiplePostOnlyOrders(place_multiple_post_only_orders::PlaceMultiplePostOnlyOrders),
    PlaceMultiplePostOnlyOrdersWithFreeFunds(
        place_multiple_post_only_orders_with_free_funds::PlaceMultiplePostOnlyOrdersWithFreeFunds,
    ),
    InitializeMarket(initialize_market::InitializeMarket),
    ClaimAuthority(claim_authority::ClaimAuthority),
    NameSuccessor(name_successor::NameSuccessor),
    ChangeMarketStatus(change_market_status::ChangeMarketStatus),
    ChangeSeatStatus(change_seat_status::ChangeSeatStatus),
    RequestSeatAuthorized(request_seat_authorized::RequestSeatAuthorized),
    EvictSeat(evict_seat::EvictSeat),
    ForceCancelOrders(force_cancel_orders::ForceCancelOrders),
    CollectFees(collect_fees::CollectFees),
    ChangeFeeRecipient(change_fee_recipient::ChangeFeeRecipient),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for PhoenixV1Decoder {
    type InstructionType = PhoenixV1Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            PhoenixV1Instruction::Swap => swap::Swap,
            PhoenixV1Instruction::SwapWithFreeFunds => swap_with_free_funds::SwapWithFreeFunds,
            PhoenixV1Instruction::PlaceLimitOrder => place_limit_order::PlaceLimitOrder,
            PhoenixV1Instruction::PlaceLimitOrderWithFreeFunds => place_limit_order_with_free_funds::PlaceLimitOrderWithFreeFunds,
            PhoenixV1Instruction::ReduceOrder => reduce_order::ReduceOrder,
            PhoenixV1Instruction::ReduceOrderWithFreeFunds => reduce_order_with_free_funds::ReduceOrderWithFreeFunds,
            PhoenixV1Instruction::CancelAllOrders => cancel_all_orders::CancelAllOrders,
            PhoenixV1Instruction::CancelAllOrdersWithFreeFunds => cancel_all_orders_with_free_funds::CancelAllOrdersWithFreeFunds,
            PhoenixV1Instruction::CancelUpTo => cancel_up_to::CancelUpTo,
            PhoenixV1Instruction::CancelUpToWithFreeFunds => cancel_up_to_with_free_funds::CancelUpToWithFreeFunds,
            PhoenixV1Instruction::CancelMultipleOrdersById => cancel_multiple_orders_by_id::CancelMultipleOrdersById,
            PhoenixV1Instruction::CancelMultipleOrdersByIdWithFreeFunds => cancel_multiple_orders_by_id_with_free_funds::CancelMultipleOrdersByIdWithFreeFunds,
            PhoenixV1Instruction::WithdrawFunds => withdraw_funds::WithdrawFunds,
            PhoenixV1Instruction::DepositFunds => deposit_funds::DepositFunds,
            PhoenixV1Instruction::RequestSeat => request_seat::RequestSeat,
            PhoenixV1Instruction::Log => log::Log,
            PhoenixV1Instruction::PlaceMultiplePostOnlyOrders => place_multiple_post_only_orders::PlaceMultiplePostOnlyOrders,
            PhoenixV1Instruction::PlaceMultiplePostOnlyOrdersWithFreeFunds => place_multiple_post_only_orders_with_free_funds::PlaceMultiplePostOnlyOrdersWithFreeFunds,
            PhoenixV1Instruction::InitializeMarket => initialize_market::InitializeMarket,
            PhoenixV1Instruction::ClaimAuthority => claim_authority::ClaimAuthority,
            PhoenixV1Instruction::NameSuccessor => name_successor::NameSuccessor,
            PhoenixV1Instruction::ChangeMarketStatus => change_market_status::ChangeMarketStatus,
            PhoenixV1Instruction::ChangeSeatStatus => change_seat_status::ChangeSeatStatus,
            PhoenixV1Instruction::RequestSeatAuthorized => request_seat_authorized::RequestSeatAuthorized,
            PhoenixV1Instruction::EvictSeat => evict_seat::EvictSeat,
            PhoenixV1Instruction::ForceCancelOrders => force_cancel_orders::ForceCancelOrders,
            PhoenixV1Instruction::CollectFees => collect_fees::CollectFees,
            PhoenixV1Instruction::ChangeFeeRecipient => change_fee_recipient::ChangeFeeRecipient,
        )
    }
}
