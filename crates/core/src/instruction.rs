//! Provides structures and traits for decoding and processing instructions
//! within transactions.
//!
//! The module includes the following main components:
//! - **`InstructionMetadata`**: Metadata associated with an instruction,
//!   capturing transaction context.
//! - **`DecodedInstruction`**: Represents an instruction that has been decoded,
//!   with associated program ID, data, and accounts.
//! - **`InstructionDecoder`**: A trait for decoding instructions into specific
//!   types.
//! - **`InstructionPipe`**: A structure that processes instructions using a
//!   decoder and a processor.
//! - **`InstructionPipes`**: An async trait for processing instructions within
//!   nested contexts.
//! - **`NestedInstruction`**: Represents instructions with potential nested
//!   inner instructions, allowing for recursive processing.
//!
//! These components enable the `carbon-core` framework to handle Solana
//! transaction instructions efficiently, decoding them into structured types
//! and facilitating hierarchical processing.

use {
    crate::{
        deserialize::CarbonDeserialize, error::CarbonResult, filter::Filter,
        metrics::MetricsCollection, processor::Processor, transaction::TransactionMetadata,
    },
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    solana_instruction::AccountMeta,
    solana_pubkey::Pubkey,
    std::{
        ops::{Deref, DerefMut},
        sync::Arc,
    },
};

/// Metadata associated with a specific instruction, including transaction-level
/// details.
///
/// `InstructionMetadata` is utilized within the pipeline to associate each
/// instruction with the broader context of its transaction, as well as its
/// position within the instruction stack.
///
/// # Fields
///
/// - `transaction_metadata`: Metadata providing details of the entire
///   transaction.
/// - `stack_height`: Represents the instruction's depth within the stack, where
///   1 is the root level.
/// - `index`: The index of the instruction in the transaction. The index is
///   relative within stack height and is 1-based. Note that the inner
///   instruction indexes are grouped into one vector, so different inner
///   instructions that have different stack heights may have continuous
///   indexes.

#[derive(Debug, Clone)]
pub struct InstructionMetadata {
    pub transaction_metadata: Arc<TransactionMetadata>,
    pub stack_height: u32,
    pub index: u32,
    pub absolute_path: Vec<u8>,
}

#[derive(Debug)]
enum LogType {
    Start(usize), // stack_height
    Data,
    CU,
    Finish,
}

impl InstructionMetadata {
    /// Decodes the log events `T` thrown by this instruction.
    ///
    /// # Parameters
    ///
    /// - `T`: The event type to decode the instruction's logs into.
    ///
    /// # Returns
    ///
    /// All successfull events of the type `T` decoded from the logs of the instruction.
    pub fn decode_log_events<T: CarbonDeserialize>(&self) -> Vec<T> {
        self.extract_event_log_data()
            .into_iter()
            .filter(|log| log.len() >= 8)
            .filter_map(|log| <T as CarbonDeserialize>::deserialize(&log))
            .collect()
    }

    /// Extracts the `data` from log messages associated with this instruction.
    ///
    /// This method filters the transaction's log messages to return only those
    /// that correspond to the current instruction, based on its stack height and
    /// absolute path within the instruction stack.
    ///
    /// Returns `Vec<Vec<u8>>` containing the `data` bytes (base64 encoded) from log messages.
    fn extract_event_log_data(&self) -> Vec<Vec<u8>> {
        let logs = match &self.transaction_metadata.meta.log_messages {
            Some(logs) => logs,
            None => return Vec::new(),
        };

        let mut extracted_logs = Vec::new();
        let mut current_stack_height = 0usize;
        let mut last_stack_height = 0usize;

        let mut position_at_level: std::collections::HashMap<usize, u8> =
            std::collections::HashMap::new();

        for log in logs {
            let parsed_log = self.parse_log(log);

            match parsed_log {
                LogType::Start(stack_height) => {
                    current_stack_height = stack_height;

                    let current_pos = if stack_height > last_stack_height {
                        0
                    } else {
                        position_at_level
                            .get(&stack_height)
                            .map(|&pos| pos + 1)
                            .unwrap_or(0)
                    };

                    position_at_level.insert(stack_height, current_pos);
                    last_stack_height = stack_height;
                }
                LogType::Finish => {
                    current_stack_height = current_stack_height.saturating_sub(1);
                }
                _ => {}
            }

            let current_path: Vec<u8> = (1..=current_stack_height)
                .map(|level| position_at_level.get(&level).copied().unwrap_or(0))
                .collect();

            if current_path == self.absolute_path && matches!(parsed_log, LogType::Data) {
                if let Some(data) = log.split_whitespace().last() {
                    if let Ok(buf) =
                        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data)
                    {
                        extracted_logs.push(buf);
                    }
                }
            }
        }

        extracted_logs
    }

    /// Parses a log line to determine its type
    fn parse_log(&self, log: &str) -> LogType {
        if log.starts_with("Program ") && log.contains(" invoke [") {
            let parts: Vec<&str> = log.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == "Program" && parts[2] == "invoke" {
                let level_str = parts[3].trim_start_matches('[').trim_end_matches(']');
                if let Ok(level) = level_str.parse::<usize>() {
                    return LogType::Start(level);
                }
            }
        } else if log.starts_with("Program ")
            && (log.ends_with(" success") || log.contains(" failed"))
        {
            let parts: Vec<&str> = log.split_whitespace().collect();
            if parts.len() >= 3 && parts[0] == "Program" {
                return LogType::Finish;
            }
        } else if log.contains("consumed") && log.contains("compute units") {
            return LogType::CU;
        }

        LogType::Data
    }
}

pub type InstructionsWithMetadata = Vec<(InstructionMetadata, solana_instruction::Instruction)>;

/// A decoded instruction containing program ID, data, and associated accounts.
///
/// The `DecodedInstruction` struct represents the outcome of decoding a raw
/// instruction, encapsulating its program ID, parsed data, and the accounts
/// involved.
///
/// # Type Parameters
///
/// - `T`: The type representing the decoded data for the instruction.
///
/// # Fields
///
/// - `program_id`: The program ID that owns the instruction.
/// - `data`: The decoded data payload for the instruction, of type `T`.
/// - `accounts`: A vector of `AccountMeta`, representing the accounts involved
///   in the instruction.

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodedInstruction<T> {
    pub program_id: Pubkey,
    pub data: T,
    pub accounts: Vec<AccountMeta>,
}

/// A trait for decoding Solana instructions into a structured type.
///
/// Implement the `InstructionDecoder` trait for types that can decode raw
/// instructions into a more meaningful structure, providing
/// application-specific logic.
///
/// # Type Parameters
///
/// - `InstructionType`: The type into which the instruction data will be
///   decoded.
///
/// # Required Methods
///
/// - `decode_instruction`: Decodes a raw Solana `Instruction` into a
///   `DecodedInstruction`.
pub trait InstructionDecoder<'a> {
    type InstructionType;

    fn decode_instruction(
        &self,
        instruction: &'a solana_instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>>;
}

/// The input type for the instruction processor.
///
/// - `T`: The instruction type
pub type InstructionProcessorInputType<T> = (
    InstructionMetadata,
    DecodedInstruction<T>,
    NestedInstructions,
    solana_instruction::Instruction,
);

/// A processing pipeline for instructions, using a decoder and processor.
///
/// The `InstructionPipe` structure enables the processing of decoded
/// instructions, pairing an `InstructionDecoder` with a `Processor`. It
/// supports generic instruction types.
///
/// # Type Parameters
///
/// - `T`: The type representing the decoded instruction data.
///
/// # Fields
///
/// - `decoder`: The decoder used for parsing instructions.
/// - `processor`: The processor that handles decoded instructions.
/// - `filters`: A collection of filters that determine which instruction
///   updates should be processed. Each filter in this collection is applied to
///   incoming instruction updates, and only updates that pass all filters
///   (return `true`) will be processed. If this collection is empty, all
///   updates are processed.
pub struct InstructionPipe<T: Send> {
    pub decoder:
        Box<dyn for<'a> InstructionDecoder<'a, InstructionType = T> + Send + Sync + 'static>,
    pub processor:
        Box<dyn Processor<InputType = InstructionProcessorInputType<T>> + Send + Sync + 'static>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

/// An async trait for processing instructions within nested contexts.
///
/// The `InstructionPipes` trait allows for recursive processing of instructions
/// that may contain nested instructions. This enables complex, hierarchical
/// instruction handling for transactions.
///
/// # Required Methods
///
/// - `run`: Processes a `NestedInstruction`, recursively processing any inner
///   instructions.
/// - `filters`: Returns a reference to the filters associated with this pipe,
///   which are used by the pipeline to determine which instruction updates
///   should be processed.
#[async_trait]
pub trait InstructionPipes<'a>: Send + Sync {
    async fn run(
        &mut self,
        nested_instruction: &NestedInstruction,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;
    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>>;
}

#[async_trait]
impl<T: Send + 'static> InstructionPipes<'_> for InstructionPipe<T> {
    async fn run(
        &mut self,
        nested_instruction: &NestedInstruction,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!(
            "InstructionPipe::run(nested_instruction: {:?}, metrics)",
            nested_instruction,
        );

        if let Some(decoded_instruction) = self
            .decoder
            .decode_instruction(&nested_instruction.instruction)
        {
            self.processor
                .process(
                    (
                        nested_instruction.metadata.clone(),
                        decoded_instruction,
                        nested_instruction.inner_instructions.clone(),
                        nested_instruction.instruction.clone(),
                    ),
                    metrics.clone(),
                )
                .await?;
        }

        for nested_inner_instruction in nested_instruction.inner_instructions.iter() {
            self.run(nested_inner_instruction, metrics.clone()).await?;
        }

        Ok(())
    }

    fn filters(&self) -> &Vec<Box<dyn Filter + Send + Sync + 'static>> {
        &self.filters
    }
}

/// Represents a nested instruction with metadata, including potential inner
/// instructions.
///
/// The `NestedInstruction` struct allows for recursive instruction handling,
/// where each instruction may have associated metadata and a list of nested
/// instructions.
///
/// # Fields
///
/// - `metadata`: The metadata associated with the instruction.
/// - `instruction`: The Solana instruction being processed.
/// - `inner_instructions`: A vector of `NestedInstruction`, representing any
///   nested instructions.
#[derive(Debug, Clone)]
pub struct NestedInstruction {
    pub metadata: InstructionMetadata,
    pub instruction: solana_instruction::Instruction,
    pub inner_instructions: NestedInstructions,
}

#[derive(Debug, Default)]
pub struct NestedInstructions(pub Vec<NestedInstruction>);

impl NestedInstructions {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, nested_instruction: NestedInstruction) {
        self.0.push(nested_instruction);
    }
}

impl Deref for NestedInstructions {
    type Target = [NestedInstruction];

    fn deref(&self) -> &[NestedInstruction] {
        &self.0[..]
    }
}

impl DerefMut for NestedInstructions {
    fn deref_mut(&mut self) -> &mut [NestedInstruction] {
        &mut self.0[..]
    }
}

impl Clone for NestedInstructions {
    fn clone(&self) -> Self {
        NestedInstructions(self.0.clone())
    }
}

impl IntoIterator for NestedInstructions {
    type Item = NestedInstruction;
    type IntoIter = std::vec::IntoIter<NestedInstruction>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Nests instructions based on stack height, producing a hierarchy of
/// `NestedInstruction`.
///
/// This function organizes instructions into a nested structure, enabling
/// hierarchical transaction analysis. Instructions are nested according to
/// their stack height, forming a tree-like structure.
///
/// # Parameters
///
/// - `instructions`: A list of tuples containing `InstructionMetadata` and
///   instructions.
///
/// # Returns
///
/// A vector of `NestedInstruction`, representing the instructions organized by
/// stack depth.
impl From<InstructionsWithMetadata> for NestedInstructions {
    fn from(instructions: InstructionsWithMetadata) -> Self {
        log::trace!("from(instructions: {:?})", instructions);

        // To avoid reallocations that result in dangling pointers.
        // Therefore the number of "push"s must be calculated to set the capacity
        let estimated_capacity = instructions
            .iter()
            .filter(|(meta, _)| meta.stack_height == 1)
            .count();

        UnsafeNestedBuilder::new(estimated_capacity).build(instructions)
    }
}

// https://github.com/anza-xyz/agave/blob/master/program-runtime/src/execution_budget.rs#L7
pub const MAX_INSTRUCTION_STACK_DEPTH: usize = 5;

pub struct UnsafeNestedBuilder {
    nested_ixs: Vec<NestedInstruction>,
    level_ptrs: [Option<*mut NestedInstruction>; MAX_INSTRUCTION_STACK_DEPTH],
}

impl UnsafeNestedBuilder {
    /// ## SAFETY:
    /// Make sure `capacity` is large enough to avoid capacity expansion caused
    /// by `push`
    pub fn new(capacity: usize) -> Self {
        Self {
            nested_ixs: Vec::with_capacity(capacity),
            level_ptrs: [None; MAX_INSTRUCTION_STACK_DEPTH],
        }
    }

    pub fn build(mut self, instructions: InstructionsWithMetadata) -> NestedInstructions {
        for (metadata, instruction) in instructions {
            let stack_height = metadata.stack_height as usize;

            assert!(stack_height > 0);
            assert!(stack_height <= MAX_INSTRUCTION_STACK_DEPTH);

            for ptr in &mut self.level_ptrs[stack_height..] {
                *ptr = None;
            }

            let new_instruction = NestedInstruction {
                metadata,
                instruction,
                inner_instructions: NestedInstructions::default(),
            };

            // SAFETY:The following operation is safe.
            // because:
            // 1. All pointers come from pre-allocated Vec (no extension)
            // 2. level_ptr does not guarantee any aliasing
            // 3. Lifecycle is limited to the build() method
            unsafe {
                if stack_height == 1 {
                    self.nested_ixs.push(new_instruction);
                    let ptr = self.nested_ixs.last_mut().unwrap_unchecked() as *mut _;
                    self.level_ptrs[0] = Some(ptr);
                } else if let Some(parent_ptr) = self.level_ptrs[stack_height - 2] {
                    (*parent_ptr).inner_instructions.push(new_instruction);
                    let ptr = (*parent_ptr)
                        .inner_instructions
                        .last_mut()
                        .unwrap_unchecked() as *mut _;
                    self.level_ptrs[stack_height - 1] = Some(ptr);
                }
            }
        }

        NestedInstructions(self.nested_ixs)
    }
}

#[cfg(test)]
mod tests {

    use {
        super::*, solana_instruction::Instruction, solana_transaction_status::TransactionStatusMeta,
    };

    fn create_instruction_with_metadata(
        index: u32,
        stack_height: u32,
        absolute_path: Vec<u8>,
    ) -> (InstructionMetadata, Instruction) {
        let metadata = InstructionMetadata {
            transaction_metadata: Arc::new(TransactionMetadata {
                meta: TransactionStatusMeta {
                    log_messages: Some(vec!["Program CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK invoke [1]".to_string(), "Program data: QMbN6CYIceLh9Vdh3ndmrpChVVDCYAykCoHLEYdQWNcAxLJNu7nWNHiJzugda0JT2xgyBCWGtm7/oWjb/wT2kcbwA0JRUuwSV88ABSiDPpXudmLYK2jIBhqh3sTXxnR7WMgtjWsyqjga53NruXU9Dj/hyRRE/RQ9xCEh3052KbW6tbtNksNK4HIr+0wAAAAAAAAAAAAAAACz/t2FxQIAAAAAAAAAAAAAACdJpynsFrOoMAAAAAAAAAD4JhBoAxAAAAAAAAAAAAAAhC8BAA==".to_string(), "Program CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK consumed 91799 of 185765 compute units".to_string(), "Program CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK success".to_string()]),
                    ..Default::default()
                },
                ..Default::default()
            }),
            stack_height,
            index,
            absolute_path,
        };
        let instruction = Instruction {
            program_id: Pubkey::new_unique(),
            accounts: vec![AccountMeta::new(Pubkey::new_unique(), false)],
            data: vec![],
        };
        (metadata, instruction)
    }

    #[test]
    fn test_nested_instructions_single_level() {
        let instructions = vec![
            create_instruction_with_metadata(1, 1, vec![1]),
            create_instruction_with_metadata(2, 1, vec![2]),
        ];
        let nested_instructions: NestedInstructions = instructions.into();
        assert_eq!(nested_instructions.len(), 2);
        assert!(nested_instructions[0].inner_instructions.is_empty());
        assert!(nested_instructions[1].inner_instructions.is_empty());
    }

    #[test]
    fn test_nested_instructions_empty() {
        let instructions: InstructionsWithMetadata = vec![];
        let nested_instructions: NestedInstructions = instructions.into();
        assert!(nested_instructions.is_empty());
    }

    #[test]
    fn test_deep_nested_instructions() {
        let instructions = vec![
            create_instruction_with_metadata(0, 1, vec![0]),
            create_instruction_with_metadata(0, 1, vec![0]),
            create_instruction_with_metadata(1, 2, vec![0, 1]),
            create_instruction_with_metadata(1, 3, vec![0, 1, 1]),
            create_instruction_with_metadata(1, 3, vec![0, 1, 1]),
            create_instruction_with_metadata(1, 3, vec![0, 1, 1]),
            create_instruction_with_metadata(1, 3, vec![0, 1, 1]),
        ];

        let nested_instructions: NestedInstructions = instructions.into();
        assert_eq!(nested_instructions.len(), 2);
        assert_eq!(nested_instructions.0[1].inner_instructions.len(), 1);
    }

    #[test]
    fn test_extract_event_log_data() {
        let logs = create_instruction_with_metadata(0, 1, vec![0])
            .0
            .extract_event_log_data();
        assert_eq!(logs.len(), 1);
        assert_eq!(
            logs[0],
            base64::Engine::decode(
                &base64::engine::general_purpose::STANDARD,
                "QMbN6CYIceLh9Vdh3ndmrpChVVDCYAykCoHLEYdQWNcAxLJNu7nWNHiJzugda0JT2xgyBCWGtm7/oWjb/wT2kcbwA0JRUuwSV88ABSiDPpXudmLYK2jIBhqh3sTXxnR7WMgtjWsyqjga53NruXU9Dj/hyRRE/RQ9xCEh3052KbW6tbtNksNK4HIr+0wAAAAAAAAAAAAAAACz/t2FxQIAAAAAAAAAAAAAACdJpynsFrOoMAAAAAAAAAD4JhBoAxAAAAAAAAAAAAAAhC8BAA=="
            )
            .expect("decode base64")
        );
    }
}
