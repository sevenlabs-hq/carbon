//! Provides PostgreSQL row metadata structures for the `carbon-core` framework.
//!
//! This module defines database row metadata types that bridge between the
//! core framework's metadata structures and PostgreSQL table schemas. These
//! types are designed to work seamlessly with `sqlx` for database operations,
//! providing type-safe conversion from framework metadata to database rows.
//!
//! # Overview
//!
//! The metadata module includes:
//! - **AccountRowMetadata**: PostgreSQL row structure for account metadata,
//!   including public key and slot information.
//! - **InstructionRowMetadata**: PostgreSQL row structure for instruction
//!   metadata, including signature, index, stack height, and slot information.
//!
//! # Key Components
//!
//! ## AccountRowMetadata
//!
//! Represents account metadata as stored in PostgreSQL tables:
//! - **pubkey**: The account's public key (stored as `__pubkey`)
//! - **slot**: The slot when the account was last updated (stored as `__slot`)
//!
//! ## InstructionRowMetadata
//!
//! Represents instruction metadata as stored in PostgreSQL tables:
//! - **signature**: The transaction signature (stored as `__signature`)
//! - **index**: The instruction index within the transaction (stored as `__instruction_index`)
//! - **stack_height**: The call stack height for nested instructions (stored as `__stack_height`)
//! - **slot**: The slot when the instruction was executed (stored as `__slot`)
//!
//! # Database Schema
//!
//! These types are designed to work with PostgreSQL tables that include
//! metadata columns with double-underscore prefixes to distinguish them
//! from application-specific data columns:
//!
//! ```sql
//! -- Example account table schema
//! CREATE TABLE accounts (
//!     __pubkey BYTEA NOT NULL,
//!     __slot BIGINT,
//!     -- application-specific columns...
//! );
//!
//! -- Example instruction table schema
//! CREATE TABLE instructions (
//!     __signature TEXT NOT NULL,
//!     __instruction_index INTEGER NOT NULL,
//!     __stack_height INTEGER NOT NULL,
//!     __slot BIGINT,
//!     -- application-specific columns...
//! );
//! ```
//!
//! # Usage Examples
//!
//! ## Converting from framework metadata
//!
//! ```ignore
//! use carbon_core::{
//!     account::AccountMetadata,
//!     instruction::InstructionMetadata,
//!     postgres::metadata::{AccountRowMetadata, InstructionRowMetadata},
//! };
//!
//! // Convert account metadata
//! let account_metadata = AccountMetadata {
//!     pubkey: solana_pubkey::Pubkey::new_unique(),
//!     slot: 12345,
//! };
//! let row_metadata: AccountRowMetadata = account_metadata.into();
//!
//! // Convert instruction metadata
//! let instruction_metadata = InstructionMetadata {
//!     transaction_metadata: TransactionMetadata {
//!         signature: solana_signature::Signature::new_unique(),
//!         slot: 12345,
//!         fee_payer: solana_pubkey::Pubkey::new_unique(),
//!     },
//!     index: 0,
//!     stack_height: 0,
//! };
//! let row_metadata: InstructionRowMetadata = instruction_metadata.into();
//! ```
//!
//! ## Using with SQLx queries
//!
//! ```ignore
//! use carbon_core::postgres::metadata::{AccountRowMetadata, InstructionRowMetadata};
//! use sqlx::PgPool;
//!
//! async fn get_account_metadata(pool: &PgPool, pubkey: Pubkey) -> Result<Option<AccountRowMetadata>, sqlx::Error> {
//!     sqlx::query_as!(
//!         AccountRowMetadata,
//!         "SELECT __pubkey, __slot FROM accounts WHERE __pubkey = $1",
//!         pubkey
//!     )
//!     .fetch_optional(pool)
//!     .await
//! }
//!
//! async fn get_instruction_metadata(pool: &PgPool, signature: &str, index: u32) -> Result<Option<InstructionRowMetadata>, sqlx::Error> {
//!     sqlx::query_as!(
//!         InstructionRowMetadata,
//!         "SELECT __signature, __instruction_index, __stack_height, __slot FROM instructions WHERE __signature = $1 AND __instruction_index = $2",
//!         signature,
//!         index
//!     )
//!     .fetch_optional(pool)
//!     .await
//! }
//! ```
//!
//! # Notes
//!
//! - All metadata fields use PostgreSQL primitive types from the `primitives` module
//! - The `From` implementations provide seamless conversion from framework metadata
//! - Column names use double-underscore prefixes to avoid conflicts with application data
//! - Optional fields (like `slot`) allow for partial metadata storage
//! - These types implement `sqlx::FromRow` for direct use in SQLx queries

use crate::{
    account::AccountMetadata,
    instruction::InstructionMetadata,
    postgres::primitives::{Pubkey, U32, U64},
};

/// PostgreSQL row metadata for account information.
///
/// This type represents account metadata as stored in PostgreSQL tables,
/// providing a bridge between the framework's `AccountMetadata` and database
/// row structures. It includes essential account identification and timing
/// information.
///
/// # Database Schema
///
/// ```sql
/// CREATE TABLE accounts (
///     __pubkey BYTEA NOT NULL,
///     __slot BIGINT,
///     -- application-specific columns...
/// );
/// ```
///
/// # Fields
///
/// - **pubkey**: The account's public key, stored as `BYTEA` in PostgreSQL
/// - **slot**: The slot when the account was last updated (optional)
///
/// # Example
///
/// ```ignore
/// use carbon_core::postgres::metadata::AccountRowMetadata;
/// use carbon_core::account::AccountMetadata;
///
/// let account_metadata = AccountMetadata {
///     pubkey: solana_pubkey::Pubkey::new_unique(),
///     slot: 12345,
/// };
/// let row_metadata: AccountRowMetadata = account_metadata.into();
/// ```
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

/// PostgreSQL row metadata for instruction information.
///
/// This type represents instruction metadata as stored in PostgreSQL tables,
/// providing a bridge between the framework's `InstructionMetadata` and database
/// row structures. It includes transaction identification, instruction positioning,
/// and execution context information.
///
/// # Database Schema
///
/// ```sql
/// CREATE TABLE instructions (
///     __signature TEXT NOT NULL,
///     __instruction_index INTEGER NOT NULL,
///     __stack_height INTEGER NOT NULL,
///     __slot BIGINT,
///     -- application-specific columns...
/// );
/// ```
///
/// # Fields
///
/// - **signature**: The transaction signature that contains this instruction
/// - **index**: The zero-based index of this instruction within the transaction
/// - **stack_height**: The call stack height for nested instruction calls
/// - **slot**: The slot when the instruction was executed (optional)
///
/// # Example
///
/// ```ignore
/// use carbon_core::postgres::metadata::InstructionRowMetadata;
/// use carbon_core::instruction::InstructionMetadata;
///
/// let instruction_metadata = InstructionMetadata {
///     transaction_metadata: TransactionMetadata {
///         signature: solana_signature::Signature::new_unique(),
///         slot: 12345,
///         fee_payer: solana_pubkey::Pubkey::new_unique(),
///     },
///     index: 0,
///     stack_height: 0,
/// };
/// let row_metadata: InstructionRowMetadata = instruction_metadata.into();
/// ```
#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct InstructionRowMetadata {
    #[sqlx(rename = "__signature")]
    pub signature: String,
    #[sqlx(rename = "__instruction_index")]
    pub instruction_index: U32,
    #[sqlx(rename = "__stack_height")]
    pub stack_height: U32,
    #[sqlx(rename = "__slot")]
    pub slot: Option<U64>,
}

impl From<InstructionMetadata> for InstructionRowMetadata {
    fn from(metadata: InstructionMetadata) -> Self {
        Self {
            signature: metadata.transaction_metadata.signature.to_string(),
            instruction_index: metadata.index.into(),
            stack_height: metadata.stack_height.into(),
            slot: Some(metadata.transaction_metadata.slot.into()),
        }
    }
}
