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
    error::CarbonResult,
    instruction::{NestedInstruction, NestedInstructions},
    transaction::TransactionMetadata,
};
use async_trait::async_trait;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
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
#[async_trait]
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

/// Deduplication filter for eliminating duplicate updates within the same Solana slot.
///
/// This module provides a `DedupFilter` that prevents duplicate processing of updates
/// that occur within the same slot. The filter maintains state on a per-slot basis,
/// automatically clearing deduplication data when transitioning to a new slot to
/// keep memory usage bounded.
///
/// # Overview
///
/// The deduplication filter tracks unique identifiers for each update type:
/// - **Account updates**: Deduplicated by `(pubkey, lamports, hashed_data)`
/// - **Instruction updates**: Deduplicated by transaction `signature` and absolute path.
///
/// The filter can operate in two modes:
/// - **Global deduplication**: Removes duplicates across all datasources
/// - **Datasource-specific deduplication**: Only removes duplicates within specified datasources
///
/// # Examples
///
/// Global deduplication across all datasources:
/// ```
/// use carbon_core::filter::DedupFilter;
///
/// let filter = DedupFilter::new();
/// ```
///
/// Deduplication for specific datasources:
/// ```
/// use carbon_core::{datasource::DatasourceId, filter::DedupFilter};
///
/// let datasource_ids = vec![
///     DatasourceId::new_named("mainnet"),
///     DatasourceId::new_named("backup"),
/// ];
/// let filter = DedupFilter::new_for_datasources(datasource_ids);
/// ```
///
/// Using with pipeline builders:
/// ```
/// use carbon_core::{datasource::DatasourceId, filter::DedupFilter};
///
/// let filter = DedupFilter::new();
/// let filters = vec![Arc::new(filter) as Arc<dyn Filter>];
///
/// // Use with pipeline builder
/// // pipeline.account_with_filters(decoder, processor, filters);
/// ```
#[derive(Clone, Default)]
pub struct DedupFilter {
    /// The current slot being tracked for deduplication.
    ///
    /// When a new slot is encountered, the deduplication state is cleared
    /// and this value is updated to track the new slot.
    current_slot: Arc<Mutex<Option<u64>>>,

    /// Internal state tracking which updates have been seen in the current slot.
    dedup_state: Arc<Mutex<DedupState>>,

    /// Optional list of datasource IDs that should participate in deduplication.
    ///
    /// If `None`, all datasources are deduplicated together. If `Some(vec)`,
    /// only updates from the specified datasources will be deduplicated, while
    /// updates from other datasources will always pass through the filter.
    allowed_datasources: Option<Vec<DatasourceId>>,
}

impl DedupFilter {
    /// Creates a new deduplication filter for specific datasources.
    ///
    /// This filter will only deduplicate updates that come from the specified
    /// datasources. Updates from other datasources will pass through without
    /// deduplication. This is useful when you have multiple datasources but
    /// only want to deduplicate within a subset of them.
    ///
    /// # Arguments
    ///
    /// * `datasource_ids` - A vector of datasource IDs that should participate
    ///   in deduplication
    ///
    /// # Returns
    ///
    /// A new `DedupFilter` configured for datasource-specific deduplication.
    ///
    /// # Examples
    ///
    /// ```
    /// use carbon_core::{datasource::DatasourceId, filter::DedupFilter};
    ///
    /// let datasource_ids = vec![
    ///     DatasourceId::new_named("mainnet"),
    ///     DatasourceId::new_named("backup"),
    /// ];
    /// let filter = DedupFilter::new_for_datasources(datasource_ids);
    ///
    /// // Use with pipeline builder
    /// let filters = vec![Arc::new(filter) as Arcs<dyn Filter>];
    /// ```
    pub fn new_for_datasources(datasource_ids: Vec<DatasourceId>) -> Self {
        Self {
            current_slot: Arc::new(Mutex::new(None)),
            dedup_state: Arc::new(Mutex::new(DedupState::default())),
            allowed_datasources: Some(datasource_ids),
        }
    }

    /// Checks if the given datasource should participate in deduplication.
    ///
    /// Returns `true` if the datasource should be deduplicated based on the
    /// filter's configuration, or `false` if updates from this datasource
    /// should pass through without deduplication checks.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource to check
    ///
    /// # Returns
    ///
    /// `true` if the datasource participates in deduplication, `false` otherwise.
    fn should_deduplicate_datasource(&self, datasource_id: &DatasourceId) -> bool {
        match &self.allowed_datasources {
            None => true, // Deduplicate all datasources
            Some(allowed) => allowed.contains(datasource_id),
        }
    }

    /// Updates the current slot and clears deduplication state if needed.
    ///
    /// This method checks if the provided slot is different from the currently
    /// tracked slot. If it is, the deduplication state is cleared and the
    /// current slot is updated. This automatic cleanup ensures memory usage
    /// remains bounded and aligns with Solana's slot-based block structure.
    ///
    /// # Arguments
    ///
    /// * `new_slot` - The slot number of the current update
    ///
    /// # Thread Safety
    ///
    /// This method acquires locks on both the current slot and deduplication
    /// state. The locks are held briefly to minimize contention.
    fn update_slot_if_needed(&self, new_slot: u64) -> CarbonResult<()> {
        let mut current_slot = self
            .current_slot
            .lock()
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;

        if current_slot.map_or(true, |slot| slot != new_slot) {
            *current_slot = Some(new_slot);
            let mut state = self
                .dedup_state
                .lock()
                .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
            *state = DedupState::default();
        }

        Ok(())
    }
}

/// Internal state for tracking deduplicated updates within a single slot.
///
/// This structure maintains separate collections for each update type,
/// allowing efficient O(1) lookup and insertion operations.
#[derive(Debug, Default)]
struct DedupState {
    /// Tracks per-datasource monotonic counter for account updates.
    ///
    /// Key: (datasource_id, pubkey)
    /// Value: counter (increments with each update from this datasource for this pubkey)
    ///
    /// Each datasource maintains its own independent counter per pubkey.
    datasource_account_counters: HashMap<(DatasourceId, Pubkey), u64>,

    /// Tracks the highest counter value emitted for each account.
    ///
    /// Key: pubkey
    /// Value: highest counter that has been emitted
    ///
    /// An account update is only emitted if its counter is strictly greater than
    /// the value stored here.
    last_emitted_account: HashMap<Pubkey, u64>,

    /// Tracks instruction updates by transaction signature and absolute path of the instruction.
    instructions: HashSet<(Signature, Vec<u8>)>,
}

impl Filter for DedupFilter {
    /// Filters account updates based on deduplication state using per-datasource counters.
    ///
    /// This method uses a counter-based approach to handle deduplication across multiple
    /// datasources. Each datasource maintains its own monotonically increasing counter
    /// per pubkey within a slot. An update is only emitted if its counter is strictly
    /// greater than the last emitted counter for that pubkey.
    /// When multiple datasources send identical updates,
    /// only the first arrival is emitted since subsequent arrivals will have the same
    /// or lower counter values.
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `account_metadata` - Metadata about the account update (pubkey and slot)
    /// * `_account` - The account data (unused in counter-based deduplication)
    ///
    /// # Returns
    ///
    /// `true` if the account update should be processed (has a higher counter than
    /// last emission or from a non-deduplicated datasource), `false` if it should
    /// be skipped as a duplicate.
    fn filter_account(
        &self,
        datasource_id: &DatasourceId,
        account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> bool {
        if !self.should_deduplicate_datasource(datasource_id) {
            return true;
        }

        if let Err(e) = self.update_slot_if_needed(account_metadata.slot) {
            log::error!("Failed to update slot for account deduplication: {:?}", e);
            return false;
        }

        let mut state = match self.dedup_state.lock() {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to lock dedup state for account filtering: {:?}", e);
                return false;
            }
        };

        let counter_key = (datasource_id.clone(), account_metadata.pubkey);

        let counter = state
            .datasource_account_counters
            .entry(counter_key)
            .and_modify(|c| *c += 1)
            .or_insert(1);

        let current_counter = *counter;

        let last_emitted_counter = state
            .last_emitted_account
            .get(&account_metadata.pubkey)
            .copied()
            .unwrap_or(0);

        if current_counter <= last_emitted_counter {
            return false;
        }

        state
            .last_emitted_account
            .insert(account_metadata.pubkey, current_counter);
        true
    }

    /// Filters instruction updates based on deduplication state.
    ///
    /// This method checks if a instruction has already been processed within
    /// the current slot. Instructions are identified by their tx signature and absolute path (within the txs).
    ///
    /// # Arguments
    ///
    /// * `datasource_id` - The ID of the datasource that produced this update
    /// * `nested_instruction` - The instruction containing transaction metadata
    ///
    /// # Returns
    ///
    /// `true` if the instruction should be processed (not a duplicate or from
    /// a non-deduplicated datasource), `false` if it should be skipped as a duplicate.
    fn filter_instruction(
        &self,
        datasource_id: &DatasourceId,
        nested_instruction: &crate::instruction::NestedInstruction,
    ) -> bool {
        if !self.should_deduplicate_datasource(datasource_id) {
            return true;
        }

        if let Err(e) =
            self.update_slot_if_needed(nested_instruction.metadata.transaction_metadata.slot)
        {
            log::error!("Failed to update slot: {:?}", e);
            return false;
        }

        let key = (
            nested_instruction.metadata.transaction_metadata.signature,
            nested_instruction.metadata.absolute_path.clone(),
        );
        if let Ok(mut state) = self.dedup_state.lock() {
            state.instructions.insert(key)
        } else {
            log::error!("Failed to lock dedup state");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datasource::DatasourceId;
    use solana_account::Account;

    use solana_pubkey::Pubkey;

    fn create_account() -> Account {
        Account {
            lamports: 1000,
            data: vec![1, 2, 3],
            owner: Pubkey::default(),
            executable: false,
            rent_epoch: 0,
        }
    }

    fn create_metadata(slot: u64, pubkey: Pubkey) -> AccountMetadata {
        AccountMetadata {
            slot,
            pubkey,
            transaction_signature: None,
        }
    }

    #[test]
    fn test_simple_duplicate_from_two_datasources() {
        let filter = DedupFilter::default();
        let ds1 = DatasourceId::new_named("source1");
        let ds2 = DatasourceId::new_named("source2");
        let pubkey = Pubkey::new_unique();
        let account = create_account();
        let metadata = create_metadata(100, pubkey);

        // First update from source1 should pass
        assert!(filter.filter_account(&ds1, &metadata, &account));

        // Same update from source2 should be filtered
        assert!(!filter.filter_account(&ds2, &metadata, &account));
    }

    #[test]
    fn test_state_cycling_123_321_123() {
        let filter = DedupFilter::default();
        let ds1 = DatasourceId::new_named("source1");
        let ds2 = DatasourceId::new_named("source2");
        let pubkey = Pubkey::new_unique();
        let metadata = create_metadata(100, pubkey);

        let account_123 = Account {
            lamports: 1000,
            data: vec![1, 2, 3],
            owner: Pubkey::default(),
            executable: false,
            rent_epoch: 0,
        };

        let account_321 = Account {
            lamports: 1000,
            data: vec![3, 2, 1],
            owner: Pubkey::default(),
            executable: false,
            rent_epoch: 0,
        };

        // Update A (123) from source1
        assert!(filter.filter_account(&ds1, &metadata, &account_123));
        // Update A (123) from source2 - duplicate
        assert!(!filter.filter_account(&ds2, &metadata, &account_123));

        // Update B (321) from source1
        assert!(filter.filter_account(&ds1, &metadata, &account_321));
        // Update B (321) from source2 - duplicate
        assert!(!filter.filter_account(&ds2, &metadata, &account_321));

        // Update C (123) from source1 - cycles back but should pass
        assert!(filter.filter_account(&ds1, &metadata, &account_123));
        // Update C (123) from source2 - duplicate
        assert!(!filter.filter_account(&ds2, &metadata, &account_123));
    }

    #[test]
    fn test_slot_boundary_clears_state() {
        let filter = DedupFilter::default();
        let ds1 = DatasourceId::new_named("source1");
        let ds2 = DatasourceId::new_named("source2");
        let pubkey = Pubkey::new_unique();
        let account = create_account();

        // Slot 100
        let metadata_100 = create_metadata(100, pubkey);
        assert!(filter.filter_account(&ds1, &metadata_100, &account));
        assert!(!filter.filter_account(&ds2, &metadata_100, &account));

        // Slot 101 - state should reset
        let metadata_101 = create_metadata(101, pubkey);
        assert!(filter.filter_account(&ds1, &metadata_101, &account));
        assert!(!filter.filter_account(&ds2, &metadata_101, &account));
    }

    #[test]
    fn test_multiple_pubkeys_independent_counters() {
        let filter = DedupFilter::default();
        let ds1 = DatasourceId::new_named("source1");
        let ds2 = DatasourceId::new_named("source2");
        let pubkey1 = Pubkey::new_unique();
        let pubkey2 = Pubkey::new_unique();
        let account = create_account();
        let metadata1 = create_metadata(100, pubkey1);
        let metadata2 = create_metadata(100, pubkey2);

        // Both pubkeys should have independent deduplication
        assert!(filter.filter_account(&ds1, &metadata1, &account));
        assert!(filter.filter_account(&ds1, &metadata2, &account));
        assert!(!filter.filter_account(&ds2, &metadata1, &account));
        assert!(!filter.filter_account(&ds2, &metadata2, &account));
    }

    #[test]
    fn test_datasource_filter_only_deduplicates_allowed() {
        let ds1 = DatasourceId::new_named("source1");
        let ds2 = DatasourceId::new_named("source2");
        let ds3 = DatasourceId::new_named("source3");

        // Only deduplicate ds1 and ds2, not ds3
        let filter = DedupFilter::new_for_datasources(vec![ds1.clone(), ds2.clone()]);

        let pubkey = Pubkey::new_unique();
        let account = create_account();
        let metadata = create_metadata(100, pubkey);

        // ds1 passes
        assert!(filter.filter_account(&ds1, &metadata, &account));
        // ds2 filtered (duplicate of ds1)
        assert!(!filter.filter_account(&ds2, &metadata, &account));
        // ds3 always passes (not in dedup list)
        assert!(filter.filter_account(&ds3, &metadata, &account));
        // ds3 again still passes (no dedup for ds3)
        assert!(filter.filter_account(&ds3, &metadata, &account));
    }

    #[test]
    fn test_three_datasources_interleaved() {
        let filter = DedupFilter::default();
        let ds1 = DatasourceId::new_named("source1");
        let ds2 = DatasourceId::new_named("source2");
        let ds3 = DatasourceId::new_named("source3");
        let pubkey = Pubkey::new_unique();
        let metadata = create_metadata(100, pubkey);

        let account_a = Account {
            lamports: 1000,
            data: vec![1],
            owner: Pubkey::default(),
            executable: false,
            rent_epoch: 0,
        };

        let account_b = Account {
            lamports: 2000,
            data: vec![2],
            owner: Pubkey::default(),
            executable: false,
            rent_epoch: 0,
        };

        // Interleaved: ds1(A), ds2(A), ds1(B), ds3(A), ds2(B), ds3(B)
        assert!(filter.filter_account(&ds1, &metadata, &account_a)); // counter=1, emit
        assert!(!filter.filter_account(&ds2, &metadata, &account_a)); // counter=1, skip
        assert!(filter.filter_account(&ds1, &metadata, &account_b)); // counter=2, emit
        assert!(!filter.filter_account(&ds3, &metadata, &account_a)); // counter=1, skip
        assert!(!filter.filter_account(&ds2, &metadata, &account_b)); // counter=2, skip
        assert!(!filter.filter_account(&ds3, &metadata, &account_b)); // counter=2, skip
    }
}
