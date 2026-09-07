//! Completion of an admitted update.

use tokio::sync::oneshot;

#[derive(Debug)]
#[must_use = "await processing before advancing a recovery cursor"]
pub struct UpdateReceipt {
    receiver: oneshot::Receiver<Result<(), UpdateReceiptError>>,
}

impl UpdateReceipt {
    // Admission will create these pairs when source queues are implemented.
    #[cfg_attr(not(test), expect(dead_code))]
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
