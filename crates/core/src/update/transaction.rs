use {
    super::{Update, UpdateValidationError},
    solana_clock::Slot,
    solana_hash::Hash,
    solana_signature::Signature,
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_status::TransactionStatusMeta,
};

/// A transaction and its runtime execution metadata.
#[derive(Debug, Clone)]
pub struct TransactionUpdate {
    transaction: VersionedTransaction,
    meta: TransactionStatusMeta,
    is_vote: Option<bool>,
    slot: Slot,
    index: Option<u64>,
    block_time: Option<i64>,
    block_hash: Option<Hash>,
}

impl TransactionUpdate {
    /// Creates a transaction update without cryptographic verification or sanitization.
    ///
    /// `meta` must describe the transaction's runtime execution.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateValidationError::MissingTransactionSignature`] if there are no signatures.
    pub fn new(
        transaction: VersionedTransaction,
        meta: TransactionStatusMeta,
        slot: Slot,
    ) -> Result<Self, UpdateValidationError> {
        if transaction.signatures.is_empty() {
            return Err(UpdateValidationError::MissingTransactionSignature);
        }

        Ok(Self {
            transaction,
            meta,
            is_vote: None,
            slot,
            index: None,
            block_time: None,
            block_hash: None,
        })
    }

    pub fn with_is_vote(mut self, is_vote: bool) -> Self {
        self.is_vote = Some(is_vote);
        self
    }

    /// Sets the transaction's position in its block, not in a filtered response.
    pub fn with_index(mut self, index: u64) -> Self {
        self.index = Some(index);
        self
    }

    pub fn with_block_time(mut self, block_time: i64) -> Self {
        self.block_time = Some(block_time);
        self
    }

    pub fn with_block_hash(mut self, block_hash: Hash) -> Self {
        self.block_hash = Some(block_hash);
        self
    }

    pub fn signature(&self) -> &Signature {
        // Construction guarantees a first signature; the transaction is immutable.
        &self.transaction.signatures[0]
    }

    pub fn transaction(&self) -> &VersionedTransaction {
        &self.transaction
    }

    pub fn meta(&self) -> &TransactionStatusMeta {
        &self.meta
    }

    pub fn is_vote(&self) -> Option<bool> {
        self.is_vote
    }

    pub fn slot(&self) -> Slot {
        self.slot
    }

    pub fn index(&self) -> Option<u64> {
        self.index
    }

    pub fn block_time(&self) -> Option<i64> {
        self.block_time
    }

    pub fn block_hash(&self) -> Option<&Hash> {
        self.block_hash.as_ref()
    }
}

impl From<TransactionUpdate> for Update {
    fn from(update: TransactionUpdate) -> Self {
        Self::Transaction(update)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transaction() -> VersionedTransaction {
        VersionedTransaction {
            signatures: vec![Signature::from([1; 64]), Signature::from([2; 64])],
            ..VersionedTransaction::default()
        }
    }

    #[test]
    fn transaction_requires_a_primary_signature() {
        assert_eq!(
            TransactionUpdate::new(
                VersionedTransaction::default(),
                TransactionStatusMeta::default(),
                7,
            )
            .unwrap_err(),
            UpdateValidationError::MissingTransactionSignature,
        );
    }

    #[test]
    fn transaction_preserves_payload_and_borrows_primary_signature() {
        let transaction = transaction();
        let meta = TransactionStatusMeta {
            fee: 42,
            log_messages: Some(vec!["runtime log".to_owned()]),
            ..TransactionStatusMeta::default()
        };
        let update = TransactionUpdate::new(transaction.clone(), meta, 7).unwrap();

        assert_eq!(update.transaction(), &transaction);
        assert_eq!(update.signature(), &transaction.signatures[0]);
        assert!(std::ptr::eq(
            update.signature(),
            &update.transaction().signatures[0]
        ));
        assert_eq!(update.meta().fee, 42);
        assert_eq!(
            update.meta().log_messages.as_deref(),
            Some(["runtime log".to_owned()].as_slice())
        );
        assert_eq!(update.slot(), 7);
        assert_eq!(update.is_vote(), None);
        assert_eq!(update.index(), None);
        assert_eq!(update.block_time(), None);
        assert_eq!(update.block_hash(), None);
    }

    #[test]
    fn transaction_preserves_optional_values_in_update() {
        let block_hash = Hash::new_from_array([1; 32]);
        let update = TransactionUpdate::new(transaction(), TransactionStatusMeta::default(), 7)
            .unwrap()
            .with_is_vote(true)
            .with_is_vote(false)
            .with_index(0)
            .with_block_time(-1)
            .with_block_hash(block_hash);

        let Update::Transaction(update) = Update::from(update) else {
            panic!("expected a transaction update");
        };
        assert_eq!(update.is_vote(), Some(false));
        assert_eq!(update.index(), Some(0));
        assert_eq!(update.block_time(), Some(-1));
        assert_eq!(update.block_hash(), Some(&block_hash));
    }
}
