use crate::{
    account::AccountMetadata,
    datasource::{AccountDeletion, BlockDetails, DatasourceId},
    instruction::{NestedInstruction, NestedInstructions},
    transaction::TransactionMetadata,
};

pub trait Filter {
    fn filter_account(
        &self,
        _datasource_id: &DatasourceId,
        _account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> bool {
        true
    }

    fn filter_instruction(
        &self,
        _datasource_id: &DatasourceId,
        _nested_instruction: &NestedInstruction,
    ) -> bool {
        true
    }

    fn filter_transaction(
        &self,
        _datasource_id: &DatasourceId,
        _transaction_metadata: &TransactionMetadata,
        _nested_instructions: &NestedInstructions,
    ) -> bool {
        true
    }

    fn filter_account_deletion(
        &self,
        _datasource_id: &DatasourceId,
        _account_deletion: &AccountDeletion,
    ) -> bool {
        true
    }

    fn filter_block_details(
        &self,
        _datasource_id: &DatasourceId,
        _block_details: &BlockDetails,
    ) -> bool {
        true
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
}

impl Filter for DatasourceFilter {
    fn filter_account(
        &self,
        datasource_id: &DatasourceId,
        _account_metadata: &AccountMetadata,
        _account: &solana_account::Account,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    fn filter_instruction(
        &self,
        datasource_id: &DatasourceId,
        _nested_instruction: &NestedInstruction,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    fn filter_transaction(
        &self,
        datasource_id: &DatasourceId,
        _transaction_metadata: &TransactionMetadata,
        _nested_instructions: &NestedInstructions,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    fn filter_account_deletion(
        &self,
        datasource_id: &DatasourceId,
        _account_deletion: &AccountDeletion,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }

    fn filter_block_details(
        &self,
        datasource_id: &DatasourceId,
        _block_details: &BlockDetails,
    ) -> bool {
        self.allowed_datasources.contains(datasource_id)
    }
}
