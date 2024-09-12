use crate::{
    collection::InstructionDecoderCollection,
    error::CarbonResult,
    instruction::NestedInstruction,
    processor::Processor,
    schema::{ParsedInstruction, TransactionSchema},
};
use async_trait::async_trait;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

#[derive(Debug, Clone)]
pub struct TransactionMetadata {
    pub slot: u64,
    pub signature: Signature,
    pub fee_payer: Pubkey,
}

pub struct TransactionPipe<T: InstructionDecoderCollection> {
    schema: TransactionSchema<T>,
    processor: Box<dyn Processor<InputType = ParsedTransaction<T>> + Send + Sync>,
}

#[derive(Debug)]
pub struct ParsedTransaction<I: InstructionDecoderCollection> {
    pub metadata: TransactionMetadata,
    pub instructions: Vec<ParsedInstruction<I>>,
}

impl<T: InstructionDecoderCollection> TransactionPipe<T> {
    pub fn new(
        schema: TransactionSchema<T>,
        processor: Box<dyn Processor<InputType = ParsedTransaction<T>> + Send + Sync>,
    ) -> Self {
        let new = Self { schema, processor };
        println!("schema: {:?}", new.schema);
        new
    }

    fn parse_instructions(&self, instructions: &[NestedInstruction]) -> Vec<ParsedInstruction<T>> {
        instructions
            .iter()
            .filter_map(|nested_instr| {
                T::parse_instruction(&nested_instr.instruction).map(|decoded_instruction| {
                    ParsedInstruction {
                        program_id: nested_instr.instruction.program_id,
                        instruction: decoded_instruction,
                        inner_instructions: self
                            .parse_instructions(&nested_instr.inner_instructions),
                    }
                })
            })
            .collect()
    }

    fn matches_schema(&self, transaction: &ParsedTransaction<T>) -> bool {
        self.schema.matches(&transaction.instructions)
    }
}

#[async_trait]
pub trait TransactionPipes {
    async fn run(
        &self,
        transaction_metadata: TransactionMetadata,
        instructions: Vec<NestedInstruction>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl<T: InstructionDecoderCollection + Sync> TransactionPipes for TransactionPipe<T> {
    async fn run(
        &self,
        transaction_metadata: TransactionMetadata,
        instructions: Vec<NestedInstruction>,
    ) -> CarbonResult<()> {
        let parsed_transaction = ParsedTransaction {
            metadata: transaction_metadata,
            instructions: self.parse_instructions(&instructions),
        };

        if self.matches_schema(&parsed_transaction) {
            self.processor.process(parsed_transaction).await?;
        }

        Ok(())
    }
}
