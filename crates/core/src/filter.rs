//! Filtering system for the carbon-core pipeline.
//!
//! This module provides a flexible filtering mechanism that allows you to selectively
//! process updates based on various criteria. Filters can be applied to different
//! types of updates (accounts, instructions, transactions, account deletions, and
//! block details) and can filter based on datasource IDs, update content, or any
//! other custom logic.
//!
//! # Overview
//!
//! The filtering system consists of:
//! - A `Filter` trait that defines the interface for all filters
//! - Concrete implementations like `DatasourceFilter` for common use cases
//! - Integration with the pipeline to apply filters before processing
//!
//! # Examples
//!
//! Basic datasource filtering:
//! ```
//! use carbon_core::{datasource::DatasourceId, filter::DatasourceFilter};
//!
//! let mainnet_id = DatasourceId::new_named("mainnet");
//! let filter = DatasourceFilter::new(mainnet_id);
//! ```
//!
//! Filtering multiple datasources:
//! ```
//! use carbon_core::{datasource::DatasourceId, filter::DatasourceFilter};
//!
//! let ids = vec![
//!     DatasourceId::new_named("mainnet"),
//!     DatasourceId::new_named("testnet"),
//! ];
//! let filter = DatasourceFilter::new_many(ids);
//! ```
//!
//! Custom filter implementation:
//! ```
//! use carbon_core::{
//!     datasource::{DatasourceId, BlockDetails},
//!     filter::Filter,
//! };
//!
//! struct BlockHeightFilter {
//!     min_height: u64,
//! }
//!
//! impl Filter for BlockHeightFilter {
//!     fn filter_block_details(
//!         &self,
//!         _datasource_id: &DatasourceId,
//!         block_details: &BlockDetails,
//!     ) -> bool {
//!         block_details.block_height >= self.min_height
//!     }
//!
//!     // Implement other methods with default behavior
//!     fn filter_account(&self, _: &DatasourceId, _: &_, _: &_) -> bool { true }
//!     fn filter_instruction(&self, _: &DatasourceId, _: &_) -> bool { true }
//!     fn filter_transaction(&self, _: &DatasourceId, _: &_, _: &_) -> bool { true }
//!     fn filter_account_deletion(&self, _: &DatasourceId, _: &_) -> bool { true }
//! }
//! ```

use crate::{
    account::AccountMetadata,
    datasource::{AccountDeletion, BlockDetails, DatasourceId},
    instruction::{NestedInstruction, NestedInstructions},
    transaction::TransactionMetadata,
};

/// A trait for filtering updates in the carbon-core pipeline.
///
/// Filters allow you to selectively process updates based on various criteria.
/// Each filter method returns `true` if the update should be processed, or
/// `false` if it should be skipped.
///
/// # Filter Methods
///
/// - `filter_account`: Filters account updates
/// - `filter_instruction`: Filters instruction updates  
/// - `filter_transaction`: Filters transaction updates
/// - `filter_account_deletion`: Filters account deletion updates
/// - `filter_block_details`: Filters block details updates
///
/// # Implementation Notes
///
/// When implementing this trait, you should:
/// - Return `true` for update types you want to process
/// - Return `false` for update types you want to skip
/// - Consider the datasource ID when making filtering decisions
/// - Keep filtering logic efficient as it runs for every update
///
/// # Examples
///
/// A simple datasource-based filter:
/// ```
/// use carbon_core::{
///     datasource::{DatasourceId, BlockDetails},
///     filter::Filter,
/// };
///
/// struct MyFilter {
///     allowed_datasource: DatasourceId,
/// }
///
/// impl Filter for MyFilter {
///     fn filter_block_details(
///         &self,
///         datasource_id: &DatasourceId,
///         _block_details: &BlockDetails,
///     ) -> bool {
///         datasource_id == &self.allowed_datasource
///     }
///
///     // Implement other methods with default behavior
///     fn filter_account(&self, _: &DatasourceId, _: &_, _: &_) -> bool { true }
///     fn filter_instruction(&self, _: &DatasourceId, _: &_) -> bool { true }
///     fn filter_transaction(&self, _: &DatasourceId, _: &_, _: &_) -> bool { true }
///     fn filter_account_deletion(&self, _: &DatasourceId, _: &_) -> bool { true }
/// }
/// ```
pub trait Filter {
    /// Filters account updates based on datasource ID and account data.
    ///
    /// This method is called for each account update before processing.
    /// Return `true` to process the account update, or `false` to skip it.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `account_metadata` - Metadata about the account update
    /// * `account` - The account data to be processed
    ///
    /// # Returns
    ///
    /// `true` if the account update should be processed, `false` otherwise.
    fn filter_account(
        &self,
        _datasource_id: &DatasourceId,
        _account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> bool {
        true
    }

    /// Filters instruction updates based on datasource ID and instruction data.
    ///
    /// This method is called for each instruction update before processing.
    /// Return `true` to process the instruction update, or `false` to skip it.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `nested_instruction` - The instruction data to be processed
    ///
    /// # Returns
    ///
    /// `true` if the instruction update should be processed, `false` otherwise.
    fn filter_instruction(
        &self,
        _datasource_id: &DatasourceId,
        _nested_instruction: &NestedInstruction,
    ) -> bool {
        true
    }

    /// Filters transaction updates based on datasource ID and transaction data.
    ///
    /// This method is called for each transaction update before processing.
    /// Return `true` to process the transaction update, or `false` to skip it.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `transaction_metadata` - Metadata about the transaction
    /// * `nested_instructions` - The instructions within the transaction
    ///
    /// # Returns
    ///
    /// `true` if the transaction update should be processed, `false` otherwise.
    fn filter_transaction(
        &self,
        _datasource_id: &DatasourceId,
        _transaction_metadata: &TransactionMetadata,
        _nested_instructions: &NestedInstructions,
    ) -> bool {
        true
    }

    /// Filters account deletion updates based on datasource ID and deletion data.
    ///
    /// This method is called for each account deletion update before processing.
    /// Return `true` to process the account deletion update, or `false` to skip it.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `account_deletion` - The account deletion data to be processed
    ///
    /// # Returns
    ///
    /// `true` if the account deletion update should be processed, `false` otherwise.
    fn filter_account_deletion(
        &self,
        _datasource_id: &DatasourceId,
        _account_deletion: &AccountDeletion,
    ) -> bool {
        true
    }

    /// Filters block details updates based on datasource ID and block data.
    ///
    /// This method is called for each block details update before processing.
    /// Return `true` to process the block details update, or `false` to skip it.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `block_details` - The block details data to be processed
    ///
    /// # Returns
    ///
    /// `true` if the block details update should be processed, `false` otherwise.
    fn filter_block_details(
        &self,
        _datasource_id: &DatasourceId,
        _block_details: &BlockDetails,
    ) -> bool {
        true
    }
}

/// A filter that allows updates from specific datasources.
///
/// This filter is useful when you want to process updates only from certain
/// datasources while ignoring others. For example, you might want to process
/// updates from a mainnet datasource but skip testnet updates.
///
/// # Examples
///
/// Filter for a single datasource:
/// ```
/// use carbon_core::{datasource::DatasourceId, filter::DatasourceFilter};
///
/// let mainnet_id = DatasourceId::new_named("mainnet");
/// let filter = DatasourceFilter::new(mainnet_id);
/// ```
///
/// Filter for multiple datasources:
/// ```
/// use carbon_core::{datasource::DatasourceId, filter::DatasourceFilter};
///
/// let ids = vec![
///     DatasourceId::new_named("mainnet"),
///     DatasourceId::new_named("testnet"),
/// ];
/// let filter = DatasourceFilter::new_many(ids);
/// ```
///
/// Using with pipeline builders:
/// ```
/// use carbon_core::{datasource::DatasourceId, filter::DatasourceFilter};
///
/// let filter = DatasourceFilter::new(DatasourceId::new_named("mainnet"));
/// let filters = vec![Box::new(filter) as Box<dyn Filter>];
///
/// // Use with pipeline builder
/// // pipeline.account_with_filters(decoder, processor, filters);
/// ```
pub struct DatasourceFilter {
    /// The list of datasource IDs that are allowed to pass through this filter.
    ///
    /// Updates from datasources with IDs in this list will be processed.
    /// Updates from other datasources will be filtered out.
    pub allowed_datasources: Vec<DatasourceId>,
}

impl DatasourceFilter {
    /// Creates a new datasource filter that allows updates from a single datasource.
    ///
    /// This is the most common use case when you want to filter for updates
    /// from a specific datasource.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource to allow
    ///
    /// # Returns
    ///
    /// A new `DatasourceFilter` that only allows updates from the specified datasource.
    ///
    /// # Examples
    ///
    /// ```
    /// use carbon_core::{datasource::DatasourceId, filter::DatasourceFilter};
    ///
    /// let mainnet_id = DatasourceId::new_named("mainnet");
    /// let filter = DatasourceFilter::new(mainnet_id);
    /// ```
    pub fn new(datasource_id: DatasourceId) -> Self {
        Self {
            allowed_datasources: vec![datasource_id],
        }
    }

    /// Creates a new datasource filter that allows updates from multiple datasources.
    ///
    /// This is useful when you want to process updates from several datasources
    /// while filtering out others.
    ///
    /// # Arguments
    ///
    /// * `datasource_ids` - A vector of datasource IDs to allow
    ///
    /// # Returns
    ///
    /// A new `DatasourceFilter` that allows updates from all specified datasources.
    ///
    /// # Examples
    ///
    /// ```
    /// use carbon_core::{datasource::DatasourceId, filter::DatasourceFilter};
    ///
    /// let ids = vec![
    ///     DatasourceId::new_named("mainnet"),
    ///     DatasourceId::new_named("testnet"),
    /// ];
    /// let filter = DatasourceFilter::new_many(ids);
    /// ```
    pub fn new_many(datasource_ids: Vec<DatasourceId>) -> Self {
        Self {
            allowed_datasources: datasource_ids,
        }
    }
}

impl Filter for DatasourceFilter {
    /// Filters account updates based on the datasource ID.
    ///
    /// Returns `true` if the account update comes from an allowed datasource,
    /// `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `_account_metadata` - Account metadata (unused in this implementation)
    /// * `_account` - Account data (unused in this implementation)
    ///
    /// # Returns
    ///
    /// `true` if the datasource ID is in the allowed list, `false` otherwise.
    fn filter_account(
        &self,
        datasource_id: &DatasourceId,
        _account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    /// Filters instruction updates based on the datasource ID.
    ///
    /// Returns `true` if the instruction update comes from an allowed datasource,
    /// `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `_nested_instruction` - Instruction data (unused in this implementation)
    ///
    /// # Returns
    ///
    /// `true` if the datasource ID is in the allowed list, `false` otherwise.
    fn filter_instruction(
        &self,
        datasource_id: &DatasourceId,
        _nested_instruction: &NestedInstruction,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    /// Filters transaction updates based on the datasource ID.
    ///
    /// Returns `true` if the transaction update comes from an allowed datasource,
    /// `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `_transaction_metadata` - Transaction metadata (unused in this implementation)
    /// * `_nested_instructions` - Transaction instructions (unused in this implementation)
    ///
    /// # Returns
    ///
    /// `true` if the datasource ID is in the allowed list, `false` otherwise.
    fn filter_transaction(
        &self,
        datasource_id: &DatasourceId,
        _transaction_metadata: &TransactionMetadata,
        _nested_instructions: &NestedInstructions,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    /// Filters account deletion updates based on the datasource ID.
    ///
    /// Returns `true` if the account deletion update comes from an allowed datasource,
    /// `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `_account_deletion` - Account deletion data (unused in this implementation)
    ///
    /// # Returns
    ///
    /// `true` if the datasource ID is in the allowed list, `false` otherwise.
    fn filter_account_deletion(
        &self,
        datasource_id: &DatasourceId,
        _account_deletion: &AccountDeletion,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    /// Filters block details updates based on the datasource ID.
    ///
    /// Returns `true` if the block details update comes from an allowed datasource,
    /// `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `_block_details` - Block details data (unused in this implementation)
    ///
    /// # Returns
    ///
    /// `true` if the datasource ID is in the allowed list, `false` otherwise.
    fn filter_block_details(
        &self,
        datasource_id: &DatasourceId,
        _block_details: &BlockDetails,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }
}
