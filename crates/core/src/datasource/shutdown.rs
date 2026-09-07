//! Read-only shutdown notification for datasources.

use tokio_util::sync::CancellationToken;

/// Observes shutdown requests. Clones observe the same request.
#[derive(Clone, Debug)]
pub struct ShutdownSignal {
    token: CancellationToken,
}

impl ShutdownSignal {
    #[cfg_attr(not(test), expect(dead_code))]
    pub(crate) fn new(token: CancellationToken) -> Self {
        Self { token }
    }

    /// Returns whether shutdown has been requested.
    pub fn is_requested(&self) -> bool {
        self.token.is_cancelled()
    }

    /// Waits for shutdown, returning immediately if already requested.
    pub async fn requested(&self) {
        self.token.cancelled().await;
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
        let token = CancellationToken::new();
        let signal = ShutdownSignal::new(token.clone());
        let clone = signal.clone();
        assert!(!signal.is_requested());
        assert!(!clone.is_requested());

        token.cancel();

        assert!(signal.is_requested());
        assert!(clone.is_requested());
    }

    #[tokio::test]
    async fn an_existing_request_completes_every_wait() {
        let token = CancellationToken::new();
        token.cancel();
        let signal = ShutdownSignal::new(token);

        signal.requested().await;
        signal.requested().await;
        signal.clone().requested().await;
    }

    #[test]
    fn a_request_wakes_pending_waiters() {
        struct WakeFlag(AtomicBool);

        impl Wake for WakeFlag {
            fn wake(self: Arc<Self>) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let token = CancellationToken::new();
        let signal = ShutdownSignal::new(token.clone());
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

        token.cancel();

        assert!(first_wake.0.load(Ordering::SeqCst));
        assert!(second_wake.0.load(Ordering::SeqCst));
        assert!(first.as_mut().poll(&mut first_context).is_ready());
        assert!(second.as_mut().poll(&mut second_context).is_ready());
    }
}
