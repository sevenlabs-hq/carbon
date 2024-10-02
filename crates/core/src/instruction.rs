use async_trait::async_trait;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

use crate::{
    error::CarbonResult,
    processor::{self, Processor},
    transaction::TransactionMetadata,
};

#[derive(Debug, Clone)]
pub struct InstructionMetadata {
    pub transaction_metadata: TransactionMetadata,
    pub stack_height: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecodedInstruction<T> {
    pub program_id: Pubkey,
    pub data: T,
}

pub trait InstructionDecoder {
    type InstructionType;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>>;
}

pub struct InstructionPipe<T: Send> {
    pub decoder: Box<dyn InstructionDecoder<InstructionType = T> + Send + Sync>,
    pub processor: Box<
        dyn Processor<
                InputType = (
                    InstructionMetadata,
                    DecodedInstruction<T>,
                    Vec<NestedInstruction>,
                ),
            > + Send
            + Sync,
    >,
}

#[async_trait]
pub trait InstructionPipes {
    async fn run(&self, nested_instruction: &NestedInstruction) -> CarbonResult<()>;
}

#[async_trait]
impl<T: Send> InstructionPipes for InstructionPipe<T> {
    async fn run(&self, nested_instruction: &NestedInstruction) -> CarbonResult<()> {
        if let Some(decoded_instruction) = self
            .decoder
            .decode_instruction(nested_instruction.instruction.clone())
        {
            self.processor
                .process((
                    nested_instruction.metadata.clone(),
                    decoded_instruction,
                    nested_instruction.inner_instructions.clone(),
                ))
                .await?;
        }

        for nested_inner_instruction in nested_instruction.inner_instructions.iter() {
            self.run(nested_inner_instruction).await?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct NestedInstruction {
    pub metadata: InstructionMetadata,
    pub instruction: solana_sdk::instruction::Instruction,
    pub inner_instructions: Vec<NestedInstruction>,
}
