use {
    crate::{
        deserialize::CarbonDeserialize, error::CarbonResult, filter::Filter,
        metrics::MetricsCollection, processor::Processor, transaction::TransactionMetadata,
    },
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    solana_pubkey::Pubkey,
    std::{
        ops::{Deref, DerefMut},
        sync::Arc,
    },
};

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

const PRECOMPILE_PROGRAMS: &[&str] = &[
    "Ed25519SigVerify111111111111111111111111111",
    "KeccakSecp256k11111111111111111111111111111",
    "Secp256r1SigVerify1111111111111111111111111",
];

impl InstructionMetadata {
    pub fn decode_log_events<T: CarbonDeserialize>(&self) -> Vec<T> {
        self.extract_event_log_data()
            .into_iter()
            .filter(|log| log.len() >= 8)
            .filter_map(|log| <T as CarbonDeserialize>::deserialize(&log))
            .collect()
    }

    fn extract_event_log_data(&self) -> Vec<Vec<u8>> {
        let logs = match &self.transaction_metadata.meta.log_messages {
            Some(logs) => logs,
            None => return Vec::new(),
        };

        let precompile_offset = self.count_precompiles_before_index();
        let adjusted_absolute_path: Vec<u8> = if !self.absolute_path.is_empty() {
            let mut adjusted = self.absolute_path.clone();
            adjusted[0] = adjusted[0].saturating_sub(precompile_offset as u8);
            adjusted
        } else {
            self.absolute_path.clone()
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

            if current_path == adjusted_absolute_path && matches!(parsed_log, LogType::Data) {
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

    fn count_precompiles_before_index(&self) -> usize {
        if self.absolute_path.is_empty() {
            return 0;
        }

        let outer_index = self.absolute_path[0] as usize;
        let account_keys = self.transaction_metadata.message.static_account_keys();
        let instructions = self.transaction_metadata.message.instructions();

        let mut precompile_count = 0;
        for (idx, ix) in instructions.iter().enumerate() {
            if idx >= outer_index {
                break;
            }
            if let Some(program_id) = account_keys.get(ix.program_id_index as usize) {
                let program_id_str = program_id.to_string();
                if PRECOMPILE_PROGRAMS.contains(&program_id_str.as_str()) {
                    precompile_count += 1;
                }
            }
        }

        precompile_count
    }

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodedInstruction<T, A> {
    pub program_id: Pubkey,
    pub data: T,
    pub accounts: A,
}

pub trait InstructionDecoder<'a> {
    type InstructionType;
    type ArrangedAccounts;

    fn decode_instruction(
        &self,
        instruction: &'a solana_instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType, Self::ArrangedAccounts>>;
}

pub type InstructionProcessorInputType<T, A> = (
    InstructionMetadata,
    DecodedInstruction<T, A>,
    NestedInstructions,
    solana_instruction::Instruction,
);

pub struct InstructionPipe<T: Send, A: Send> {
    pub decoder: Box<
        dyn for<'a> InstructionDecoder<'a, InstructionType = T, ArrangedAccounts = A>
            + Send
            + Sync
            + 'static,
    >,
    pub processor:
        Box<dyn Processor<InputType = InstructionProcessorInputType<T, A>> + Send + Sync + 'static>,
    pub filters: Vec<Box<dyn Filter + Send + Sync + 'static>>,
}

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
impl<T: Send + 'static, A: Send + 'static> InstructionPipes<'_> for InstructionPipe<T, A> {
    async fn run(
        &mut self,
        nested_instruction: &NestedInstruction,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!("InstructionPipe::run(nested_instruction: {nested_instruction:?}, metrics)",);

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

impl From<InstructionsWithMetadata> for NestedInstructions {
    fn from(instructions: InstructionsWithMetadata) -> Self {
        log::trace!("from(instructions: {instructions:?})");

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
        super::*, solana_instruction::Instruction,
        solana_transaction_status::TransactionStatusMeta, std::str::FromStr,
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

    fn create_metadata_with_message(
        absolute_path: Vec<u8>,
        stack_height: u32,
        logs: Vec<String>,
        account_keys: Vec<Pubkey>,
        instructions: Vec<solana_message::compiled_instruction::CompiledInstruction>,
    ) -> InstructionMetadata {
        use solana_message::{legacy::Message as LegacyMessage, VersionedMessage};

        let message = VersionedMessage::Legacy(LegacyMessage {
            header: solana_message::MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 0,
            },
            account_keys,
            recent_blockhash: solana_hash::Hash::default(),
            instructions,
        });

        InstructionMetadata {
            transaction_metadata: Arc::new(TransactionMetadata {
                meta: TransactionStatusMeta {
                    log_messages: Some(logs),
                    ..Default::default()
                },
                message,
                ..Default::default()
            }),
            stack_height,
            index: absolute_path.first().copied().unwrap_or(0) as u32,
            absolute_path,
        }
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

    #[test]
    fn test_count_precompiles_before_index_no_precompiles() {
        use solana_message::compiled_instruction::CompiledInstruction;

        // Transaction structure:
        // ix 0: program1
        // ix 1: program2
        //
        // No precompiles present, so count before ix 1 should be 0.

        let program1 = Pubkey::new_unique();
        let program2 = Pubkey::new_unique();

        let metadata = create_metadata_with_message(
            vec![1],
            1,
            vec![],
            vec![Pubkey::new_unique(), program1, program2],
            vec![
                CompiledInstruction {
                    program_id_index: 1,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 2,
                    accounts: vec![],
                    data: vec![],
                },
            ],
        );

        assert_eq!(metadata.count_precompiles_before_index(), 0);
    }

    #[test]
    fn test_count_precompiles_before_index_with_ed25519() {
        use solana_message::compiled_instruction::CompiledInstruction;

        // Transaction structure:
        // ix 0: Ed25519 precompile
        // ix 1: program1
        // ix 2: program2
        //
        // One precompile before ix 2, so count should be 1.

        let ed25519 = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();
        let program1 = Pubkey::new_unique();
        let program2 = Pubkey::new_unique();

        let metadata = create_metadata_with_message(
            vec![2],
            1,
            vec![],
            vec![Pubkey::new_unique(), ed25519, program1, program2],
            vec![
                CompiledInstruction {
                    program_id_index: 1,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 2,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 3,
                    accounts: vec![],
                    data: vec![],
                },
            ],
        );

        assert_eq!(metadata.count_precompiles_before_index(), 1);
    }

    #[test]
    fn test_count_precompiles_before_index_with_multiple_precompiles() {
        use solana_message::compiled_instruction::CompiledInstruction;

        // Transaction structure:
        // ix 0: Ed25519 precompile
        // ix 1: Secp256k1 precompile
        // ix 2: program1
        // ix 3: program2
        //
        // Two precompiles before ix 3, so count should be 2.

        let ed25519 = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();
        let secp256k1 = Pubkey::from_str("KeccakSecp256k11111111111111111111111111111").unwrap();
        let program1 = Pubkey::new_unique();
        let program2 = Pubkey::new_unique();

        let metadata = create_metadata_with_message(
            vec![3],
            1,
            vec![],
            vec![Pubkey::new_unique(), ed25519, secp256k1, program1, program2],
            vec![
                CompiledInstruction {
                    program_id_index: 1,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 2,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 3,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 4,
                    accounts: vec![],
                    data: vec![],
                },
            ],
        );

        assert_eq!(metadata.count_precompiles_before_index(), 2);
    }

    #[test]
    fn test_extract_event_log_data_with_precompile_offset() {
        use solana_message::compiled_instruction::CompiledInstruction;

        // Transaction structure:
        // ix 0: ComputeBudget (logs invoke [1])
        // ix 1: Ed25519 precompile (NO logs)
        // ix 2: target_program (logs invoke [1])
        //
        // Log positions (ignoring precompiles):
        // Position [0] = ComputeBudget
        // Position [1] = target_program (message index 2, but log position 1 due to 1 precompile)

        let compute_budget = Pubkey::new_unique();
        let ed25519 = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();
        let target_program = Pubkey::new_unique();

        let logs = vec![
            "Program ComputeBudget111111111111111111111111111 invoke [1]".to_string(),
            "Program ComputeBudget111111111111111111111111111 success".to_string(),
            format!("Program {} invoke [1]", target_program),
            "Program data: dGVzdF9kYXRh".to_string(), // "test_data" in base64
            format!("Program {} success", target_program),
        ];

        let metadata = create_metadata_with_message(
            vec![2],
            1,
            logs,
            vec![
                Pubkey::new_unique(),
                compute_budget,
                ed25519,
                target_program,
            ],
            vec![
                CompiledInstruction {
                    program_id_index: 1,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 2,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 3,
                    accounts: vec![],
                    data: vec![],
                },
            ],
        );

        let extracted = metadata.extract_event_log_data();
        assert_eq!(extracted.len(), 1);
        assert_eq!(extracted[0], b"test_data");
    }

    #[test]
    fn test_extract_event_log_data_cpi_with_precompile_offset() {
        use solana_message::compiled_instruction::CompiledInstruction;

        // Transaction structure:
        // ix 0: ComputeBudget (logs invoke [1])
        // ix 1: Ed25519 precompile (NO logs)
        // ix 2: router_program (logs invoke [1])
        //   -> CPI to target_program (logs invoke [2])
        //
        // Log positions (ignoring precompiles):
        // Position [0] = ComputeBudget
        // Position [1] = router_program (message index 2, but log position 1 due to 1 precompile)
        // Position [1, 0] = target_program CPI

        let compute_budget = Pubkey::new_unique();
        let ed25519 = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();
        let router_program = Pubkey::new_unique();
        let target_program = Pubkey::new_unique();

        let logs = vec![
            "Program ComputeBudget111111111111111111111111111 invoke [1]".to_string(),
            "Program ComputeBudget111111111111111111111111111 success".to_string(),
            format!("Program {} invoke [1]", router_program),
            format!("Program {} invoke [2]", target_program),
            "Program data: Y3BpX2RhdGE=".to_string(), // "cpi_data" in base64
            format!("Program {} success", target_program),
            format!("Program {} success", router_program),
        ];

        let metadata = create_metadata_with_message(
            vec![2, 0],
            2,
            logs,
            vec![
                Pubkey::new_unique(),
                compute_budget,
                ed25519,
                router_program,
                target_program,
            ],
            vec![
                CompiledInstruction {
                    program_id_index: 1,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 2,
                    accounts: vec![],
                    data: vec![],
                },
                CompiledInstruction {
                    program_id_index: 3,
                    accounts: vec![],
                    data: vec![],
                },
            ],
        );

        let extracted = metadata.extract_event_log_data();
        assert_eq!(extracted.len(), 1);
        assert_eq!(extracted[0], b"cpi_data");
    }

    #[test]
    fn test_extract_event_log_data_cpi_with_multiple_precompiles_and_instructions() {
        use solana_message::compiled_instruction::CompiledInstruction;

        let compute_budget = Pubkey::new_unique();
        let ed25519 = Pubkey::from_str("Ed25519SigVerify111111111111111111111111111").unwrap();
        let secp256k1 = Pubkey::from_str("KeccakSecp256k11111111111111111111111111111").unwrap();
        let router_program = Pubkey::new_unique();
        let swap_program = Pubkey::new_unique();
        let token_program = Pubkey::new_unique();

        // Transaction structure:
        // ix 0: ComputeBudget (logs invoke [1])
        // ix 1: Ed25519 precompile
        // ix 2: Secp256k1 precompile
        // ix 3: Router program (logs invoke [1])
        //   -> CPI to swap_program (logs invoke [2])
        //      -> CPI to token_program (logs invoke [3])
        //   -> CPI to swap_program again (logs invoke [2])
        //
        // Log positions (ignoring precompiles):
        // Position [0] = ComputeBudget
        // Position [1] = Router (message index 3, but log position 1 due to 2 precompiles)
        // Position [1, 0] = first swap CPI
        // Position [1, 0, 0] = token CPI inside first swap
        // Position [1, 1] = second swap CPI

        let logs = vec![
            "Program ComputeBudget111111111111111111111111111 invoke [1]".to_string(),
            "Program ComputeBudget111111111111111111111111111 success".to_string(),
            format!("Program {} invoke [1]", router_program),
            format!("Program {} invoke [2]", swap_program),
            format!("Program {} invoke [3]", token_program),
            "Program data: dG9rZW5fZGF0YQ==".to_string(), // "token_data" in base64
            format!("Program {} success", token_program),
            "Program data: c3dhcF9kYXRhXzE=".to_string(), // "swap_data_1" in base64
            format!("Program {} success", swap_program),
            format!("Program {} invoke [2]", swap_program),
            "Program data: c3dhcF9kYXRhXzI=".to_string(), // "swap_data_2" in base64
            format!("Program {} success", swap_program),
            "Program data: cm91dGVyX2RhdGE=".to_string(), // "router_data" in base64
            format!("Program {} success", router_program),
        ];

        let account_keys = vec![
            Pubkey::new_unique(),
            compute_budget,
            ed25519,
            secp256k1,
            router_program,
            swap_program,
            token_program,
        ];

        let instructions = vec![
            CompiledInstruction {
                program_id_index: 1,
                accounts: vec![],
                data: vec![],
            },
            CompiledInstruction {
                program_id_index: 2,
                accounts: vec![],
                data: vec![],
            },
            CompiledInstruction {
                program_id_index: 3,
                accounts: vec![],
                data: vec![],
            },
            CompiledInstruction {
                program_id_index: 4,
                accounts: vec![],
                data: vec![],
            },
        ];

        let router_metadata = create_metadata_with_message(
            vec![3],
            1,
            logs.clone(),
            account_keys.clone(),
            instructions.clone(),
        );
        let router_extracted = router_metadata.extract_event_log_data();
        assert_eq!(router_extracted.len(), 1);
        assert_eq!(router_extracted[0], b"router_data");

        let first_swap_metadata = create_metadata_with_message(
            vec![3, 0],
            2,
            logs.clone(),
            account_keys.clone(),
            instructions.clone(),
        );
        let first_swap_extracted = first_swap_metadata.extract_event_log_data();
        assert_eq!(first_swap_extracted.len(), 1);
        assert_eq!(first_swap_extracted[0], b"swap_data_1");

        let token_metadata = create_metadata_with_message(
            vec![3, 0, 0],
            3,
            logs.clone(),
            account_keys.clone(),
            instructions.clone(),
        );
        let token_extracted = token_metadata.extract_event_log_data();
        assert_eq!(token_extracted.len(), 1);
        assert_eq!(token_extracted[0], b"token_data");

        let second_swap_metadata = create_metadata_with_message(
            vec![3, 1],
            2,
            logs.clone(),
            account_keys.clone(),
            instructions.clone(),
        );
        let second_swap_extracted = second_swap_metadata.extract_event_log_data();
        assert_eq!(second_swap_extracted.len(), 1);
        assert_eq!(second_swap_extracted[0], b"swap_data_2");

        assert_eq!(router_metadata.count_precompiles_before_index(), 2);
        assert_eq!(first_swap_metadata.count_precompiles_before_index(), 2);
    }
}
