pub enum JetstreamerRange {
    /// First and last slot to stream, both inclusive.
    Slot(u64, u64),
    Epoch(u64),
}

impl JetstreamerRange {
    pub fn new_with_slots(start_slot: u64, end_slot: u64) -> Self {
        Self::Slot(start_slot, end_slot)
    }

    pub fn new_with_epoch(epoch: u64) -> Self {
        Self::Epoch(epoch)
    }

    /// Returns the first and last slot of the range, both inclusive.
    pub fn into_slots(&self) -> (u64, u64) {
        match self {
            Self::Slot(start_slot, end_slot) => (*start_slot, *end_slot),
            Self::Epoch(epoch) => jetstreamer_firehose::epochs::epoch_to_slot_range(*epoch),
        }
    }
}
