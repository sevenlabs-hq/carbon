use crate::{
    account::AccountMetadata,
    instruction::InstructionMetadata,
    postgres::primitives::{Pubkey, U32, U64},
};

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
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

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct InstructionRowMetadata {
    #[sqlx(rename = "__signature")]
    pub signature: String,
    #[sqlx(rename = "__instruction_index")]
    pub index: U32,
    #[sqlx(rename = "__stack_height")]
    pub stack_height: U32,
    #[sqlx(rename = "__slot")]
    pub slot: Option<U64>,
}

impl From<InstructionMetadata> for InstructionRowMetadata {
    fn from(metadata: InstructionMetadata) -> Self {
        Self {
            signature: metadata.transaction_metadata.signature.to_string(),
            index: metadata.index.into(),
            stack_height: metadata.stack_height.into(),
            slot: Some(metadata.transaction_metadata.slot.into()),
        }
    }
}
