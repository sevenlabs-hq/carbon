//! Filters evaluated before decoding, with a commit hook after processing.

use {
    crate::{
        error::BoxError,
        id::Id,
        instruction::NestedInstruction,
        processor::ProcessorResult,
        route::RouteContext,
        update::{AccountClosureUpdate, AccountUpdate, BlockUpdate, TransactionUpdate},
    },
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::{
        collections::HashMap,
        future::Future,
        pin::Pin,
        time::{Duration, Instant},
    },
};

pub trait Filter<T>: Send
where
    T: Sync,
{
    fn filter(
        &mut self,
        context: &RouteContext<'_>,
        value: &T,
    ) -> impl Future<Output = Result<bool, BoxError>> + Send;

    fn commit(
        &mut self,
        _context: &RouteContext<'_>,
        _value: &T,
        _processor_result: &ProcessorResult,
    ) -> impl Future<Output = Result<(), BoxError>> + Send {
        std::future::ready(Ok(()))
    }
}

type FilterFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, BoxError>> + Send + 'a>>;

trait DynFilter<T>: Send {
    fn filter<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        value: &'a T,
    ) -> FilterFuture<'a, bool>;
    fn commit<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        value: &'a T,
        result: &'a ProcessorResult,
    ) -> FilterFuture<'a, ()>;
}

impl<T: Sync, F: Filter<T>> DynFilter<T> for F {
    fn filter<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        value: &'a T,
    ) -> FilterFuture<'a, bool> {
        Box::pin(Filter::filter(self, context, value))
    }

    fn commit<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        value: &'a T,
        result: &'a ProcessorResult,
    ) -> FilterFuture<'a, ()> {
        Box::pin(Filter::commit(self, context, value, result))
    }
}

/// An ordered list of filters for one route input type.
pub struct Filters<T> {
    filters: Vec<Box<dyn DynFilter<T>>>,
}

impl<T> Default for Filters<T> {
    fn default() -> Self {
        Self {
            filters: Vec::new(),
        }
    }
}

impl<T: Sync> Filters<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, filter: impl Filter<T> + 'static) {
        self.filters.push(Box::new(filter));
    }

    pub(crate) async fn filter(
        &mut self,
        context: &RouteContext<'_>,
        value: &T,
    ) -> Result<bool, BoxError> {
        for filter in &mut self.filters {
            if !filter.filter(context, value).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    // Called only after every filter accepted and a processor attempt is eligible.
    pub(crate) async fn commit(
        &mut self,
        context: &RouteContext<'_>,
        value: &T,
        result: &ProcessorResult,
    ) -> Result<(), BoxError> {
        let mut first_error = None;
        for filter in &mut self.filters {
            if let Err(error) = filter.commit(context, value, result).await {
                if first_error.is_none() {
                    first_error = Some(error);
                } else {
                    log::error!("additional filter commit failure: {error}");
                }
            }
        }
        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

/// Accepts only updates from the listed datasources.
pub struct DatasourceFilter {
    pub allowed_datasources: Vec<Id>,
}

impl DatasourceFilter {
    pub fn new(datasource_id: Id) -> Self {
        Self {
            allowed_datasources: vec![datasource_id],
        }
    }

    pub fn new_many(datasource_ids: Vec<Id>) -> Self {
        Self {
            allowed_datasources: datasource_ids,
        }
    }
}

impl<T: Sync> Filter<T> for DatasourceFilter {
    async fn filter(&mut self, context: &RouteContext<'_>, _value: &T) -> Result<bool, BoxError> {
        Ok(self.allowed_datasources.contains(context.datasource_id()))
    }
}

/// Drops instructions and accounts with recently committed keys.
pub struct DeduplicationFilter {
    seen_instructions: HashMap<(Signature, Vec<u8>), Instant>,
    seen_accounts: HashMap<(Signature, Pubkey), Instant>,
    ttl: Duration,
    last_cleanup: Instant,
}

impl DeduplicationFilter {
    pub fn new(ttl: Duration) -> Self {
        Self {
            seen_instructions: HashMap::new(),
            seen_accounts: HashMap::new(),
            ttl,
            last_cleanup: Instant::now(),
        }
    }

    pub fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.seen_instructions
            .retain(|_, time| now.duration_since(*time) < self.ttl);
        self.seen_accounts
            .retain(|_, time| now.duration_since(*time) < self.ttl);
        self.last_cleanup = now;
    }

    fn cleanup_if_needed(&mut self) {
        if self.last_cleanup.elapsed() >= Duration::from_secs(60) {
            self.cleanup_expired();
        }
    }
}

impl Filter<NestedInstruction> for DeduplicationFilter {
    async fn filter(
        &mut self,
        _context: &RouteContext<'_>,
        value: &NestedInstruction,
    ) -> Result<bool, BoxError> {
        self.cleanup_if_needed();
        let key = (
            value.metadata.transaction_metadata.signature,
            value.metadata.absolute_path.clone(),
        );
        Ok(self
            .seen_instructions
            .get(&key)
            .is_none_or(|time| time.elapsed() >= self.ttl))
    }

    async fn commit(
        &mut self,
        _context: &RouteContext<'_>,
        value: &NestedInstruction,
        _result: &ProcessorResult,
    ) -> Result<(), BoxError> {
        let key = (
            value.metadata.transaction_metadata.signature,
            value.metadata.absolute_path.clone(),
        );
        self.seen_instructions.insert(key, Instant::now());
        Ok(())
    }
}

impl Filter<AccountUpdate> for DeduplicationFilter {
    async fn filter(
        &mut self,
        _context: &RouteContext<'_>,
        value: &AccountUpdate,
    ) -> Result<bool, BoxError> {
        self.cleanup_if_needed();
        let Some(signature) = value.transaction_signature() else {
            return Ok(true);
        };
        let key = (*signature, *value.pubkey());
        Ok(self
            .seen_accounts
            .get(&key)
            .is_none_or(|time| time.elapsed() >= self.ttl))
    }

    async fn commit(
        &mut self,
        _context: &RouteContext<'_>,
        value: &AccountUpdate,
        _result: &ProcessorResult,
    ) -> Result<(), BoxError> {
        if let Some(signature) = value.transaction_signature() {
            self.seen_accounts
                .insert((*signature, *value.pubkey()), Instant::now());
        }
        Ok(())
    }
}

/// Half-open `[from, to)` slot range filter with optional transaction-index
/// precision.
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

impl Filter<AccountUpdate> for SlotRangeFilter {
    async fn filter(
        &mut self,
        _context: &RouteContext<'_>,
        value: &AccountUpdate,
    ) -> Result<bool, BoxError> {
        Ok(self.contains(value.slot(), None))
    }
}

impl Filter<AccountClosureUpdate> for SlotRangeFilter {
    async fn filter(
        &mut self,
        _context: &RouteContext<'_>,
        value: &AccountClosureUpdate,
    ) -> Result<bool, BoxError> {
        Ok(self.contains(value.slot(), None))
    }
}

impl Filter<BlockUpdate> for SlotRangeFilter {
    async fn filter(
        &mut self,
        _context: &RouteContext<'_>,
        value: &BlockUpdate,
    ) -> Result<bool, BoxError> {
        Ok(self.contains(value.slot(), None))
    }
}

impl Filter<NestedInstruction> for SlotRangeFilter {
    async fn filter(
        &mut self,
        _context: &RouteContext<'_>,
        value: &NestedInstruction,
    ) -> Result<bool, BoxError> {
        Ok(self.contains(
            value.metadata.transaction_metadata.slot,
            value.metadata.transaction_metadata.index,
        ))
    }
}

impl Filter<TransactionUpdate> for SlotRangeFilter {
    async fn filter(
        &mut self,
        _context: &RouteContext<'_>,
        value: &TransactionUpdate,
    ) -> Result<bool, BoxError> {
        Ok(self.contains(value.slot(), value.index()))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        solana_account::Account,
        std::{
            cell::Cell,
            sync::{Arc, Mutex},
        },
    };

    #[derive(Debug, thiserror::Error)]
    #[error("rejected {0}")]
    struct Rejected(u8);

    type Events = Arc<Mutex<Vec<(u8, &'static str, usize)>>>;

    struct Observer {
        id: u8,
        calls: Cell<usize>,
        events: Events,
        accept: bool,
        fail_filter: bool,
        fail_commit: bool,
    }

    impl Filter<u8> for Observer {
        async fn filter(
            &mut self,
            context: &RouteContext<'_>,
            value: &u8,
        ) -> Result<bool, BoxError> {
            assert_eq!(context.pipeline_id().as_str(), "pipeline");
            assert_eq!(context.datasource_id().as_str(), "source");
            assert_eq!(context.route_id().as_str(), "route");
            assert_eq!(*value, 7);
            self.calls.set(self.calls.get() + 1);
            tokio::task::yield_now().await;
            self.events
                .lock()
                .unwrap()
                .push((self.id, "filter", self.calls.get()));
            if self.fail_filter {
                return Err(Box::new(Rejected(self.id)));
            }
            Ok(self.accept)
        }

        async fn commit(
            &mut self,
            _context: &RouteContext<'_>,
            _value: &u8,
            result: &ProcessorResult,
        ) -> Result<(), BoxError> {
            tokio::task::yield_now().await;
            let event = if let Err(error) = result {
                assert_eq!(error.downcast_ref::<Rejected>().unwrap().0, 9);
                "commit-error"
            } else {
                "commit"
            };
            self.events
                .lock()
                .unwrap()
                .push((self.id, event, self.calls.get()));
            if self.fail_commit {
                return Err(Box::new(Rejected(self.id)));
            }
            Ok(())
        }
    }

    fn observer(id: u8, events: &Events) -> Observer {
        Observer {
            id,
            events: events.clone(),
            calls: Cell::new(0),
            accept: true,
            fail_filter: false,
            fail_commit: false,
        }
    }

    fn ids() -> [Id; 3] {
        ["pipeline", "source", "route"].map(|id| Id::new(id).unwrap())
    }

    #[tokio::test]
    async fn mixed_filters_preserve_order_state_and_default_commit() {
        let ids = ids();
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut filters = Filters::new();
        filters.push(observer(1, &events));
        filters.push(DatasourceFilter::new(ids[1].clone()));
        filters.push(observer(2, &events));
        for _ in 0..2 {
            assert!(filters.filter(&context, &7).await.unwrap());
            filters.commit(&context, &7, &Ok(())).await.unwrap();
        }
        assert_eq!(
            *events.lock().unwrap(),
            vec![
                (1, "filter", 1),
                (2, "filter", 1),
                (1, "commit", 1),
                (2, "commit", 1),
                (1, "filter", 2),
                (2, "filter", 2),
                (1, "commit", 2),
                (2, "commit", 2),
            ]
        );
        // The concrete filter is not Sync, but both adapter futures are Send.
        fn assert_send<T: Send>(_: T) {}
        assert_send(filters.filter(&context, &7));
        assert_send(filters.commit(&context, &7, &Ok(())));
    }

    #[tokio::test]
    async fn rejection_and_errors_short_circuit_the_filter_list() {
        let ids = ids();
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        for fail in [false, true] {
            let events = Arc::new(Mutex::new(Vec::new()));
            let mut filters = Filters::new();
            let mut first = observer(1, &events);
            first.accept = false;
            first.fail_filter = fail;
            filters.push(first);
            filters.push(observer(2, &events));
            let result = filters.filter(&context, &7).await;
            if fail {
                assert_eq!(result.unwrap_err().downcast_ref::<Rejected>().unwrap().0, 1);
            } else {
                assert!(!result.unwrap());
            }
            assert_eq!(*events.lock().unwrap(), vec![(1, "filter", 1)]);
        }
    }

    #[tokio::test]
    async fn commits_receive_the_processor_error_and_continue_after_failure() {
        let ids = ids();
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut filters = Filters::new();
        for id in 1..=3 {
            let mut filter = observer(id, &events);
            filter.fail_commit = id != 2;
            filters.push(filter);
        }
        assert!(filters.filter(&context, &7).await.unwrap());
        let result = Err(Box::new(Rejected(9)) as BoxError);
        let error = filters.commit(&context, &7, &result).await.unwrap_err();
        assert_eq!(error.downcast_ref::<Rejected>().unwrap().0, 1);
        assert_eq!(
            *events.lock().unwrap(),
            vec![
                (1, "filter", 1),
                (2, "filter", 1),
                (3, "filter", 1),
                (1, "commit-error", 1),
                (2, "commit-error", 1),
                (3, "commit-error", 1),
            ]
        );
    }

    #[tokio::test]
    async fn empty_filters_accept_and_commit() {
        let ids = ids();
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let mut filters = Filters::<u8>::new();
        assert!(filters.filter(&context, &7).await.unwrap());
        filters.commit(&context, &7, &Ok(())).await.unwrap();
    }

    #[tokio::test]
    async fn deduplication_only_records_committed_accounts() {
        let ids = ids();
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let update = AccountUpdate::new(Pubkey::new_unique(), Account::default(), 1)
            .with_transaction_signature(Signature::default());
        let mut filters = Filters::new();
        filters.push(DeduplicationFilter::new(Duration::from_secs(60)));
        assert!(filters.filter(&context, &update).await.unwrap());
        assert!(filters.filter(&context, &update).await.unwrap());
        filters.commit(&context, &update, &Ok(())).await.unwrap();
        assert!(!filters.filter(&context, &update).await.unwrap());

        let mut expired = Filters::new();
        expired.push(DeduplicationFilter::new(Duration::ZERO));
        expired.commit(&context, &update, &Ok(())).await.unwrap();
        assert!(expired.filter(&context, &update).await.unwrap());
    }
}
