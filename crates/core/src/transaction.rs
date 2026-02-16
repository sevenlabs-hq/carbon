use crate::filter::Filter;
use solana_program::hash::Hash;

use {
    crate::{
        collection::InstructionDecoderCollection,
        error::CarbonResult,
        instruction::{InstructionMetadata, ParsedInstruction},
        processor::Processor,
    },
    async_trait::async_trait,
    core::convert::TryFrom,
    solana_instruction::Instruction,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::sync::Arc,
};
#[derive(Debug, Clone, Default)]
pub struct TransactionMetadata {
    pub slot: u64,
    pub signature: Signature,
    pub fee_payer: Pubkey,
    pub meta: solana_transaction_status::TransactionStatusMeta,
    pub message: solana_message::VersionedMessage,
    pub index: Option<u64>,
    pub block_time: Option<i64>,
    pub block_hash: Option<Hash>,
}

impl TryFrom<crate::datasource::TransactionUpdate> for TransactionMetadata {
    type Error = crate::error::Error;

    fn try_from(value: crate::datasource::TransactionUpdate) -> Result<Self, Self::Error> {
        log::trace!("try_from(transaction_update: {value:?})");
        let accounts = value.transaction.message.static_account_keys();

        Ok(TransactionMetadata {
            slot: value.slot,
            signature: value.signature,
            fee_payer: *accounts
                .first()
                .ok_or(crate::error::Error::MissingFeePayer)?,
            meta: value.meta.clone(),
            message: value.transaction.message.clone(),
            index: value.index,
            block_time: value.block_time,
            block_hash: value.block_hash,
        })
    }
}

#[derive(Debug)]
pub struct TransactionProcessorInputType<'a, T> {
    pub metadata: &'a Arc<TransactionMetadata>,
    pub instructions: &'a Vec<(InstructionMetadata, T)>,
}

pub struct TransactionPipe<T: InstructionDecoderCollection, P> {
    processor: P,
    filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
    _phantom: std::marker::PhantomData<T>,
}

pub struct ParsedTransaction<I: InstructionDecoderCollection> {
    pub metadata: TransactionMetadata,
    pub instructions: Vec<ParsedInstruction<I>>,
}

impl<T: InstructionDecoderCollection, P> TransactionPipe<T, P> {
    pub fn new(processor: P, filters: Vec<Box<dyn Filter + Send + Sync + 'static>>) -> Self {
        log::trace!(
            "TransactionPipe::new(processor: {:?})",
            stringify!(processor)
        );
        Self {
            processor,
            filters,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub fn parse_instructions_flat<T: InstructionDecoderCollection>(
    instructions: &[(InstructionMetadata, Instruction)],
) -> Vec<(InstructionMetadata, T)> {
    log::trace!(
        "parse_instructions_flat(instructions: len={})",
        instructions.len()
    );

    instructions
        .iter()
        .filter_map(|(metadata, instruction)| {
            T::parse_instruction(instruction).map(|parsed| (metadata.clone(), parsed))
        })
        .collect()
}

#[async_trait]
pub trait TransactionPipes<'a>: Send + Sync {
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[(InstructionMetadata, Instruction)],
    ) -> CarbonResult<()>;

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
}

#[async_trait]
impl<T, P> TransactionPipes<'_> for TransactionPipe<T, P>
where
    T: InstructionDecoderCollection + Sync + 'static,
    P: for<'a> Processor<TransactionProcessorInputType<'a, T>> + Send + Sync + 'static,
{
    async fn run(
        &mut self,
        transaction_metadata: Arc<TransactionMetadata>,
        instructions: &[(InstructionMetadata, Instruction)],
    ) -> CarbonResult<()> {
        log::trace!(
            "TransactionPipe::run(instructions: len={})",
            instructions.len()
        );

        let parsed_instructions = parse_instructions_flat::<T>(instructions);

        let data = TransactionProcessorInputType {
            metadata: &transaction_metadata,
            instructions: &parsed_instructions,
        };

        self.processor.process(&data).await?;

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::InstructionsWithMetadata;
    use solana_instruction::{AccountMeta, Instruction};
    use solana_pubkey::Pubkey;
    use solana_transaction_status::TransactionStatusMeta;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
    struct TestDecoder;
    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
    enum TestInstructionType {
        Parsed,
    }

    impl InstructionDecoderCollection for TestDecoder {
        type InstructionType = TestInstructionType;

        fn parse_instruction(instruction: &Instruction) -> Option<Self> {
            if instruction.data.first() == Some(&1u8) {
                Some(TestDecoder)
            } else {
                None
            }
        }

        fn get_type(&self) -> Self::InstructionType {
            TestInstructionType::Parsed
        }
    }

    fn create_flat_instruction(
        data_byte: u8,
        stack_height: u32,
        index: u32,
    ) -> (InstructionMetadata, Instruction) {
        let metadata = InstructionMetadata {
            transaction_metadata: Arc::new(TransactionMetadata {
                meta: TransactionStatusMeta::default(),
                ..Default::default()
            }),
            stack_height,
            index,
            absolute_path: vec![index as u8],
        };
        let instruction = Instruction {
            program_id: Pubkey::new_unique(),
            accounts: vec![AccountMeta::new(Pubkey::new_unique(), false)],
            data: vec![data_byte],
        };
        (metadata, instruction)
    }

    #[test]
    fn test_parse_instructions_flat_empty() {
        let instructions: InstructionsWithMetadata = vec![];
        let parsed = parse_instructions_flat::<TestDecoder>(&instructions);
        assert!(parsed.is_empty());
    }

    #[test]
    fn test_parse_instructions_flat_all_match() {
        let instructions: InstructionsWithMetadata = vec![
            create_flat_instruction(1, 1, 0),
            create_flat_instruction(1, 1, 1),
            create_flat_instruction(1, 2, 2),
        ];
        let parsed = parse_instructions_flat::<TestDecoder>(&instructions);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].0.stack_height, 1);
        assert_eq!(parsed[0].0.index, 0);
        assert_eq!(parsed[1].0.stack_height, 1);
        assert_eq!(parsed[1].0.index, 1);
        assert_eq!(parsed[2].0.stack_height, 2);
        assert_eq!(parsed[2].0.index, 2);
    }

    #[test]
    fn test_parse_instructions_flat_none_match() {
        let instructions: InstructionsWithMetadata = vec![
            create_flat_instruction(0, 1, 0),
            create_flat_instruction(2, 1, 1),
        ];
        let parsed = parse_instructions_flat::<TestDecoder>(&instructions);
        assert!(parsed.is_empty());
    }

    #[test]
    fn test_parse_instructions_flat_mixed() {
        let instructions: InstructionsWithMetadata = vec![
            create_flat_instruction(0, 1, 0),
            create_flat_instruction(1, 1, 1),
            create_flat_instruction(2, 2, 2),
            create_flat_instruction(1, 2, 3),
        ];
        let parsed = parse_instructions_flat::<TestDecoder>(&instructions);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].0.index, 1);
        assert_eq!(parsed[0].0.stack_height, 1);
        assert_eq!(parsed[1].0.index, 3);
        assert_eq!(parsed[1].0.stack_height, 2);
    }

    #[test]
    fn test_parse_instructions_flat_preserves_metadata() {
        let (metadata, instruction) = create_flat_instruction(1, 3, 42);
        let instructions = vec![(metadata.clone(), instruction)];
        let parsed = parse_instructions_flat::<TestDecoder>(&instructions);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].0.stack_height, metadata.stack_height);
        assert_eq!(parsed[0].0.index, metadata.index);
        assert_eq!(parsed[0].0.absolute_path, metadata.absolute_path);
    }
}
