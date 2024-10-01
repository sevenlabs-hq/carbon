use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use crate::InvestmentWatchesProgramDecoder;
pub mod create_watch;
pub mod reclaim_watch;
pub mod redeem_microshare;
pub mod redeem_watch;
pub mod register_microshare;

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
pub enum InvestmentWatchesProgramInstruction {
    CreateWatch(create_watch::CreateWatch),
    ReclaimWatch(reclaim_watch::ReclaimWatch),
    RedeemMicroshare(redeem_microshare::RedeemMicroshare),
    RedeemWatch(redeem_watch::RedeemWatch),
    RegisterMicroshare(register_microshare::RegisterMicroshare),
}

impl InstructionDecoder for InvestmentWatchesProgramDecoder {
    type InstructionType = InvestmentWatchesProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            create_watch::CreateWatch::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: InvestmentWatchesProgramInstruction::CreateWatch(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            reclaim_watch::ReclaimWatch::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: InvestmentWatchesProgramInstruction::ReclaimWatch(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            redeem_microshare::RedeemMicroshare::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: InvestmentWatchesProgramInstruction::RedeemMicroshare(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            redeem_watch::RedeemWatch::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: InvestmentWatchesProgramInstruction::RedeemWatch(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            register_microshare::RegisterMicroshare::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: InvestmentWatchesProgramInstruction::RegisterMicroshare(decoded_instruction),
            });
        }

        None
    }
}
