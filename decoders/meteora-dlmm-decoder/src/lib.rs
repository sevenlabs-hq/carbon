use std::str::FromStr;

use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionDecoder, InstructionMetadata},
    processor::Processor,
};
use instructions::MeteoraInstruction;
use serde::{self, Deserialize};
use solana_sdk::pubkey::Pubkey;

pub mod instructions;

pub const METEORA_DLMM_PROGRAM: &'static str = "mFivYY5xPoh3rDCxbdwzkgN1Rv2kC9s9kEpHHsnUWtf";

pub struct MeteoraInstructionDecoder;
impl InstructionDecoder for MeteoraInstructionDecoder {
    type InstructionType = MeteoraInstruction;

    fn decode(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if instruction.program_id == Pubkey::from_str(METEORA_DLMM_PROGRAM).unwrap() {
            Some(DecodedInstruction {
                program_id: instruction.program_id,
                data: MeteoraInstruction::unpack(&instruction.data).ok()?,
            })
        } else {
            None
        }
    }
}

pub struct MeteoraInstructionProcessor;
#[async_trait]
impl Processor for MeteoraInstructionProcessor {
    type InputType = (InstructionMetadata, DecodedInstruction<MeteoraInstruction>);

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MeteoraOutput {}

pub struct MeteoraTransactionProcessor;
#[async_trait]
impl Processor for MeteoraTransactionProcessor {
    type InputType = MeteoraOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        println!("Matched meteora");

        Ok(())
    }
}
