//! Per-source channels and update selection.

use {
    super::receipt::UpdateReceiptSender,
    crate::{id::Id, update::Update},
    std::{future::poll_fn, task::Poll},
    tokio::sync::{mpsc, Semaphore},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverflowPolicy {
    #[default]
    Wait,
    Exit,
    Drop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DatasourceOptions {
    pub(crate) queue_capacity: usize,
    pub(crate) overflow_policy: OverflowPolicy,
}

impl Default for DatasourceOptions {
    fn default() -> Self {
        Self {
            queue_capacity: 1_000,
            overflow_policy: OverflowPolicy::Wait,
        }
    }
}

impl DatasourceOptions {
    /// Sets the maximum number of queued updates, excluding processing work.
    pub fn queue_capacity(mut self, capacity: usize) -> Self {
        self.queue_capacity = capacity;
        self
    }

    pub fn overflow_policy(mut self, policy: OverflowPolicy) -> Self {
        self.overflow_policy = policy;
        self
    }

    pub(crate) fn capacity_is_valid(&self) -> bool {
        self.queue_capacity > 0 && self.queue_capacity <= Semaphore::MAX_PERMITS
    }

    pub(crate) fn channel(
        &self,
    ) -> Option<(mpsc::Sender<QueuedUpdate>, mpsc::Receiver<QueuedUpdate>)> {
        if !self.capacity_is_valid() {
            return None;
        }
        Some(mpsc::channel(self.queue_capacity))
    }
}

pub(crate) struct QueuedUpdate {
    pub(crate) update: Update,
    pub(crate) receipt_sender: UpdateReceiptSender,
}

pub(crate) async fn next_update(
    queues: &mut [(Id, mpsc::Receiver<QueuedUpdate>)],
    next: &mut usize,
) -> Option<(Id, QueuedUpdate)> {
    poll_fn(|cx| {
        let mut pending = false;
        for offset in 0..queues.len() {
            let index = (*next + offset) % queues.len();
            let (id, receiver) = &mut queues[index];
            match receiver.poll_recv(cx) {
                Poll::Ready(Some(update)) => {
                    let id = id.clone();
                    *next = (index + 1) % queues.len();
                    return Poll::Ready(Some((id, update)));
                }
                Poll::Ready(None) => {}
                Poll::Pending => pending = true,
            }
        }
        if pending {
            return Poll::Pending;
        }
        Poll::Ready(None)
    })
    .await
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            datasource::receipt::{ReceiptGroup, UpdateReceipt, UpdateReceiptError},
            update::BlockUpdate,
        },
        std::{
            future::Future,
            sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            },
            task::{Context, Wake, Waker},
        },
    };

    fn channel(capacity: usize) -> (mpsc::Sender<QueuedUpdate>, mpsc::Receiver<QueuedUpdate>) {
        DatasourceOptions::default()
            .queue_capacity(capacity)
            .channel()
            .unwrap()
    }

    fn entry(slot: u64) -> (QueuedUpdate, UpdateReceipt) {
        let (sender, receipt) = UpdateReceipt::new();
        (
            QueuedUpdate {
                update: BlockUpdate::new(slot).into(),
                receipt_sender: UpdateReceiptSender::Single(sender),
            },
            receipt,
        )
    }

    #[test]
    fn defaults_and_capacity_validation() {
        let options = DatasourceOptions::default();
        let (_, receiver) = options.channel().unwrap();
        assert_eq!(receiver.max_capacity(), 1_000);
        assert_eq!(options.overflow_policy, OverflowPolicy::Wait);
        assert_eq!(
            options
                .overflow_policy(OverflowPolicy::Exit)
                .overflow_policy,
            OverflowPolicy::Exit
        );
        assert!(options.queue_capacity(0).channel().is_none());
        assert!(options.queue_capacity(usize::MAX).channel().is_none());
    }

    #[tokio::test]
    async fn selection_preserves_fifo_and_rotates_between_sources() {
        let mut queues = Vec::new();
        for source in 0..3 {
            let (sender, receiver) = channel(2);
            for slot in [source, source + 3] {
                sender.try_reserve().unwrap().send(entry(slot).0);
            }
            queues.push((Id::new(source.to_string()).unwrap(), receiver));
        }

        let mut next = 0;
        for slot in 0..6 {
            let (id, queued) = next_update(&mut queues, &mut next).await.unwrap();
            assert_eq!(id, Id::new((slot % 3).to_string()).unwrap());
            assert!(matches!(queued.update, Update::Block(block) if block.slot() == slot));
        }
        assert!(next_update(&mut queues, &mut next).await.is_none());
        assert!(next_update(&mut [], &mut next).await.is_none());
    }

    #[test]
    fn empty_queues_register_wakeups_and_closed_queues_are_skipped() {
        struct WakeFlag(AtomicBool);

        impl Wake for WakeFlag {
            fn wake(self: Arc<Self>) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let flag = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(flag.clone());
        let mut cx = Context::from_waker(&waker);
        let mut queues = Vec::new();
        let mut senders = Vec::new();
        for source in 0..3 {
            let (sender, receiver) = channel(1);
            queues.push((Id::new(source.to_string()).unwrap(), receiver));
            senders.push(sender);
        }
        queues[0].1.close();

        let mut next = 0;
        for source in [1, 2] {
            let mut read = std::pin::pin!(next_update(&mut queues, &mut next));
            assert!(read.as_mut().poll(&mut cx).is_pending());
            flag.0.store(false, Ordering::SeqCst);
            senders[source].try_reserve().unwrap().send(entry(1).0);
            assert!(flag.0.load(Ordering::SeqCst));
            let Poll::Ready(Some((id, _))) = read.as_mut().poll(&mut cx) else {
                panic!("the queued update was not returned");
            };
            assert_eq!(id, Id::new(source.to_string()).unwrap());
        }
    }

    #[tokio::test]
    async fn grouped_entries_use_the_same_channel() {
        let (sender, mut receiver) = channel(2);
        let mut group = ReceiptGroup::new();
        for slot in 0..2 {
            let permit = sender.reserve().await.unwrap();
            permit.send(QueuedUpdate {
                update: BlockUpdate::new(slot).into(),
                receipt_sender: UpdateReceiptSender::Group(group.member()),
            });
        }
        let receipt = group.seal().processed();
        tokio::pin!(receipt);
        receiver
            .recv()
            .await
            .unwrap()
            .receipt_sender
            .send(Err(UpdateReceiptError::Failed));
        assert!(receipt
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending());
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        assert_eq!(receipt.await, Err(UpdateReceiptError::Failed));
    }

    #[tokio::test]
    async fn receiver_loss_aborts_individual_and_grouped_updates() {
        let (sender, receiver) = channel(2);
        let (queued, individual) = entry(1);
        sender.try_reserve().unwrap().send(queued);
        let mut group = ReceiptGroup::new();
        let permit = sender.reserve().await.unwrap();
        permit.send(QueuedUpdate {
            update: BlockUpdate::new(2).into(),
            receipt_sender: UpdateReceiptSender::Group(group.member()),
        });
        let grouped = group.seal();
        drop(receiver);
        assert_eq!(
            individual.processed().await,
            Err(UpdateReceiptError::Aborted)
        );
        assert_eq!(grouped.processed().await, Err(UpdateReceiptError::Aborted));
    }

    // Draining a closed receiver must account for outstanding permits.
    #[tokio::test]
    async fn closing_the_receiver_does_not_revoke_existing_permits() {
        let (sender, mut receiver) = channel(1);
        let permit = sender.reserve().await.unwrap();
        receiver.close();
        let (queued, receipt) = entry(1);
        permit.send(queued);
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        receipt.processed().await.unwrap();
        assert!(receiver.recv().await.is_none());
    }
}
