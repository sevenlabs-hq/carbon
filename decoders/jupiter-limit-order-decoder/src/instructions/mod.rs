use super::JupiterLimitOrderDecoder;
pub mod cancel_expired_order;
pub mod cancel_order;
pub mod cancel_order_event;
pub mod create_order_event;
pub mod fill_order;
pub mod flash_fill_order;
pub mod init_fee;
pub mod initialize_order;
pub mod pre_flash_fill_order;
pub mod trade_event;
pub mod update_fee;
pub mod withdraw_fee;

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
pub enum JupiterLimitOrderInstruction {
    InitializeOrder(initialize_order::InitializeOrder),
    FillOrder(fill_order::FillOrder),
    PreFlashFillOrder(pre_flash_fill_order::PreFlashFillOrder),
    FlashFillOrder(flash_fill_order::FlashFillOrder),
    CancelOrder(cancel_order::CancelOrder),
    CancelExpiredOrder(cancel_expired_order::CancelExpiredOrder),
    WithdrawFee(withdraw_fee::WithdrawFee),
    InitFee(init_fee::InitFee),
    UpdateFee(update_fee::UpdateFee),
    TradeEvent(trade_event::TradeEvent),
    CancelOrderEvent(cancel_order_event::CancelOrderEvent),
    CreateOrderEvent(create_order_event::CreateOrderEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterLimitOrderDecoder {
    type InstructionType = JupiterLimitOrderInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterLimitOrderInstruction::InitializeOrder => initialize_order::InitializeOrder,
            JupiterLimitOrderInstruction::FillOrder => fill_order::FillOrder,
            JupiterLimitOrderInstruction::PreFlashFillOrder => pre_flash_fill_order::PreFlashFillOrder,
            JupiterLimitOrderInstruction::FlashFillOrder => flash_fill_order::FlashFillOrder,
            JupiterLimitOrderInstruction::CancelOrder => cancel_order::CancelOrder,
            JupiterLimitOrderInstruction::CancelExpiredOrder => cancel_expired_order::CancelExpiredOrder,
            JupiterLimitOrderInstruction::WithdrawFee => withdraw_fee::WithdrawFee,
            JupiterLimitOrderInstruction::InitFee => init_fee::InitFee,
            JupiterLimitOrderInstruction::UpdateFee => update_fee::UpdateFee,
            JupiterLimitOrderInstruction::TradeEvent => trade_event::TradeEvent,
            JupiterLimitOrderInstruction::CancelOrderEvent => cancel_order_event::CancelOrderEvent,
            JupiterLimitOrderInstruction::CreateOrderEvent => create_order_event::CreateOrderEvent,
        )
    }
}
