//! Per-source channel configuration.

use {
    super::receipt::UpdateReceiptSender,
    crate::update::Update,
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
    queue_capacity: usize,
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

    // Datasource registration will own the sender/receiver pair.
    #[cfg_attr(not(test), expect(dead_code))]
    pub(crate) fn channel(
        &self,
    ) -> Option<(mpsc::Sender<QueuedUpdate>, mpsc::Receiver<QueuedUpdate>)> {
        if self.queue_capacity == 0 || self.queue_capacity > Semaphore::MAX_PERMITS {
            return None;
        }
        Some(mpsc::channel(self.queue_capacity))
    }
}

#[cfg_attr(not(test), expect(dead_code))]
pub(crate) struct QueuedUpdate {
    pub(crate) update: Update,
    pub(crate) receipt_sender: UpdateReceiptSender,
}

/// Reserves capacity before creating a receipt. None means overflow Drop.
/// The context must coordinate the final permission check and send with shutdown.
#[cfg_attr(not(test), expect(dead_code))]
pub(crate) async fn reserve(
    sender: &mpsc::Sender<QueuedUpdate>,
    policy: OverflowPolicy,
) -> Result<Option<mpsc::Permit<'_, QueuedUpdate>>, mpsc::error::TrySendError<()>> {
    match policy {
        OverflowPolicy::Wait => match sender.reserve().await {
            Ok(permit) => Ok(Some(permit)),
            Err(_) => Err(mpsc::error::TrySendError::Closed(())),
        },
        OverflowPolicy::Exit => sender.try_reserve().map(Some),
        OverflowPolicy::Drop => match sender.try_reserve() {
            Ok(permit) => Ok(Some(permit)),
            Err(mpsc::error::TrySendError::Full(())) => Ok(None),
            Err(error) => Err(error),
        },
    }
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
            task::{Context, Poll, Waker},
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

    async fn admit(
        sender: &mpsc::Sender<QueuedUpdate>,
        policy: OverflowPolicy,
        slot: u64,
    ) -> Result<Option<UpdateReceipt>, mpsc::error::TrySendError<()>> {
        let Some(permit) = reserve(sender, policy).await? else {
            return Ok(None);
        };
        let (entry, receipt) = entry(slot);
        permit.send(entry);
        Ok(Some(receipt))
    }

    fn slot(entry: &QueuedUpdate) -> u64 {
        let Update::Block(update) = &entry.update else {
            panic!("expected block update");
        };
        update.slot()
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
    async fn receiving_frees_capacity_without_completing_the_receipt() {
        let (sender, mut receiver) = channel(2);
        let first = admit(&sender, OverflowPolicy::Wait, 1)
            .await
            .unwrap()
            .unwrap();
        let second = admit(&sender, OverflowPolicy::Wait, 2)
            .await
            .unwrap()
            .unwrap();
        let processing = receiver.recv().await.unwrap();
        assert_eq!(slot(&processing), 1);
        let third = admit(&sender, OverflowPolicy::Drop, 3)
            .await
            .unwrap()
            .unwrap();

        let first = first.processed();
        tokio::pin!(first);
        assert_eq!(
            first.as_mut().poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        );
        processing.receipt_sender.send(Ok(()));
        first.await.unwrap();

        for (expected_slot, receipt) in [(2, second), (3, third)] {
            let queued = receiver.recv().await.unwrap();
            assert_eq!(slot(&queued), expected_slot);
            queued.receipt_sender.send(Ok(()));
            receipt.processed().await.unwrap();
        }
    }

    #[tokio::test]
    async fn full_and_closed_channels_map_to_the_overflow_policy() {
        let (sender, mut receiver) = channel(1);
        let receipt = admit(&sender, OverflowPolicy::Wait, 1)
            .await
            .unwrap()
            .unwrap();
        assert!(admit(&sender, OverflowPolicy::Drop, 2)
            .await
            .unwrap()
            .is_none());
        assert_eq!(
            admit(&sender, OverflowPolicy::Exit, 2).await.unwrap_err(),
            mpsc::error::TrySendError::Full(())
        );

        receiver.close();
        for policy in [
            OverflowPolicy::Wait,
            OverflowPolicy::Drop,
            OverflowPolicy::Exit,
        ] {
            assert_eq!(
                admit(&sender, policy, 2).await.unwrap_err(),
                mpsc::error::TrySendError::Closed(())
            );
        }
        let queued = receiver.recv().await.unwrap();
        assert_eq!(slot(&queued), 1);
        queued.receipt_sender.send(Ok(()));
        receipt.processed().await.unwrap();
        assert!(receiver.recv().await.is_none());
    }

    #[tokio::test]
    async fn wait_admits_only_after_capacity_is_available() {
        let (sender, mut receiver) = channel(1);
        let first = admit(&sender, OverflowPolicy::Wait, 1)
            .await
            .unwrap()
            .unwrap();
        let waiting = admit(&sender, OverflowPolicy::Wait, 2);
        tokio::pin!(waiting);
        assert!(waiting
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending());
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        let second = waiting.await.unwrap().unwrap();
        let queued = receiver.recv().await.unwrap();
        assert_eq!(slot(&queued), 2);
        queued.receipt_sender.send(Ok(()));
        first.processed().await.unwrap();
        second.processed().await.unwrap();
    }

    #[tokio::test]
    async fn grouped_entries_use_the_same_channel() {
        let (sender, mut receiver) = channel(2);
        let mut group = ReceiptGroup::new();
        for slot in 0..2 {
            let permit = reserve(&sender, OverflowPolicy::Wait)
                .await
                .unwrap()
                .unwrap();
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
        let individual = admit(&sender, OverflowPolicy::Wait, 1)
            .await
            .unwrap()
            .unwrap();
        let mut group = ReceiptGroup::new();
        let permit = reserve(&sender, OverflowPolicy::Wait)
            .await
            .unwrap()
            .unwrap();
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

    // This is why the context needs a final admission check coordinated with shutdown.
    #[tokio::test]
    async fn closing_the_receiver_does_not_revoke_existing_permits() {
        let (sender, mut receiver) = channel(1);
        let permit = reserve(&sender, OverflowPolicy::Wait)
            .await
            .unwrap()
            .unwrap();
        receiver.close();
        let (queued, receipt) = entry(1);
        permit.send(queued);
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        receipt.processed().await.unwrap();
        assert!(receiver.recv().await.is_none());
    }
}
