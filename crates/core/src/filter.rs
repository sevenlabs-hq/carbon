use crate::{
    account::AccountMetadata,
    datasource::{AccountDeletion, BlockDetails, DatasourceId, UpdateType},
    instruction::{NestedInstruction, NestedInstructions},
    transaction::TransactionMetadata,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterContext<'a> {
    pub datasource_id: &'a DatasourceId,
    pub update_type: UpdateType,
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
