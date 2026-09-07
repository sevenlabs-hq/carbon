//! Updates emitted by datasources.

mod account;
mod account_closure;
mod block;
mod error;
mod transaction;

pub use {
    account::AccountUpdate, account_closure::AccountClosureUpdate, block::BlockUpdate,
    error::UpdateValidationError, transaction::TransactionUpdate,
};

/// A datasource update.
#[derive(Debug, Clone)]
pub enum Update {
    Account(AccountUpdate),
    AccountClosure(AccountClosureUpdate),
    Transaction(TransactionUpdate),
    Block(BlockUpdate),
}
