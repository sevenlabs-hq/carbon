//! Transaction positions and shared progress.

use {
    solana_clock::Slot,
    std::sync::{Arc, Mutex, PoisonError},
};

/// A transaction's position in its block.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionCursor {
    slot: Slot,
    transaction_index: u64,
}

impl TransactionCursor {
    pub const fn new(slot: Slot, transaction_index: u64) -> Self {
        Self {
            slot,
            transaction_index,
        }
    }

    pub const fn slot(self) -> Slot {
        self.slot
    }

    pub const fn transaction_index(self) -> u64 {
        self.transaction_index
    }
}

/// Shared in-memory progress. Clones share the same position.
#[derive(Clone, Debug)]
pub struct TransactionCursorHandle {
    position: Arc<Mutex<Option<TransactionCursor>>>,
}

impl TransactionCursorHandle {
    pub fn new(position: Option<TransactionCursor>) -> Self {
        Self {
            position: Arc::new(Mutex::new(position)),
        }
    }

    /// Reads the current position.
    pub fn get(&self) -> Option<TransactionCursor> {
        *self.position.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Replaces the current position.
    pub fn set(&self, position: TransactionCursor) {
        *self.position.lock().unwrap_or_else(PoisonError::into_inner) = Some(position);
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::thread};

    #[test]
    fn cursor_preserves_position() {
        let cursor = TransactionCursor::new(7, 3);
        assert_eq!(cursor.slot(), 7);
        assert_eq!(cursor.transaction_index(), 3);
    }

    #[test]
    fn cursor_orders_by_slot_then_index() {
        assert!(TransactionCursor::new(7, 3) < TransactionCursor::new(7, 4));
        assert!(TransactionCursor::new(7, u64::MAX) < TransactionCursor::new(8, 0));
        assert_eq!(TransactionCursor::new(7, 3), TransactionCursor::new(7, 3));
    }

    #[test]
    fn empty_handle_accepts_first_position() {
        let handle = TransactionCursorHandle::new(None);
        assert_eq!(handle.get(), None);
        handle.set(TransactionCursor::new(0, 0));
        assert_eq!(handle.get(), Some(TransactionCursor::new(0, 0)));
    }

    #[test]
    fn clones_share_unconditional_replacements() {
        let initial = TransactionCursor::new(7, 3);
        let handle = TransactionCursorHandle::new(Some(initial));
        let clone = handle.clone();
        assert_eq!(clone.get(), Some(initial));
        clone.set(initial);
        assert_eq!(handle.get(), Some(initial));
        clone.set(TransactionCursor::new(7, 2));
        assert_eq!(handle.get(), Some(TransactionCursor::new(7, 2)));
        clone.set(TransactionCursor::new(6, u64::MAX));
        assert_eq!(handle.get(), Some(TransactionCursor::new(6, u64::MAX)));
        handle.set(TransactionCursor::new(8, 0));
        assert_eq!(clone.get(), Some(TransactionCursor::new(8, 0)));
    }

    #[test]
    fn concurrent_reads_observe_complete_positions() {
        let handle = TransactionCursorHandle::new(None);
        thread::scope(|scope| {
            for offset in 0..4 {
                let handle = handle.clone();
                scope.spawn(move || {
                    for slot in (offset..1000).step_by(4) {
                        handle.set(TransactionCursor::new(slot, 1000 - slot));
                        let current = handle.get().unwrap();
                        assert_eq!(current.transaction_index(), 1000 - current.slot());
                    }
                });
            }
        });
        let current = handle.get().unwrap();
        assert_eq!(current.transaction_index(), 1000 - current.slot());
    }
}
