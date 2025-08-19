use {super::PhoenixDecoder, crate::PROGRAM_ID};
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
pub enum PhoenixInstruction {
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

impl carbon_core::instruction::InstructionDecoder<'_> for PhoenixDecoder {
    type InstructionType = PhoenixInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            PhoenixInstruction::Swap => swap::Swap,
            PhoenixInstruction::SwapWithFreeFunds => swap_with_free_funds::SwapWithFreeFunds,
            PhoenixInstruction::PlaceLimitOrder => place_limit_order::PlaceLimitOrder,
            PhoenixInstruction::PlaceLimitOrderWithFreeFunds => place_limit_order_with_free_funds::PlaceLimitOrderWithFreeFunds,
            PhoenixInstruction::ReduceOrder => reduce_order::ReduceOrder,
            PhoenixInstruction::ReduceOrderWithFreeFunds => reduce_order_with_free_funds::ReduceOrderWithFreeFunds,
            PhoenixInstruction::CancelAllOrders => cancel_all_orders::CancelAllOrders,
            PhoenixInstruction::CancelAllOrdersWithFreeFunds => cancel_all_orders_with_free_funds::CancelAllOrdersWithFreeFunds,
            PhoenixInstruction::CancelUpTo => cancel_up_to::CancelUpTo,
            PhoenixInstruction::CancelUpToWithFreeFunds => cancel_up_to_with_free_funds::CancelUpToWithFreeFunds,
            PhoenixInstruction::CancelMultipleOrdersById => cancel_multiple_orders_by_id::CancelMultipleOrdersById,
            PhoenixInstruction::CancelMultipleOrdersByIdWithFreeFunds => cancel_multiple_orders_by_id_with_free_funds::CancelMultipleOrdersByIdWithFreeFunds,
            PhoenixInstruction::WithdrawFunds => withdraw_funds::WithdrawFunds,
            PhoenixInstruction::DepositFunds => deposit_funds::DepositFunds,
            PhoenixInstruction::RequestSeat => request_seat::RequestSeat,
            PhoenixInstruction::Log => log::Log,
            PhoenixInstruction::PlaceMultiplePostOnlyOrders => place_multiple_post_only_orders::PlaceMultiplePostOnlyOrders,
            PhoenixInstruction::PlaceMultiplePostOnlyOrdersWithFreeFunds => place_multiple_post_only_orders_with_free_funds::PlaceMultiplePostOnlyOrdersWithFreeFunds,
            PhoenixInstruction::InitializeMarket => initialize_market::InitializeMarket,
            PhoenixInstruction::ClaimAuthority => claim_authority::ClaimAuthority,
            PhoenixInstruction::NameSuccessor => name_successor::NameSuccessor,
            PhoenixInstruction::ChangeMarketStatus => change_market_status::ChangeMarketStatus,
            PhoenixInstruction::ChangeSeatStatus => change_seat_status::ChangeSeatStatus,
            PhoenixInstruction::RequestSeatAuthorized => request_seat_authorized::RequestSeatAuthorized,
            PhoenixInstruction::EvictSeat => evict_seat::EvictSeat,
            PhoenixInstruction::ForceCancelOrders => force_cancel_orders::ForceCancelOrders,
            PhoenixInstruction::CollectFees => collect_fees::CollectFees,
            PhoenixInstruction::ChangeFeeRecipient => change_fee_recipient::ChangeFeeRecipient,
        )
    }
}
