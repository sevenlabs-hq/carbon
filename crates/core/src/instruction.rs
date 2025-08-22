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
        error::CarbonResult, filter::Filter, metrics::MetricsCollection, processor::Processor,
        transaction::TransactionMetadata,
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
    pub fn extract_logs(&self) -> Vec<String> {
        let logs = match &self.transaction_metadata.meta.log_messages {
            Some(logs) => logs,
            None => return Vec::new(),
        };

        let mut extracted_logs = Vec::new();
        let mut current_path: Vec<u8> = vec![];
        let mut position_counters: Vec<u8> = vec![0]; // Track position at each stack height

        for log in logs {
            let parsed_log = self.parse_log(log);

            match parsed_log {
                LogType::Start(stack_height) => {
                    // Adjust position counters for this stack height
                    if stack_height > position_counters.len() {
                        // Going deeper - add new level starting at 0
                        position_counters.push(0);
                    } else {
                        // Same or higher level - increment counter at this level
                        position_counters[stack_height - 1] += 1;
                        // Clear deeper levels
                        position_counters.truncate(stack_height);
                    }

                    // Update current path from position counters
                    current_path = position_counters[..stack_height]
                        .iter()
                        .map(|&x| x)
                        .collect();
                }
                LogType::Finish => {
                    // Don't change path during finish processing
                }
                LogType::Data | LogType::CU => {
                    // Don't change path for data/CU logs
                }
            }

            // Extract if we're on target path and it's a data log (not CU)
            if current_path == self.absolute_path {
                match parsed_log {
                    LogType::Data => {
                        extracted_logs.push(log.clone());
                    }
                    _ => {} // Skip Start, Finish, and CU logs
                }
            }

            // After processing finish, pop the path
            if let LogType::Finish = parsed_log {
                current_path.pop();
            }
        }

        extracted_logs
    }

    // /// Extracts only the logs belonging to this specific instruction
    // pub fn extract_logs(&self) -> Vec<String> {
    //     let logs = match &self.transaction_metadata.meta.log_messages {
    //         Some(logs) => logs,
    //         None => return Vec::new(),
    //     };

    //     if self.absolute_path.is_empty() {
    //         return Vec::new();
    //     }

    //     let mut extracted_logs: Vec<String> = vec![];
    //     let mut current_ixs_path: Vec<u8> = vec![];
    //     let mut potential_next: Option<u8> = None;

    //     for log in logs {
    //         let parsed_log = self.parse_log(log);

    //         match parsed_log {
    //             LogType::Start(log_stack_height) => {
    //                 if log_stack_height > current_ixs_path.len() {
    //                     // New deeper level - add position 0
    //                     current_ixs_path.push(potential_next.take().unwrap_or(0));
    //                 } else if log_stack_height == current_ixs_path.len() {
    //                     potential_next = None;
    //                     // Same level - increment position
    //                     if let Some(last) = current_ixs_path.last_mut() {
    //                         *last += 1;
    //                     }
    //                 }
    //                 // else {
    //                 //     // Going back to higher level
    //                 // let target_len = log_stack_height;
    //                 // current_ixs_path.truncate(target_len);

    //                 //     // Increment position at current level
    //                 //     if let Some(last) = current_ixs_path.last_mut() {
    //                 //         *last += 1;
    //                 //     }
    //                 // }
    //             }
    //             LogType::Finish => {
    //                 potential_next = current_ixs_path.last().map(|x| x + 1);
    //                 current_ixs_path.pop();
    //             }
    //             _ => {}
    //         }

    //         // Check if current path matches our target path
    //         if current_ixs_path.len() == self.absolute_path.len()
    //             && current_ixs_path == self.absolute_path
    //         {
    //             if let LogType::Data = parsed_log {
    //                 extracted_logs.push(log.clone());
    //             }
    //             // if let LogType::Finish = parsed_log {
    //             //     break;
    //             // }
    //         }

    //         // if is_prev_log_finish
    //         //     && current_ixs_path.len() > 0
    //         //     && current_ixs_path[..current_ixs_path.len() - 1] == self.absolute_path[..]
    //         // {
    //         //     match parsed_log {
    //         //         LogType::Data => {
    //         //             extracted_logs.push(log.clone());
    //         //         }
    //         //         LogType::Finish => {
    //         //             extracted_logs.push(log.clone());
    //         //         }
    //         //         _ => {}
    //         //     }
    //         // }
    //     }

    //     extracted_logs
    // }

    /// Parses a log line to determine its type
    fn parse_log(&self, log: &str) -> LogType {
        if log.starts_with("Program ") && log.contains(" invoke [") {
            // Parse: "Program <program_id> invoke [level]"
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
            // Parse: "Program <program_id> success" or "Program <program_id> failed"
            let parts: Vec<&str> = log.split_whitespace().collect();
            if parts.len() >= 3 && parts[0] == "Program" {
                return LogType::Finish;
            }
        } else if log.contains("consumed") && log.contains("compute units") {
            return LogType::CU;
        }

        // Everything else is data (program logs, consumed logs, return logs, etc.)
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

    use {super::*, solana_instruction::Instruction};

    fn create_instruction_with_metadata(
        index: u32,
        stack_height: u32,
    ) -> (InstructionMetadata, Instruction) {
        let metadata = InstructionMetadata {
            transaction_metadata: Arc::default(),
            stack_height,
            index,
            absolute_path: vec![],
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
            create_instruction_with_metadata(1, 1),
            create_instruction_with_metadata(2, 1),
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
            create_instruction_with_metadata(0, 1),
            create_instruction_with_metadata(0, 1),
            create_instruction_with_metadata(1, 2),
            create_instruction_with_metadata(1, 3),
            create_instruction_with_metadata(1, 3),
            create_instruction_with_metadata(1, 3),
        ];

        let nested_instructions: NestedInstructions = instructions.into();
        assert_eq!(nested_instructions.len(), 2);
        assert_eq!(nested_instructions.0[1].inner_instructions.len(), 1);
    }
}
