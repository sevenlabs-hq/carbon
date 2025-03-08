use crate::PROGRAM_ID;

use super::PumpfunDecoder;
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
pub enum PumpfunInstruction {
    Buy(buy::Buy),
    Create(create::Create),
    Initialize(initialize::Initialize),
    Sell(sell::Sell),
    SetParams(set_params::SetParams),
    Withdraw(withdraw::Withdraw),
    CompleteEvent(complete_event::CompleteEvent),
    CreateEvent(create_event::CreateEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
    TradeEvent(trade_event::TradeEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::Withdraw => withdraw::Withdraw,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
        )
    }
}
