

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

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::Withdraw => withdraw::Withdraw,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
        )
    }
}
