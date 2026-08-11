//! Metadata columns shared by generated ClickHouse rows.

use crate::{account::AccountMetadata, instruction::InstructionMetadata};

/// Common columns written with decoded account rows.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountRowMetadata {
    #[serde(rename = "__pubkey")]
    pub pubkey: String,
    #[serde(rename = "__slot")]
    pub slot: u64,
}

impl From<AccountMetadata> for AccountRowMetadata {
    fn from(metadata: AccountMetadata) -> Self {
        Self {
            pubkey: metadata.pubkey.to_string(),
            slot: metadata.slot,
        }
    }
}

/// Common columns written with generated instruction rows.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstructionRowMetadata {
    #[serde(rename = "__signature")]
    pub signature: String,
    #[serde(rename = "__instruction_index")]
    pub instruction_index: u32,
    #[serde(rename = "__stack_height")]
    pub stack_height: u32,
    #[serde(rename = "__slot")]
    pub slot: u64,
}

impl From<InstructionMetadata> for InstructionRowMetadata {
    fn from(metadata: InstructionMetadata) -> Self {
        Self {
            signature: metadata.transaction_metadata.signature.to_string(),
            instruction_index: metadata.index,
            stack_height: metadata.stack_height,
            slot: metadata.transaction_metadata.slot,
        }
    }
}
