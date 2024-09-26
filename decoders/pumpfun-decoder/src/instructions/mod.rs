use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use crate::PumpfunDecoder;

pub mod buy;
pub mod complete_event;
pub mod create;
pub mod create_event;
pub mod initialize;
pub mod sell;
pub mod set_params;
pub mod set_params_event;
pub mod trade_event;
pub mod withdraw;

#[derive(Debug, carbon_proc_macros::InstructionType)]
pub enum PumpFunInstruction {
    Initialize(initialize::Initialize),
    SetParams(set_params::SetParams),
    Create(create::Create),
    Buy(buy::Buy),
    Sell(sell::Sell),
    Withdraw(withdraw::Withdraw),
    CreateEvent(create_event::CreateEvent),
    TradeEvent(trade_event::TradeEvent),
    CompleteEvent(complete_event::CompleteEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
}

impl InstructionDecoder for PumpfunDecoder {
    type InstructionType = PumpFunInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            initialize::Initialize::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::Initialize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_params::SetParams::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::SetParams(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create::Create::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::Create(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = buy::Buy::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::Buy(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = sell::Sell::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::Sell(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw::Withdraw::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::Withdraw(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_event::CreateEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::CreateEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            trade_event::TradeEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::TradeEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            complete_event::CompleteEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::CompleteEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_params_event::SetParamsEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpFunInstruction::SetParamsEvent(decoded_instruction),
            });
        }
        None
    }
}
