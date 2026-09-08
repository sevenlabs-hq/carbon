//! Read-only shutdown notification for datasources.

use {crate::pipeline::ShutdownMode, tokio::sync::watch};

/// Observes shutdown requests. Clones observe the same request.
#[derive(Clone, Debug)]
pub struct ShutdownSignal {
    receiver: watch::Receiver<Option<ShutdownMode>>,
}

impl ShutdownSignal {
    pub(crate) fn new(receiver: watch::Receiver<Option<ShutdownMode>>) -> Self {
        Self { receiver }
    }

    pub(crate) fn mode(&self) -> Option<ShutdownMode> {
        *self.receiver.borrow()
    }

    /// Returns whether shutdown has been requested.
    pub fn is_requested(&self) -> bool {
        self.mode().is_some()
    }

    /// Waits for shutdown, returning immediately if already requested.
    pub async fn requested(&self) {
        let mut receiver = self.receiver.clone();
        if receiver.wait_for(Option::is_some).await.is_err() {
            // Losing the sender is not a shutdown request.
            std::future::pending::<()>().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            future::Future,
            pin::pin,
            sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            },
            task::{Context, Wake, Waker},
        },
    };

    #[test]
    fn clones_observe_the_same_request() {
        let (sender, receiver) = watch::channel(None);
        let signal = ShutdownSignal::new(receiver);
        let clone = signal.clone();
        assert!(!signal.is_requested());
        assert!(!clone.is_requested());

        sender.send_replace(Some(ShutdownMode::Drain));

        assert!(signal.is_requested());
        assert!(clone.is_requested());
        assert_eq!(signal.mode(), Some(ShutdownMode::Drain));

        sender.send_replace(Some(ShutdownMode::Drop));
        assert_eq!(signal.mode(), Some(ShutdownMode::Drop));
        assert_eq!(clone.mode(), Some(ShutdownMode::Drop));
    }

    #[tokio::test]
    async fn an_existing_request_completes_every_wait() {
        let (sender, receiver) = watch::channel(Some(ShutdownMode::Drain));
        let signal = ShutdownSignal::new(receiver);
        drop(sender);

        signal.requested().await;
        signal.requested().await;
        signal.clone().requested().await;
    }

    #[test]
    fn losing_the_sender_does_not_request_shutdown() {
        let (sender, receiver) = watch::channel(None);
        let signal = ShutdownSignal::new(receiver);
        let mut requested = pin!(signal.requested());
        let mut context = Context::from_waker(Waker::noop());
        assert!(requested.as_mut().poll(&mut context).is_pending());

        drop(sender);

        assert!(!signal.is_requested());
        assert!(requested.as_mut().poll(&mut context).is_pending());
    }

    #[test]
    fn a_request_wakes_pending_waiters() {
        struct WakeFlag(AtomicBool);

        impl Wake for WakeFlag {
            fn wake(self: Arc<Self>) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let (sender, receiver) = watch::channel(None);
        let signal = ShutdownSignal::new(receiver);
        let clone = signal.clone();
        let mut first = pin!(signal.requested());
        let mut second = pin!(clone.requested());
        let first_wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let second_wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let first_waker = Waker::from(first_wake.clone());
        let second_waker = Waker::from(second_wake.clone());
        let mut first_context = Context::from_waker(&first_waker);
        let mut second_context = Context::from_waker(&second_waker);
        assert!(first.as_mut().poll(&mut first_context).is_pending());
        assert!(second.as_mut().poll(&mut second_context).is_pending());

        sender.send_replace(Some(ShutdownMode::Drop));

        assert!(first_wake.0.load(Ordering::SeqCst));
        assert!(second_wake.0.load(Ordering::SeqCst));
        assert!(first.as_mut().poll(&mut first_context).is_ready());
        assert!(second.as_mut().poll(&mut second_context).is_ready());
    }
}
