use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;

use crate::{error::CarbonResult, processor::Processor, transaction::TransactionMetadata};

#[derive(Debug, Clone)]
pub struct InstructionMetadata {
    pub transaction_metadata: TransactionMetadata,
    pub stack_height: u32,
}

#[derive(Debug, Clone)]
pub struct DecodedInstruction<T> {
    pub program_id: Pubkey,
    pub data: T,
}

pub trait InstructionDecoder {
    type InstructionType;

    fn decode(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>>;
}

pub struct InstructionPipe<T: Send> {
    pub decoder: Box<dyn InstructionDecoder<InstructionType = T> + Send + Sync + 'static>,
    pub processor: Box<
        dyn Processor<InputType = (InstructionMetadata, DecodedInstruction<T>)>
            + Send
            + Sync
            + 'static,
    >,
}

#[async_trait]
pub trait InstructionPipes: Send + Sync {
    async fn run(
        &self,
        instruction_with_metadata: (InstructionMetadata, solana_sdk::instruction::Instruction),
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<T: Send + 'static> InstructionPipes for InstructionPipe<T> {
    async fn run(
        &self,
        instruction_with_metadata: (InstructionMetadata, solana_sdk::instruction::Instruction),
    ) -> CarbonResult<()> {
        if let Some(decoded_instruction) = self.decoder.decode(&instruction_with_metadata.1) {
            self.processor
                .process((instruction_with_metadata.0, decoded_instruction))
                .await?;
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
