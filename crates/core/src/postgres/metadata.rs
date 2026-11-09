//! Conversions from pipeline metadata to the column-prefixes embedded
//! in `AccountRow` / `InstructionRow` via `#[sqlx(flatten)]`.

use crate::{
    account::AccountMetadata,
    instruction::InstructionMetadata,
    postgres::primitives::{Pubkey, U32, U64},
};

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct AccountRowMetadata {
    #[sqlx(rename = "__pubkey")]
    pub pubkey: Pubkey,
    #[sqlx(rename = "__slot")]
    pub slot: Option<U64>,
}

impl From<AccountMetadata> for AccountRowMetadata {
    fn from(metadata: AccountMetadata) -> Self {
        Self {
            pubkey: metadata.pubkey.into(),
            slot: Some(metadata.slot.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct InstructionRowMetadata {
    #[sqlx(rename = "__signature")]
    pub signature: String,
    #[sqlx(rename = "__instruction_index")]
    pub instruction_index: U32,
    #[sqlx(rename = "__stack_height")]
    pub stack_height: U32,
    #[sqlx(rename = "__absolute_path")]
    pub absolute_path: Vec<i16>,
    #[sqlx(rename = "__slot")]
    pub slot: Option<U64>,
}

impl From<InstructionMetadata> for InstructionRowMetadata {
    fn from(metadata: InstructionMetadata) -> Self {
        Self {
            signature: metadata.transaction_metadata.signature.to_string(),
            instruction_index: metadata.index.into(),
            stack_height: metadata.stack_height.into(),
            absolute_path: metadata
                .absolute_path
                .iter()
                .copied()
                .map(i16::from)
                .collect(),
            slot: Some(metadata.transaction_metadata.slot.into()),
        }
    }
}
