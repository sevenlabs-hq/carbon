use {super::Update, solana_clock::Slot, solana_hash::Hash, solana_transaction_status::Rewards};

/// Metadata for a block.
#[derive(Debug, Clone)]
pub struct BlockUpdate {
    slot: Slot,
    block_hash: Option<Hash>,
    previous_block_hash: Option<Hash>,
    rewards: Option<Rewards>,
    num_reward_partitions: Option<u64>,
    block_time: Option<i64>,
    block_height: Option<u64>,
}

impl BlockUpdate {
    pub fn new(slot: Slot) -> Self {
        Self {
            slot,
            block_hash: None,
            previous_block_hash: None,
            rewards: None,
            num_reward_partitions: None,
            block_time: None,
            block_height: None,
        }
    }

    pub fn with_block_hash(mut self, block_hash: Hash) -> Self {
        self.block_hash = Some(block_hash);
        self
    }

    pub fn with_previous_block_hash(mut self, previous_block_hash: Hash) -> Self {
        self.previous_block_hash = Some(previous_block_hash);
        self
    }

    pub fn with_rewards(mut self, rewards: Rewards) -> Self {
        self.rewards = Some(rewards);
        self
    }

    pub fn with_num_reward_partitions(mut self, count: u64) -> Self {
        self.num_reward_partitions = Some(count);
        self
    }

    pub fn with_block_time(mut self, block_time: i64) -> Self {
        self.block_time = Some(block_time);
        self
    }

    pub fn with_block_height(mut self, block_height: u64) -> Self {
        self.block_height = Some(block_height);
        self
    }

    pub fn slot(&self) -> Slot {
        self.slot
    }

    pub fn block_hash(&self) -> Option<&Hash> {
        self.block_hash.as_ref()
    }

    pub fn previous_block_hash(&self) -> Option<&Hash> {
        self.previous_block_hash.as_ref()
    }

    pub fn rewards(&self) -> Option<&Rewards> {
        self.rewards.as_ref()
    }

    pub fn num_reward_partitions(&self) -> Option<u64> {
        self.num_reward_partitions
    }

    pub fn block_time(&self) -> Option<i64> {
        self.block_time
    }

    pub fn block_height(&self) -> Option<u64> {
        self.block_height
    }
}

impl From<BlockUpdate> for Update {
    fn from(update: BlockUpdate) -> Self {
        Self::Block(update)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_defaults_to_unknown_optional_values() {
        let update = BlockUpdate::new(0);
        assert_eq!(update.slot(), 0);
        assert_eq!(update.block_hash(), None);
        assert_eq!(update.previous_block_hash(), None);
        assert_eq!(update.rewards(), None);
        assert_eq!(update.num_reward_partitions(), None);
        assert_eq!(update.block_time(), None);
        assert_eq!(update.block_height(), None);
    }

    #[test]
    fn block_preserves_optional_values_in_update() {
        let block_hash = Hash::new_from_array([1; 32]);
        let previous_block_hash = Hash::new_from_array([2; 32]);
        let update = BlockUpdate::new(7)
            .with_block_hash(block_hash)
            .with_previous_block_hash(previous_block_hash)
            .with_rewards(vec![])
            .with_num_reward_partitions(0)
            .with_block_time(-1)
            .with_block_height(0);

        let Update::Block(update) = Update::from(update) else {
            panic!("expected a block update");
        };
        assert_eq!(update.slot(), 7);
        assert_eq!(update.block_hash(), Some(&block_hash));
        assert_eq!(update.previous_block_hash(), Some(&previous_block_hash));
        assert_eq!(update.rewards(), Some(&vec![]));
        assert_eq!(update.num_reward_partitions(), Some(0));
        assert_eq!(update.block_time(), Some(-1));
        assert_eq!(update.block_height(), Some(0));
    }
}
