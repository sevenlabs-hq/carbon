
use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;


use super::PumpDecoder;
pub mod initialize;
pub mod set_params;
pub mod create;
pub mod buy;
pub mod sell;
pub mod withdraw;
pub mod create_event;
pub mod trade_event;
pub mod complete_event;
pub mod set_params_event;

#[derive(carbon_proc_macros::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum PumpInstruction {
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

impl InstructionDecoder for PumpDecoder {
    type InstructionType = PumpInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) = initialize::Initialize::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::Initialize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_params::SetParams::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::SetParams(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create::Create::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::Create(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = buy::Buy::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::Buy(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = sell::Sell::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::Sell(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = withdraw::Withdraw::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::Withdraw(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_event::CreateEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::CreateEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = trade_event::TradeEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::TradeEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = complete_event::CompleteEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::CompleteEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_params_event::SetParamsEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: PumpInstruction::SetParamsEvent(decoded_instruction),
            });
        }

        None
    }
}