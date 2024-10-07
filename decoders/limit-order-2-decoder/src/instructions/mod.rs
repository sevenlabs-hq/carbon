
use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;


use super::LimitOrder2Decoder;
pub mod update_fee;
pub mod withdraw_fee;
pub mod initialize_order;
pub mod cancel_order;
pub mod pre_flash_fill_order;
pub mod flash_fill_order;
pub mod trade_event;
pub mod cancel_order_event;
pub mod create_order_event;

#[derive(carbon_proc_macros::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum LimitOrder2Instruction {
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

impl InstructionDecoder for LimitOrder2Decoder {
    type InstructionType = LimitOrder2Instruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) = update_fee::UpdateFee::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::UpdateFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = withdraw_fee::WithdrawFee::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::WithdrawFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = initialize_order::InitializeOrder::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::InitializeOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = cancel_order::CancelOrder::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::CancelOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = pre_flash_fill_order::PreFlashFillOrder::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::PreFlashFillOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = flash_fill_order::FlashFillOrder::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::FlashFillOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = trade_event::TradeEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::TradeEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = cancel_order_event::CancelOrderEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::CancelOrderEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_order_event::CreateOrderEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrder2Instruction::CreateOrderEvent(decoded_instruction),
            });
        }

        None
    }
}