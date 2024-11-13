use super::JupiterLimitOrder2Decoder;
pub mod cancel_order;
pub mod cancel_order_event;
pub mod create_order_event;
pub mod flash_fill_order;
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
pub enum JupiterLimitOrder2Instruction {
    UpdateFee(update_fee::UpdateFee),
    WithdrawFee(withdraw_fee::WithdrawFee),
    InitializeOrder(initialize_order::InitializeOrder),
    CancelOrder(cancel_order::CancelOrder),
    PreFlashFillOrder(pre_flash_fill_order::PreFlashFillOrder),
    FlashFillOrder(flash_fill_order::FlashFillOrder),
    TradeEvent(trade_event::TradeEvent),
    CancelOrderEvent(cancel_order_event::CancelOrderEvent),
    CreateOrderEvent(create_order_event::CreateOrderEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterLimitOrder2Decoder {
    type InstructionType = JupiterLimitOrder2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterLimitOrder2Instruction::UpdateFee => update_fee::UpdateFee,
            JupiterLimitOrder2Instruction::WithdrawFee => withdraw_fee::WithdrawFee,
            JupiterLimitOrder2Instruction::InitializeOrder => initialize_order::InitializeOrder,
            JupiterLimitOrder2Instruction::CancelOrder => cancel_order::CancelOrder,
            JupiterLimitOrder2Instruction::PreFlashFillOrder => pre_flash_fill_order::PreFlashFillOrder,
            JupiterLimitOrder2Instruction::FlashFillOrder => flash_fill_order::FlashFillOrder,
            JupiterLimitOrder2Instruction::TradeEvent => trade_event::TradeEvent,
            JupiterLimitOrder2Instruction::CancelOrderEvent => cancel_order_event::CancelOrderEvent,
            JupiterLimitOrder2Instruction::CreateOrderEvent => create_order_event::CreateOrderEvent,
        )
    }
}
