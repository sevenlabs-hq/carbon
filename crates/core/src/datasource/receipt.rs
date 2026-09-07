//! Completion of an admitted update or group of updates.

use {
    std::sync::{Arc, Mutex},
    tokio::sync::oneshot,
};

#[derive(Debug)]
#[must_use = "await processing before advancing a recovery cursor"]
pub struct UpdateReceipt {
    receiver: oneshot::Receiver<Result<(), UpdateReceiptError>>,
}

impl UpdateReceipt {
    pub(crate) fn new() -> (oneshot::Sender<Result<(), UpdateReceiptError>>, Self) {
        let (sender, receiver) = oneshot::channel();
        (sender, Self { receiver })
    }

    /// Waits for completion under the configured route policies.
    pub async fn processed(self) -> Result<(), UpdateReceiptError> {
        self.receiver
            .await
            .unwrap_or(Err(UpdateReceiptError::Aborted))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum UpdateReceiptError {
    #[error("update processing failed")]
    Failed,
    #[error("unfinished update was dropped during shutdown")]
    Dropped,
    #[error("update processing was aborted")]
    Aborted,
}

// The datasource context will own this while a group is open.
pub(crate) struct ReceiptGroup {
    state: Arc<GroupState>,
    receipt: UpdateReceipt,
}

#[cfg_attr(not(test), expect(dead_code))]
impl ReceiptGroup {
    pub(crate) fn new() -> Self {
        let (sender, receipt) = UpdateReceipt::new();
        Self {
            state: Arc::new(GroupState {
                result: Mutex::new(Ok(())),
                sender: Some(sender),
            }),
            receipt,
        }
    }

    /// Registers one admitted update.
    pub(crate) fn member(&mut self) -> GroupMember {
        GroupMember {
            state: Some(self.state.clone()),
        }
    }

    /// Records incomplete admission, including overflow drops and emission errors.
    pub(crate) fn record_error(&mut self, error: UpdateReceiptError) {
        self.state.record_error(error);
    }

    pub(crate) fn seal(self) -> UpdateReceipt {
        drop(self.state);
        self.receipt
    }
}

pub(crate) struct GroupMember {
    state: Option<Arc<GroupState>>,
}

impl GroupMember {
    #[cfg_attr(not(test), expect(dead_code))]
    pub(crate) fn complete(mut self, result: Result<(), UpdateReceiptError>) {
        if let Some(state) = self.state.take() {
            if let Err(error) = result {
                state.record_error(error);
            }
        }
    }
}

impl Drop for GroupMember {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            state.record_error(UpdateReceiptError::Aborted);
        }
    }
}

struct GroupState {
    result: Mutex<Result<(), UpdateReceiptError>>,
    sender: Option<oneshot::Sender<Result<(), UpdateReceiptError>>>,
}

impl GroupState {
    fn record_error(&self, error: UpdateReceiptError) {
        use UpdateReceiptError::{Aborted, Dropped, Failed};

        let mut result = self
            .result
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        *result = Err(match (*result, error) {
            (Err(Failed), _) | (_, Failed) => Failed,
            (Err(Aborted), _) | (_, Aborted) => Aborted,
            _ => Dropped,
        });
    }
}

// The open group and each unfinished member hold one Arc. The last one sends
// the result, so completion needs no separate pending count or sealed flag.
impl Drop for GroupState {
    fn drop(&mut self) {
        let result = *self
            .result
            .get_mut()
            .unwrap_or_else(|error| error.into_inner());
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            future::{pending, Future},
            sync::{
                atomic::{AtomicUsize, Ordering},
                Arc,
            },
            task::{Context, Poll, Wake, Waker},
        },
    };

    const OUTCOMES: [Result<(), UpdateReceiptError>; 4] = [
        Ok(()),
        Err(UpdateReceiptError::Failed),
        Err(UpdateReceiptError::Dropped),
        Err(UpdateReceiptError::Aborted),
    ];

    #[tokio::test]
    async fn observes_results_sent_before_waiting() {
        for outcome in OUTCOMES {
            let (sender, receipt) = UpdateReceipt::new();
            sender.send(outcome).unwrap();
            assert_eq!(receipt.processed().await, outcome);
        }
    }

    #[derive(Default)]
    struct WakeCounter(AtomicUsize);

    impl Wake for WakeCounter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn settlement_wakes_a_pending_receipt() {
        for outcome in OUTCOMES {
            let (sender, receipt) = UpdateReceipt::new();
            let counter = Arc::new(WakeCounter::default());
            let waker = Waker::from(counter.clone());
            let mut context = Context::from_waker(&waker);
            let future = receipt.processed();
            tokio::pin!(future);

            assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
            sender.send(outcome).unwrap();
            assert_eq!(counter.0.load(Ordering::Relaxed), 1);
            assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(outcome));
        }
    }

    #[tokio::test]
    async fn losing_the_sender_aborts_the_receipt() {
        let (sender, receipt) = UpdateReceipt::new();
        drop(sender);
        assert_eq!(receipt.processed().await, Err(UpdateReceiptError::Aborted));
    }

    #[test]
    fn losing_the_sender_wakes_a_pending_receipt() {
        let (sender, receipt) = UpdateReceipt::new();
        let counter = Arc::new(WakeCounter::default());
        let waker = Waker::from(counter.clone());
        let mut context = Context::from_waker(&waker);
        let future = receipt.processed();
        tokio::pin!(future);

        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        drop(sender);
        assert_eq!(counter.0.load(Ordering::Relaxed), 1);
        assert_eq!(
            future.as_mut().poll(&mut context),
            Poll::Ready(Err(UpdateReceiptError::Aborted))
        );
    }

    #[tokio::test]
    async fn cancelling_the_sending_task_aborts_the_receipt() {
        let (sender, receipt) = UpdateReceipt::new();
        let task = tokio::spawn(async move {
            let _sender = sender;
            pending::<()>().await;
        });
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert_eq!(receipt.processed().await, Err(UpdateReceiptError::Aborted));
    }

    #[tokio::test]
    async fn dropping_the_receipt_does_not_cancel_the_sending_task() {
        let (sender, receipt) = UpdateReceipt::new();
        drop(receipt);
        let task = tokio::spawn(async move {
            tokio::task::yield_now().await;
            assert_eq!(sender.send(Ok(())), Err(Ok(())));
        });
        task.await.unwrap();
    }
}

#[cfg(test)]
mod group_tests {
    use {
        super::*,
        std::{
            future::{pending, Future},
            task::{Context, Poll, Waker},
        },
    };

    // In increasing precedence order.
    const OUTCOMES: [Result<(), UpdateReceiptError>; 4] = [
        Ok(()),
        Err(UpdateReceiptError::Dropped),
        Err(UpdateReceiptError::Aborted),
        Err(UpdateReceiptError::Failed),
    ];

    #[tokio::test]
    async fn empty_group_succeeds() {
        assert_eq!(ReceiptGroup::new().seal().processed().await, Ok(()));
    }

    #[tokio::test]
    async fn membership_remains_open_after_earlier_members_finish() {
        let mut group = ReceiptGroup::new();
        group.member().complete(Ok(()));
        group.member().complete(Err(UpdateReceiptError::Dropped));
        group.member().complete(Ok(()));
        assert_eq!(
            group.seal().processed().await,
            Err(UpdateReceiptError::Dropped)
        );
    }

    #[test]
    fn sealed_group_waits_for_every_member_even_after_failure() {
        let mut group = ReceiptGroup::new();
        let first = group.member();
        let second = group.member();
        let third = group.member();
        let receipt = group.seal().processed();
        tokio::pin!(receipt);
        let mut context = Context::from_waker(Waker::noop());

        assert_eq!(receipt.as_mut().poll(&mut context), Poll::Pending);
        first.complete(Err(UpdateReceiptError::Failed));
        assert_eq!(receipt.as_mut().poll(&mut context), Poll::Pending);
        second.complete(Ok(()));
        assert_eq!(receipt.as_mut().poll(&mut context), Poll::Pending);
        drop(third);
        assert_eq!(
            receipt.as_mut().poll(&mut context),
            Poll::Ready(Err(UpdateReceiptError::Failed))
        );
    }

    #[tokio::test]
    async fn result_precedence_does_not_depend_on_completion_order() {
        for (i, left) in OUTCOMES.into_iter().enumerate() {
            for (j, right) in OUTCOMES.into_iter().enumerate() {
                for reverse in [false, true] {
                    let mut group = ReceiptGroup::new();
                    let first = group.member();
                    let second = group.member();
                    let receipt = group.seal();
                    if reverse {
                        second.complete(right);
                        first.complete(left);
                    } else {
                        first.complete(left);
                        second.complete(right);
                    }
                    assert_eq!(receipt.processed().await, OUTCOMES[i.max(j)]);
                }
            }
        }
    }

    #[tokio::test]
    async fn admission_errors_contribute_without_creating_members() {
        for result in OUTCOMES.into_iter().skip(1) {
            let error = result.unwrap_err();
            let mut empty = ReceiptGroup::new();
            empty.record_error(error);
            assert_eq!(empty.seal().processed().await, result);

            let mut partial = ReceiptGroup::new();
            let member = partial.member();
            partial.record_error(error);
            let receipt = partial.seal();
            member.complete(Ok(()));
            assert_eq!(receipt.processed().await, result);
        }
    }

    #[tokio::test]
    async fn a_lost_member_aborts_the_group_before_or_after_sealing() {
        let mut group = ReceiptGroup::new();
        drop(group.member());
        assert_eq!(
            group.seal().processed().await,
            Err(UpdateReceiptError::Aborted)
        );

        let mut group = ReceiptGroup::new();
        let member = group.member();
        let receipt = group.seal();
        drop(member);
        assert_eq!(receipt.processed().await, Err(UpdateReceiptError::Aborted));
    }

    #[tokio::test]
    async fn cancelling_a_member_task_aborts_the_group() {
        let mut group = ReceiptGroup::new();
        let member = group.member();
        let receipt = group.seal();
        let task = tokio::spawn(async move {
            let _member = member;
            pending::<()>().await;
        });
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert_eq!(receipt.processed().await, Err(UpdateReceiptError::Aborted));
    }

    #[tokio::test]
    async fn dropping_the_group_or_receipt_does_not_cancel_members() {
        let mut group = ReceiptGroup::new();
        let member = group.member();
        drop(group);
        let task = tokio::spawn(async move {
            tokio::task::yield_now().await;
            member.complete(Ok(()));
        });
        task.await.unwrap();

        let mut group = ReceiptGroup::new();
        let member = group.member();
        drop(group.seal());
        member.complete(Err(UpdateReceiptError::Failed));
    }

    #[tokio::test]
    async fn concurrent_members_share_one_result() {
        let mut group = ReceiptGroup::new();
        let mut threads = Vec::new();
        for _ in 0..4 {
            let members: Vec<_> = (0..64).map(|_| group.member()).collect();
            threads.push(std::thread::spawn(move || {
                for (index, member) in members.into_iter().enumerate() {
                    std::thread::yield_now();
                    member.complete(OUTCOMES[index % OUTCOMES.len()]);
                }
            }));
        }
        assert_eq!(
            group.seal().processed().await,
            Err(UpdateReceiptError::Failed)
        );
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
