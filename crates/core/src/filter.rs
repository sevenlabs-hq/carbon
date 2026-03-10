use crate::{
    account::AccountMetadata,
    datasource::{AccountDeletion, BlockDetails, DatasourceId},
    instruction::{NestedInstruction, NestedInstructions},
    transaction::TransactionMetadata,
};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterContext<'a> {
    pub datasource_id: &'a DatasourceId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterResult {
    Accept,
    Reject,
}

pub trait Filter: Send + Sync {
    fn filter_account(
        &self,
        _context: &FilterContext,
        _account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> FilterResult {
        FilterResult::Accept
    }

    fn filter_instruction(
        &self,
        _context: &FilterContext,
        _nested_instruction: &NestedInstruction,
    ) -> FilterResult {
        FilterResult::Accept
    }

    fn filter_transaction(
        &self,
        _context: &FilterContext,
        _transaction_metadata: &TransactionMetadata,
        _nested_instructions: &NestedInstructions,
    ) -> FilterResult {
        FilterResult::Accept
    }

    fn filter_account_deletion(
        &self,
        _context: &FilterContext,
        _account_deletion: &AccountDeletion,
    ) -> FilterResult {
        FilterResult::Accept
    }

    fn filter_block_details(
        &self,
        _context: &FilterContext,
        _block_details: &BlockDetails,
    ) -> FilterResult {
        FilterResult::Accept
    }
}

const DEDUP_CLEANUP_INTERVAL_SECS: u64 = 60;

type SeenInstructionsMap = HashMap<(Signature, Vec<u8>), Instant>;
type SeenAccountsMap = HashMap<(Signature, Pubkey), Instant>;

type SeenInstructions = HashMap<(Signature, Vec<u8>), Instant>;

pub struct DeduplicationFilter {
    seen_instructions: Arc<RwLock<SeenInstructionsMap>>,
    seen_accounts: Arc<RwLock<SeenAccountsMap>>,
    ttl: Duration,
    creation: Instant,
    last_cleanup_secs: AtomicU64,
}

impl DeduplicationFilter {
    pub fn new(ttl: Duration) -> Self {
        let creation = Instant::now();
        Self {
            seen_instructions: Arc::new(RwLock::new(HashMap::new())),
            seen_accounts: Arc::new(RwLock::new(HashMap::new())),
            ttl,
            creation,
            last_cleanup_secs: AtomicU64::new(0),
        }
    }

    pub fn cleanup_expired(&self) {
        let cutoff = Instant::now() - self.ttl;
        if let Ok(mut seen) = self.seen_instructions.write() {
            seen.retain(|_, t| *t > cutoff);
        }
        if let Ok(mut seen) = self.seen_accounts.write() {
            seen.retain(|_, t| *t > cutoff);
        }
        self.last_cleanup_secs
            .store(self.creation.elapsed().as_secs(), Ordering::Relaxed);
    }

    #[inline(always)]
    fn cleanup_if_needed(&self) {
        let elapsed_secs = self.creation.elapsed().as_secs();
        let last = self.last_cleanup_secs.load(Ordering::Relaxed);
        if elapsed_secs.saturating_sub(last) >= DEDUP_CLEANUP_INTERVAL_SECS {
            self.cleanup_expired();
        }
    }
}

impl Filter for DeduplicationFilter {
    #[inline(always)]
    fn filter_instruction(
        &self,
        _context: &FilterContext,
        nested_instruction: &NestedInstruction,
    ) -> FilterResult {
        self.cleanup_if_needed();
        let sig = nested_instruction.metadata.transaction_metadata.signature;
        let path = nested_instruction.metadata.absolute_path.clone();
        let key = (sig, path);
        let Ok(mut seen) = self.seen_instructions.write() else {
            return FilterResult::Accept;
        };
        if seen.insert(key, Instant::now()).is_none() {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    #[inline(always)]
    fn filter_account(
        &self,
        _context: &FilterContext,
        account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> FilterResult {
        self.cleanup_if_needed();
        let Some(tx_sig) = account_metadata.transaction_signature else {
            return FilterResult::Accept;
        };
        let key = (tx_sig, account_metadata.pubkey);
        let Ok(mut seen) = self.seen_accounts.write() else {
            return FilterResult::Accept;
        };
        if seen.insert(key, Instant::now()).is_none() {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }
}

pub struct DatasourceFilter {
    pub allowed_datasources: Vec<DatasourceId>,
}

impl DatasourceFilter {
    pub fn new(datasource_id: DatasourceId) -> Self {
        Self {
            allowed_datasources: vec![datasource_id],
        }
    }

    pub fn new_many(datasource_ids: Vec<DatasourceId>) -> Self {
        Self {
            allowed_datasources: datasource_ids,
        }
    }

    fn allows(&self, datasource_id: &DatasourceId) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }
}

impl Filter for DatasourceFilter {
    fn filter_account(
        &self,
        context: &FilterContext,
        _account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> FilterResult {
        if self.allows(context.datasource_id) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_instruction(
        &self,
        context: &FilterContext,
        _nested_instruction: &NestedInstruction,
    ) -> FilterResult {
        if self.allows(context.datasource_id) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_transaction(
        &self,
        context: &FilterContext,
        _transaction_metadata: &TransactionMetadata,
        _nested_instructions: &NestedInstructions,
    ) -> FilterResult {
        if self.allows(context.datasource_id) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_account_deletion(
        &self,
        context: &FilterContext,
        _account_deletion: &AccountDeletion,
    ) -> FilterResult {
        if self.allows(context.datasource_id) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_block_details(
        &self,
        context: &FilterContext,
        _block_details: &BlockDetails,
    ) -> FilterResult {
        if self.allows(context.datasource_id) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }
}

#[derive(Debug, Clone)]
pub struct SlotRangeFilter {
    from_slot: Option<u64>,
    from_transaction_index: Option<u64>,
    to_slot: Option<u64>,
    to_transaction_index: Option<u64>,
}

impl SlotRangeFilter {
    pub fn from(slot: u64, transaction_index: Option<u64>) -> Self {
        Self {
            from_slot: Some(slot),
            from_transaction_index: transaction_index,
            to_slot: None,
            to_transaction_index: None,
        }
    }

    pub fn to(slot: u64, transaction_index: Option<u64>) -> Self {
        Self {
            from_slot: None,
            from_transaction_index: None,
            to_slot: Some(slot),
            to_transaction_index: transaction_index,
        }
    }

    pub fn between(
        from_slot: u64,
        from_transaction_index: Option<u64>,
        to_slot: u64,
        to_transaction_index: Option<u64>,
    ) -> Self {
        Self {
            from_slot: Some(from_slot),
            from_transaction_index,
            to_slot: Some(to_slot),
            to_transaction_index,
        }
    }

    #[inline(always)]
    pub fn contains(&self, slot: u64, index: Option<u64>) -> bool {
        if let Some(from) = self.from_slot {
            if slot < from {
                return false;
            }

            if slot == from {
                if let (Some(from_idx), Some(tx_idx)) = (self.from_transaction_index, index) {
                    if tx_idx < from_idx {
                        return false;
                    }
                }
            }
        }

        if let Some(to) = self.to_slot {
            if slot > to {
                return false;
            }

            if slot == to {
                if let (Some(to_idx), Some(tx_idx)) = (self.to_transaction_index, index) {
                    if tx_idx >= to_idx {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl Filter for SlotRangeFilter {
    fn filter_instruction(
        &self,
        _context: &FilterContext,
        instruction: &NestedInstruction,
    ) -> FilterResult {
        let slot = instruction.metadata.transaction_metadata.slot;
        let index = instruction.metadata.transaction_metadata.index;

        if self.contains(slot, index) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_account(
        &self,
        _context: &FilterContext,
        metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> FilterResult {
        if self.contains(metadata.slot, None) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_transaction(
        &self,
        _context: &FilterContext,
        metadata: &TransactionMetadata,
        _instructions: &NestedInstructions,
    ) -> FilterResult {
        if self.contains(metadata.slot, metadata.index) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_account_deletion(
        &self,
        _context: &FilterContext,
        deletion: &crate::datasource::AccountDeletion,
    ) -> FilterResult {
        if self.contains(deletion.slot, None) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }

    fn filter_block_details(
        &self,
        _context: &FilterContext,
        details: &crate::datasource::BlockDetails,
    ) -> FilterResult {
        if self.contains(details.slot, None) {
            FilterResult::Accept
        } else {
            FilterResult::Reject
        }
    }
}
