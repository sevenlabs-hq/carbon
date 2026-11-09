//! Raises SIGINT in its own process, so it lives in a test binary of its own.
#![cfg(unix)]

use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, SlotStatus, SlotStatusUpdate, Update, UpdateType},
        error::CarbonResult,
        pipeline::{Pipeline, ShutdownStrategy},
        processor::Processor,
    },
    std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::Duration,
    },
    tokio::sync::{mpsc::Sender, Notify},
    tokio_util::sync::CancellationToken,
};

struct QueuedSlots(u64);

#[async_trait]
impl Datasource for QueuedSlots {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        for slot in 0..self.0 {
            let update = Update::SlotStatus(SlotStatusUpdate {
                slot,
                bank_id: None,
                status: SlotStatus::Processed,
                parent: None,
            });
            let _ = sender.send((update, id.clone())).await;
        }
        cancellation_token.cancelled().await;
        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::SlotStatus]
    }
}

struct SlowProcessor {
    started: Arc<Notify>,
    processed: Arc<AtomicUsize>,
}

impl Processor<SlotStatusUpdate> for SlowProcessor {
    async fn process(&mut self, _update: &SlotStatusUpdate) -> CarbonResult<()> {
        self.started.notify_one();
        tokio::time::sleep(Duration::from_millis(100)).await;
        self.processed.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

#[tokio::test]
async fn ctrl_c_during_processing_stops_the_pipeline() {
    let started = Arc::new(Notify::new());
    let processed = Arc::new(AtomicUsize::new(0));
    let cancellation_token = CancellationToken::new();
    let mut pipeline = Pipeline::builder()
        .datasource(QueuedSlots(100))
        .slot_status(SlowProcessor {
            started: Arc::clone(&started),
            processed: Arc::clone(&processed),
        })
        .datasource_cancellation_token(cancellation_token.clone())
        .shutdown_strategy(ShutdownStrategy::Immediate)
        .build()
        .unwrap();

    tokio::spawn(async move {
        started.notified().await;
        unsafe { libc::raise(libc::SIGINT) };
    });

    tokio::time::timeout(Duration::from_secs(2), pipeline.run())
        .await
        .expect("the pipeline should stop on Ctrl-C")
        .unwrap();
    assert!(cancellation_token.is_cancelled());
    assert_eq!(processed.load(Ordering::SeqCst), 1);
}
