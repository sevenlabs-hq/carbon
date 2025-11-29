use std::collections::HashSet;

use solana_pubkey::Pubkey;

#[derive(Clone, Debug, Default)]
pub struct JetstreamerFilter {
    pub include_transactions: bool,
    pub include_blocks: bool,
    pub transaction_filters: Vec<TransactionFilter>,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionFilter {
    pub vote: Option<bool>,
    pub failed: Option<bool>,
    pub account_include: HashSet<Pubkey>,
    pub account_exclude: HashSet<Pubkey>,
    pub account_required: HashSet<Pubkey>,
}

impl TransactionFilter {
    pub fn new(
        vote: Option<bool>,
        failed: Option<bool>,
        account_include: HashSet<Pubkey>,
        account_exclude: HashSet<Pubkey>,
        account_required: HashSet<Pubkey>,
    ) -> Self {
        Self {
            vote,
            failed,
            account_include,
            account_exclude,
            account_required,
        }
    }

    pub fn matches(&self, accounts: &HashSet<Pubkey>, is_vote: bool, is_failed: bool) -> bool {
        if let Some(vote_filter) = self.vote {
            if is_vote != vote_filter {
                return false;
            }
        }

        if let Some(failed_filter) = self.failed {
            if is_failed != failed_filter {
                return false;
            }
        }

        if !self.account_include.is_empty() && self.account_include.is_disjoint(accounts) {
            return false;
        }

        if !self.account_exclude.is_disjoint(accounts) {
            return false;
        }

        if !self.account_required.is_subset(accounts) {
            return false;
        }

        true
    }
}
