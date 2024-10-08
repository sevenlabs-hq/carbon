use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use super::LimitOrderDecoder;
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
    carbon_proc_macros::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum LimitOrderInstruction {
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

impl InstructionDecoder for LimitOrderDecoder {
    type InstructionType = LimitOrderInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            initialize_order::InitializeOrder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::InitializeOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            fill_order::FillOrder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::FillOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            pre_flash_fill_order::PreFlashFillOrder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::PreFlashFillOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            flash_fill_order::FlashFillOrder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::FlashFillOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            cancel_order::CancelOrder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::CancelOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            cancel_expired_order::CancelExpiredOrder::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::CancelExpiredOrder(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_fee::WithdrawFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::WithdrawFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            init_fee::InitFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::InitFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_fee::UpdateFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::UpdateFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            trade_event::TradeEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::TradeEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            cancel_order_event::CancelOrderEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::CancelOrderEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_order_event::CreateOrderEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: LimitOrderInstruction::CreateOrderEvent(decoded_instruction),
            });
        }

        None
    }
}
