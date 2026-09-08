//! Datasource execution and update emission.

pub mod queue;
pub mod receipt;
mod shutdown;

pub use {
    queue::{DatasourceOptions, OverflowPolicy},
    receipt::UpdateGroup,
    shutdown::ShutdownSignal,
};

use {
    self::{
        queue::QueuedUpdate,
        receipt::{ReceiptGroup, UpdateReceipt, UpdateReceiptError, UpdateReceiptSender},
    },
    crate::{error::BoxError, id::Id, update::Update},
    std::{future::Future, pin::Pin},
    tokio::sync::mpsc::{self, error::TrySendError},
};

/// Produces updates through its context.
pub trait Datasource: Send + 'static {
    fn run(self, context: DatasourceContext) -> impl Future<Output = Result<(), BoxError>> + Send;
}

pub(crate) trait DynDatasource: Send {
    fn run(
        self: Box<Self>,
        context: DatasourceContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), BoxError>> + Send>>;
}

impl<D: Datasource> DynDatasource for D {
    fn run(
        self: Box<Self>,
        context: DatasourceContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), BoxError>> + Send>> {
        Box::pin(Datasource::run(*self, context))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum EmitError {
    #[error("datasource queue is full")]
    QueueFull,
    #[error("pipeline is shutting down")]
    ShuttingDown,
    #[error("pipeline can no longer accept updates")]
    PipelineStopped,
}

/// One datasource's update sender and shutdown signal.
pub struct DatasourceContext {
    pipeline_id: Id,
    datasource_id: Id,
    sender: mpsc::Sender<QueuedUpdate>,
    overflow_policy: OverflowPolicy,
    shutdown: ShutdownSignal,
}

impl DatasourceContext {
    pub(crate) fn new(
        pipeline_id: Id,
        datasource_id: Id,
        sender: mpsc::Sender<QueuedUpdate>,
        overflow_policy: OverflowPolicy,
        shutdown: ShutdownSignal,
    ) -> Self {
        Self {
            pipeline_id,
            datasource_id,
            sender,
            overflow_policy,
            shutdown,
        }
    }

    pub fn pipeline_id(&self) -> &Id {
        &self.pipeline_id
    }

    pub fn datasource_id(&self) -> &Id {
        &self.datasource_id
    }

    pub fn shutdown(&self) -> &ShutdownSignal {
        &self.shutdown
    }

    /// Enqueues an update. Returns `None` when overflow policy `Drop` discards it.
    pub async fn emit(&mut self, update: Update) -> Result<Option<UpdateReceipt>, EmitError> {
        self.emit_inner(update, None).await
    }

    pub fn begin_group(&mut self) -> UpdateGroup<'_> {
        UpdateGroup::new(self)
    }

    async fn emit_inner(
        &mut self,
        update: Update,
        group: Option<&mut ReceiptGroup>,
    ) -> Result<Option<UpdateReceipt>, EmitError> {
        if self.shutdown.is_requested() {
            return Err(EmitError::ShuttingDown);
        }

        let permit = match self.sender.try_reserve() {
            Ok(permit) => permit,
            Err(TrySendError::Closed(())) => return Err(EmitError::PipelineStopped),
            Err(TrySendError::Full(())) => match self.overflow_policy {
                OverflowPolicy::Wait => {
                    tokio::select! {
                        biased;
                        _ = self.shutdown.requested() => return Err(EmitError::ShuttingDown),
                        result = self.sender.reserve() => result.map_err(|_| EmitError::PipelineStopped)?,
                    }
                }
                OverflowPolicy::Drop => {
                    if let Some(group) = group {
                        group.record_error(UpdateReceiptError::Dropped);
                    }
                    return Ok(None);
                }
                OverflowPolicy::Exit => return Err(EmitError::QueueFull),
            },
        };

        if let Some(group) = group {
            permit.send(QueuedUpdate {
                update,
                receipt_sender: UpdateReceiptSender::Group(group.member()),
            });
            return Ok(None);
        }

        let (sender, receipt) = UpdateReceipt::new();
        permit.send(QueuedUpdate {
            update,
            receipt_sender: UpdateReceiptSender::Single(sender),
        });
        Ok(Some(receipt))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::update::BlockUpdate,
        std::task::{Context, Waker},
        tokio_util::sync::CancellationToken,
    };

    fn context(
        capacity: usize,
        overflow_policy: OverflowPolicy,
    ) -> (DatasourceContext, mpsc::Receiver<QueuedUpdate>) {
        let (sender, receiver) = DatasourceOptions::default()
            .queue_capacity(capacity)
            .channel()
            .unwrap();
        (
            DatasourceContext::new(
                Id::new("pipeline").unwrap(),
                Id::new("source").unwrap(),
                sender,
                overflow_policy,
                ShutdownSignal::new(CancellationToken::new()),
            ),
            receiver,
        )
    }

    #[tokio::test]
    async fn receiving_frees_capacity_without_completing_the_receipt() {
        let (mut context, mut receiver) = context(2, OverflowPolicy::Drop);
        let first = context
            .emit(BlockUpdate::new(1).into())
            .await
            .unwrap()
            .unwrap();
        let second = context
            .emit(BlockUpdate::new(2).into())
            .await
            .unwrap()
            .unwrap();
        let processing = receiver.recv().await.unwrap();
        assert!(matches!(processing.update, Update::Block(ref block) if block.slot() == 1));
        let third = context
            .emit(BlockUpdate::new(3).into())
            .await
            .unwrap()
            .unwrap();

        let first = first.processed();
        tokio::pin!(first);
        assert!(first
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending());
        processing.receipt_sender.send(Ok(()));
        first.await.unwrap();

        for (slot, receipt) in [(2, second), (3, third)] {
            let queued = receiver.recv().await.unwrap();
            assert!(matches!(queued.update, Update::Block(block) if block.slot() == slot));
            queued.receipt_sender.send(Ok(()));
            receipt.processed().await.unwrap();
        }
    }

    #[tokio::test]
    async fn grouped_updates_can_be_processed_before_sealing() {
        let (mut context, mut receiver) = context(1, OverflowPolicy::Wait);
        let mut group = context.begin_group();
        group.emit(BlockUpdate::new(1).into()).await.unwrap();
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        group.emit(BlockUpdate::new(2).into()).await.unwrap();
        let receipt = group.seal().processed();
        tokio::pin!(receipt);
        assert!(receipt
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending());
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        receipt.await.unwrap();

        let individual = context
            .emit(BlockUpdate::new(3).into())
            .await
            .unwrap()
            .unwrap();
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        individual.processed().await.unwrap();
    }

    #[tokio::test]
    async fn an_all_dropped_group_does_not_succeed() {
        let (mut context, mut receiver) = context(1, OverflowPolicy::Drop);
        let individual = context
            .emit(BlockUpdate::new(1).into())
            .await
            .unwrap()
            .unwrap();
        let mut group = context.begin_group();
        group.emit(BlockUpdate::new(2).into()).await.unwrap();
        assert_eq!(
            group.seal().processed().await,
            Err(UpdateReceiptError::Dropped)
        );
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        individual.processed().await.unwrap();
    }

    #[tokio::test]
    async fn group_overflow_error_waits_for_accepted_members() {
        let (mut context, mut receiver) = context(1, OverflowPolicy::Exit);
        let mut group = context.begin_group();
        group.emit(BlockUpdate::new(1).into()).await.unwrap();
        assert_eq!(
            group.emit(BlockUpdate::new(2).into()).await,
            Err(EmitError::QueueFull)
        );
        let receipt = group.seal().processed();
        tokio::pin!(receipt);
        assert!(receipt
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending());
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        assert_eq!(receipt.await, Err(UpdateReceiptError::Failed));
    }

    #[tokio::test]
    async fn group_emission_errors_prevent_success() {
        let (mut source, receiver) = context(1, OverflowPolicy::Wait);
        let mut group = source.begin_group();
        drop(receiver);
        assert_eq!(
            group.emit(BlockUpdate::new(1).into()).await,
            Err(EmitError::PipelineStopped)
        );
        assert_eq!(
            group.seal().processed().await,
            Err(UpdateReceiptError::Aborted)
        );

        let (mut source, _receiver) = context(1, OverflowPolicy::Wait);
        let token = CancellationToken::new();
        source.shutdown = ShutdownSignal::new(token.clone());
        let mut group = source.begin_group();
        token.cancel();
        assert_eq!(
            group.emit(BlockUpdate::new(1).into()).await,
            Err(EmitError::ShuttingDown)
        );
        assert_eq!(
            group.seal().processed().await,
            Err(UpdateReceiptError::Dropped)
        );
    }

    #[tokio::test]
    async fn empty_groups_and_abandonment_release_the_context() {
        let (mut context, mut receiver) = context(1, OverflowPolicy::Wait);
        context.begin_group().seal().processed().await.unwrap();
        let mut group = context.begin_group();
        group.emit(BlockUpdate::new(1).into()).await.unwrap();
        group.abandon();
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        let individual = context
            .emit(BlockUpdate::new(2).into())
            .await
            .unwrap()
            .unwrap();
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        individual.processed().await.unwrap();
    }

    #[tokio::test]
    async fn full_queue_applies_drop_or_exit_without_removing_existing_work() {
        for policy in [OverflowPolicy::Drop, OverflowPolicy::Exit] {
            let (mut context, mut receiver) = context(1, policy);
            let receipt = context
                .emit(BlockUpdate::new(1).into())
                .await
                .unwrap()
                .unwrap();
            let result = context.emit(BlockUpdate::new(2).into()).await;
            if policy == OverflowPolicy::Drop {
                assert!(result.unwrap().is_none());
            } else {
                assert_eq!(result.unwrap_err(), EmitError::QueueFull);
            }
            assert_eq!(receiver.len(), 1);
            let queued = receiver.recv().await.unwrap();
            assert!(matches!(queued.update, Update::Block(block) if block.slot() == 1));
            queued.receipt_sender.send(Ok(()));
            receipt.processed().await.unwrap();
        }
    }

    #[tokio::test]
    async fn closed_queue_rejects_every_policy_but_preserves_queued_work() {
        for policy in [
            OverflowPolicy::Wait,
            OverflowPolicy::Drop,
            OverflowPolicy::Exit,
        ] {
            let (mut context, mut receiver) = context(1, policy);
            let receipt = context
                .emit(BlockUpdate::new(1).into())
                .await
                .unwrap()
                .unwrap();
            receiver.close();
            assert_eq!(
                context.emit(BlockUpdate::new(2).into()).await.unwrap_err(),
                EmitError::PipelineStopped,
            );
            receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
            receipt.processed().await.unwrap();
            assert!(receiver.recv().await.is_none());
        }
    }

    #[tokio::test]
    async fn wait_admits_only_after_capacity_is_available() {
        let (mut context, mut receiver) = context(1, OverflowPolicy::Wait);
        let first = context
            .emit(BlockUpdate::new(1).into())
            .await
            .unwrap()
            .unwrap();
        let waiting = context.emit(BlockUpdate::new(2).into());
        tokio::pin!(waiting);
        assert!(waiting
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending());
        receiver.recv().await.unwrap().receipt_sender.send(Ok(()));
        let second = waiting.await.unwrap().unwrap();
        let queued = receiver.recv().await.unwrap();
        assert!(matches!(queued.update, Update::Block(block) if block.slot() == 2));
        queued.receipt_sender.send(Ok(()));
        first.processed().await.unwrap();
        second.processed().await.unwrap();
    }
}
