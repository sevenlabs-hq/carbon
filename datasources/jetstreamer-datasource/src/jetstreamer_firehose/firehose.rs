#[cfg(feature = "geyser")]
use crossbeam_channel::{Receiver, Sender, unbounded};
use dashmap::{DashMap, DashSet};
use futures_util::future::BoxFuture;
#[cfg(feature = "geyser")]
use reqwest::{Client, Url};
use solana_accounts_db::stake_rewards::StakeRewardInfo;
use solana_address::Address;
#[cfg(feature = "geyser")]
use solana_geyser_plugin_manager::{
    block_metadata_notifier_interface::BlockMetadataNotifier,
    geyser_plugin_service::GeyserPluginServiceError,
};
use solana_hash::Hash;
#[cfg(feature = "geyser")]
use solana_ledger::entry_notifier_interface::EntryNotifier;
use solana_reward_info::RewardType;
#[cfg(feature = "geyser")]
use solana_rpc::{
    optimistically_confirmed_bank_tracker::SlotNotification,
    transaction_notifier_interface::TransactionNotifier,
};
use solana_runtime::bank::KeyedRewardsAndNumPartitions;
use solana_sdk_ids::vote::id as vote_program_id;
use solana_transaction::versioned::VersionedTransaction;
use std::{
    fmt::Display,
    io,
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    },
};
#[cfg(feature = "geyser")]
use std::{future::Future, path::PathBuf};
use thiserror::Error;
use tokio::{
    sync::{
        broadcast::{self, error::TryRecvError},
        mpsc, oneshot,
    },
    time::{sleep, timeout},
};

use crate::jetstreamer_firehose::{
    LOG_MODULE, SharedError,
    epochs::{
        FetchEpochStreamOptions, epoch_to_slot_range, fetch_epoch_stream,
        fetch_epoch_stream_with_options, slot_to_epoch,
    },
    index::{SLOT_OFFSET_INDEX, SlotOffsetIndexError},
    node_reader::{NodeReader, wait_for_request_slot},
    pace::pace,
    utils,
};

/// Timeout applied to each asynchronous firehose operation (fetching epoch stream, reading
/// header, seeking, reading next block). Adjust here to tune stall detection/restart
/// aggressiveness. Public so frontends (e.g. the TUI) can derive staleness thresholds from it.
pub const OP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);
const OP_TIMEOUT_SEQUENTIAL: std::time::Duration = std::time::Duration::from_secs(180);
// Epochs earlier than this were bincode-encoded in Old Faithful.
const BINCODE_EPOCH_CUTOFF: u64 = 157;

fn poll_shutdown(
    flag: &Arc<std::sync::atomic::AtomicBool>,
    receiver: &mut Option<broadcast::Receiver<()>>,
) -> bool {
    if let Some(rx) = receiver {
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Lagged(_)) => {
                flag.store(true, Ordering::SeqCst);
            }
            Err(TryRecvError::Closed) => {
                flag.store(true, Ordering::SeqCst);
            }
            Err(TryRecvError::Empty) => {}
        }
    }
    flag.load(Ordering::SeqCst)
}

fn is_shutdown_error(err: &FirehoseError) -> bool {
    fn is_interrupted(inner: &(dyn std::error::Error + 'static)) -> bool {
        inner
            .downcast_ref::<io::Error>()
            .map(|io_err| io_err.kind() == io::ErrorKind::Interrupted)
            .unwrap_or(false)
    }

    match err {
        FirehoseError::BlockHandlerError(inner)
        | FirehoseError::TransactionHandlerError(inner)
        | FirehoseError::EntryHandlerError(inner)
        | FirehoseError::RewardHandlerError(inner)
        | FirehoseError::OnStatsHandlerError(inner) => is_interrupted(inner.as_ref()),
        _ => false,
    }
}

/// Per-thread "data flowed" timestamps, stamped each time a firehose thread reads a full
/// block. Drives the health-gated staggered launch and is available to frontends.
pub mod thread_activity {
    use dashmap::{DashMap, DashSet};
    use once_cell::sync::Lazy;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Instant;

    static ORIGIN: Lazy<Instant> = Lazy::new(Instant::now);
    static LAST_ACTIVITY_MS: Lazy<DashMap<usize, u64, ahash::RandomState>> =
        Lazy::new(|| DashMap::with_hasher(ahash::RandomState::new()));
    static FINISHED: Lazy<DashSet<usize, ahash::RandomState>> =
        Lazy::new(|| DashSet::with_hasher(ahash::RandomState::new()));
    static TX_COUNTS: Lazy<DashMap<usize, u64, ahash::RandomState>> =
        Lazy::new(|| DashMap::with_hasher(ahash::RandomState::new()));
    static STREAM_START_MS: Lazy<DashMap<usize, u64, ahash::RandomState>> =
        Lazy::new(|| DashMap::with_hasher(ahash::RandomState::new()));
    static RECYCLE_REQUESTED: Lazy<DashSet<usize, ahash::RandomState>> =
        Lazy::new(|| DashSet::with_hasher(ahash::RandomState::new()));
    static RECYCLES: AtomicU64 = AtomicU64::new(0);
    static TIMEOUTS: AtomicU64 = AtomicU64::new(0);
    static STEALS: AtomicU64 = AtomicU64::new(0);

    /// Milliseconds since tracking began (a process-wide monotonic clock).
    pub fn now_ms() -> u64 {
        ORIGIN.elapsed().as_millis() as u64
    }

    /// Clears stamps left over from a previous run.
    pub fn reset() {
        Lazy::force(&ORIGIN);
        LAST_ACTIVITY_MS.clear();
        FINISHED.clear();
        TX_COUNTS.clear();
        STREAM_START_MS.clear();
        RECYCLE_REQUESTED.clear();
        RECYCLES.store(0, Ordering::Relaxed);
        TIMEOUTS.store(0, Ordering::Relaxed);
        STEALS.store(0, Ordering::Relaxed);
    }

    /// Records a successful work steal.
    pub fn note_steal() {
        STEALS.fetch_add(1, Ordering::Relaxed);
    }

    /// Total work steals this run.
    pub fn steal_count() -> u64 {
        STEALS.load(Ordering::Relaxed)
    }

    /// Records a completed connection recycle.
    pub fn note_recycle() {
        RECYCLES.fetch_add(1, Ordering::Relaxed);
    }

    /// Total connection recycles this run.
    pub fn recycle_count() -> u64 {
        RECYCLES.load(Ordering::Relaxed)
    }

    /// Records an operation timeout (a stall that forced a restart).
    pub fn note_timeout() {
        TIMEOUTS.fetch_add(1, Ordering::Relaxed);
    }

    /// Total operation timeouts this run.
    pub fn timeout_count() -> u64 {
        TIMEOUTS.load(Ordering::Relaxed)
    }

    /// Adds processed transactions to `thread_index`'s cumulative count (recycle-rate input).
    pub fn add_transactions(thread_index: usize, count: u64) {
        *TX_COUNTS.entry(thread_index).or_insert(0) += count;
    }

    /// Cumulative transactions processed by `thread_index`.
    pub fn tx_count(thread_index: usize) -> u64 {
        TX_COUNTS
            .get(&thread_index)
            .map(|count| *count)
            .unwrap_or(0)
    }

    /// Records that `thread_index` just (re)opened its stream.
    pub fn note_stream_start(thread_index: usize) {
        STREAM_START_MS.insert(thread_index, now_ms());
    }

    /// Milliseconds since `thread_index` last (re)opened its stream.
    pub fn stream_age_ms(thread_index: usize) -> Option<u64> {
        STREAM_START_MS
            .get(&thread_index)
            .map(|stamp| now_ms().saturating_sub(*stamp))
    }

    /// Asks `thread_index` to recycle its connection at the next block boundary.
    pub fn request_recycle(thread_index: usize) {
        RECYCLE_REQUESTED.insert(thread_index);
    }

    /// Consumes a pending recycle request for `thread_index`.
    pub fn take_recycle(thread_index: usize) -> bool {
        RECYCLE_REQUESTED.remove(&thread_index).is_some()
    }

    /// Records that `thread_index` completed its entire slot range. A finished thread stops
    /// reading forever — without this marker its idle clock would make it look stalled.
    pub fn note_finished(thread_index: usize) {
        FINISHED.insert(thread_index);
    }

    /// Whether `thread_index` completed its slot range.
    pub fn is_finished(thread_index: usize) -> bool {
        FINISHED.contains(&thread_index)
    }

    /// Un-marks a finished thread that adopted stolen work and is running again.
    pub fn clear_finished(thread_index: usize) {
        FINISHED.remove(&thread_index);
    }

    /// Records that `thread_index` just read data.
    pub fn note(thread_index: usize) {
        LAST_ACTIVITY_MS.insert(thread_index, now_ms());
    }

    /// Milliseconds since `thread_index` last read data; `None` if it has not read any yet.
    pub fn idle_ms(thread_index: usize) -> Option<u64> {
        LAST_ACTIVITY_MS
            .get(&thread_index)
            .map(|stamp| now_ms().saturating_sub(*stamp))
    }
}

/// Default launch-gate grace: how long to wait for every running thread to turn green before
/// spawning the next one anyway. Overridden by `JETSTREAMER_SPAWN_GRACE_SECS`; `0` disables
/// launch gating entirely.
const SPAWN_GRACE_DEFAULT: std::time::Duration = std::time::Duration::from_secs(30);

/// Recycle threshold as a percent of the fastest thread's current rate: threads persistently
/// below it restart their connection to shed a throughput-clamped one. Override with
/// `JETSTREAMER_RECYCLE_PCT`; `0` disables recycling.
fn recycle_threshold_pct() -> u64 {
    std::env::var("JETSTREAMER_RECYCLE_PCT")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .map(|pct| pct.min(100))
        .unwrap_or(50)
}

/// Maximum number of running-but-not-yet-green threads the launch gate allows before pausing
/// the ramp. Override with `JETSTREAMER_SPAWN_PENDING`; `1` reproduces the strict
/// one-at-a-time ramp. The default comes from the [`pace`].
fn spawn_pending_max() -> usize {
    std::env::var("JETSTREAMER_SPAWN_PENDING")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|&pending| pending > 0)
        .unwrap_or_else(|| pace().default_spawn_pending())
}

fn spawn_grace_from_env() -> Option<std::time::Duration> {
    match std::env::var("JETSTREAMER_SPAWN_GRACE_SECS") {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(0) => None,
            Ok(secs) => Some(std::time::Duration::from_secs(secs)),
            Err(_) => Some(SPAWN_GRACE_DEFAULT),
        },
        Err(_) => Some(SPAWN_GRACE_DEFAULT),
    }
}

/// Launch gate for the staggered thread ramp: waits until every already-running thread has
/// read data within the "green" window (10% of [`OP_TIMEOUT`], matching the TUI thread grid)
/// so load is only added while the source is keeping up.
///
/// Up to [`spawn_pending_max`] not-yet-green threads may be in flight at once (a freshly
/// spawned thread needs a few seconds to fetch its stream, seek, and read its first block —
/// requiring strict all-green would serialize the ramp on that startup latency). `grace`
/// bounds the wait for merely *sluggish* threads (yellow/orange in the TUI) so a flickering
/// thread cannot stall the ramp forever — but a **red** thread (idle at or beyond the op
/// timeout: stalled or backing off) holds the ramp outright, and the grace clock restarts
/// whenever one is present. Spawning into visible distress only feeds the throttling that
/// caused it. A requested shutdown releases the gate immediately (the spawned thread
/// observes the shutdown flag and exits right away).
async fn wait_for_green_threads(
    grace: std::time::Duration,
    shutdown_flag: &Arc<AtomicBool>,
    handles: &[tokio::task::JoinHandle<()>],
) {
    let green_ms = (OP_TIMEOUT.as_millis() as u64) / 10;
    let red_ms = OP_TIMEOUT.as_millis() as u64;
    let pending_max = spawn_pending_max();
    let spawned = handles.len();
    let mut deadline = std::time::Instant::now() + grace;
    let mut logged_red_hold = false;
    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            return;
        }
        // A thread that already completed its slot range stops reading forever; count it as
        // healthy rather than letting it hold the ramp to the grace timeout.
        let idle_of_running = |(thread, handle): (usize, &tokio::task::JoinHandle<()>)| {
            if handle.is_finished() {
                None
            } else {
                Some(thread_activity::idle_ms(thread))
            }
        };
        let not_yet_green = handles
            .iter()
            .enumerate()
            .filter_map(idle_of_running)
            .filter(|idle| !idle.is_some_and(|idle| idle < green_ms))
            .count();
        let any_red = handles
            .iter()
            .enumerate()
            .filter_map(idle_of_running)
            .any(|idle| idle.is_some_and(|idle| idle >= red_ms));
        // Red is checked before the pending window: red threads count as not-yet-green, and
        // a couple of them must hold the ramp rather than slip under the window.
        if any_red {
            // Hold the ramp and restart the grace clock; only a red-free interval of `grace`
            // can force a spawn past non-green threads.
            deadline = std::time::Instant::now() + grace;
            if !logged_red_hold {
                logged_red_hold = true;
                log::info!(
                    target: LOG_MODULE,
                    "holding thread ramp at {} threads while stalled threads recover",
                    spawned
                );
            }
        } else if not_yet_green < pending_max {
            return;
        } else if std::time::Instant::now() >= deadline {
            log::info!(
                target: LOG_MODULE,
                "spawn grace elapsed with non-green threads; launching thread {} anyway",
                spawned
            );
            return;
        }
        sleep(std::time::Duration::from_millis(15)).await;
    }
}

/// Shared per-thread work ledger used for work-steal victim selection. `start` is the
/// beginning of the thread's current assignment (reset when it adopts stolen work), `next`
/// is the next slot the owner will process (published at each block boundary), and `end` is
/// the half-open end of the slice. Every field is written **only by its owning thread**;
/// other threads read it purely as advisory telemetry when picking a steal victim. Actual
/// splits happen over the steal message protocol (see [`StealRequest`]), never by writing to
/// another thread's slice.
struct WorkSlice {
    start: AtomicU64,
    next: AtomicU64,
    end: AtomicU64,
}

/// Minimum remaining slots a slice must have to be worth splitting: half of this must cover
/// the thief's reconnect + seek setup cost.
const MIN_STEAL_SLOTS: u64 = 64;

/// The active run's work ledger, stashed so out-of-band reporters (e.g. the fatal
/// ClickHouse-abort path) can compute a safe resume point.
static ACTIVE_WORK_LEDGER: std::sync::Mutex<Option<Arc<Vec<WorkSlice>>>> =
    std::sync::Mutex::new(None);

/// The lowest slot not yet fully processed by the active run, if one is running: everything
/// below this is complete, so `resume_floor..original_end` is a safe (if conservative —
/// higher threads' finished work above the floor gets re-read) resume range. Returns `None`
/// when no run is active or every slice is complete.
pub fn resume_floor() -> Option<u64> {
    let ledger = ACTIVE_WORK_LEDGER.lock().unwrap();
    ledger.as_ref().and_then(|slices| {
        slices
            .iter()
            .filter_map(|slice| {
                let next = slice.next.load(Ordering::SeqCst);
                let end = slice.end.load(Ordering::SeqCst);
                (next < end).then_some(next)
            })
            .min()
    })
}

/// A work-steal proposal sent to a victim thread's steal inbox: "hand me half of your
/// remaining work." The victim answers on `reply` with the granted range, or `None` when it
/// has too little work left to split. The victim only services its inbox at quiescent points
/// (between block batches, at restart boundaries, or while parked in backoff), so a grant
/// can never race in-flight emission — the victim's answer *is* the authoritative split.
struct StealRequest {
    reply: oneshot::Sender<Option<Range<u64>>>,
}

/// Services a victim's steal inbox at a quiescent point. `position` is the victim's
/// authoritative next-slot-to-process; `allow` is false when the victim is completing its
/// range and only draining (all requests answered `None`). A grant is committed — the local
/// range end shrinks and the ledger is updated — only if the reply is actually delivered, so
/// an abandoned request can never orphan slots.
fn service_steal_inbox(
    inbox: &mut mpsc::UnboundedReceiver<StealRequest>,
    slot_range: &mut Range<u64>,
    position: u64,
    slice: &WorkSlice,
    log_target: &str,
    allow: bool,
) {
    while let Ok(request) = inbox.try_recv() {
        let remaining = slot_range.end.saturating_sub(position);
        if !allow || remaining < MIN_STEAL_SLOTS {
            let _ = request.reply.send(None);
            continue;
        }
        let mid = position + remaining / 2;
        let granted = mid..slot_range.end;
        if request.reply.send(Some(granted.clone())).is_ok() {
            log::info!(
                target: log_target,
                "🐾 handed slots {}..{} to a work-stealing thread; continuing to {}",
                granted.start,
                granted.end,
                mid
            );
            slot_range.end = mid;
            slice.end.store(mid, Ordering::SeqCst);
        }
    }
}

/// Asks the least-progressed running thread for half of its remaining work, walking the
/// candidate list until a victim grants. Candidates are ranked by lowest completed fraction
/// of their current assignment (ties broken by most remaining), skipping threads that have
/// not started streaming yet (they cannot answer their inbox) and threads without enough
/// work to split.
///
/// Deadlock freedom: while awaiting a victim's answer, the thief keeps servicing its **own**
/// inbox (rejecting — it has nothing to give), so every thread in every waiting state stays
/// responsive. A request can only go permanently unanswered if the victim task exits, which
/// drops its inbox and resolves the wait with an error. `lock` is held only around the
/// scan-and-send (never across the reply await) to keep simultaneous thieves from bursting
/// requests at the same victim; the victim re-validates every grant against its own
/// authoritative position anyway.
async fn request_steal(
    registry: &[WorkSlice],
    inboxes: &[mpsc::UnboundedSender<StealRequest>],
    own_inbox: &mut mpsc::UnboundedReceiver<StealRequest>,
    thief: usize,
    lock: &tokio::sync::Mutex<()>,
) -> Option<(usize, Range<u64>)> {
    let mut candidates: Vec<(f64, std::cmp::Reverse<u64>, usize)> = {
        let _guard = lock.lock().await;
        registry
            .iter()
            .enumerate()
            .filter(|&(index, _)| {
                index != thief
                    && !thread_activity::is_finished(index)
                    // A thread that has not begun streaming cannot answer its inbox.
                    && thread_activity::stream_age_ms(index).is_some()
            })
            .filter_map(|(index, slice)| {
                let start = slice.start.load(Ordering::SeqCst);
                let next = slice.next.load(Ordering::SeqCst);
                let end = slice.end.load(Ordering::SeqCst);
                let remaining = end.saturating_sub(next);
                if remaining < MIN_STEAL_SLOTS {
                    return None;
                }
                let assigned = end.saturating_sub(start).max(1);
                let fraction = next.saturating_sub(start) as f64 / assigned as f64;
                Some((fraction, std::cmp::Reverse(remaining), index))
            })
            .collect()
    };
    candidates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    for (_, _, victim) in candidates {
        let (reply_tx, reply_rx) = oneshot::channel();
        if inboxes[victim]
            .send(StealRequest { reply: reply_tx })
            .is_err()
        {
            continue;
        }
        // Await the victim's answer while staying responsive to our own inbox.
        let mut reply_rx = reply_rx;
        let outcome = loop {
            tokio::select! {
                reply = &mut reply_rx => break reply,
                incoming = own_inbox.recv() => {
                    match incoming {
                        // We are hunting because we have nothing left; refuse.
                        Some(request) => {
                            let _ = request.reply.send(None);
                        }
                        // Own inbox closed (shutdown teardown): just wait for the reply.
                        None => break (&mut reply_rx).await,
                    }
                }
            }
        };
        match outcome {
            Ok(Some(stolen)) => {
                registry[thief].start.store(stolen.start, Ordering::SeqCst);
                registry[thief].next.store(stolen.start, Ordering::SeqCst);
                registry[thief].end.store(stolen.end, Ordering::SeqCst);
                return Some((victim, stolen));
            }
            Ok(None) | Err(_) => continue,
        }
    }
    None
}

/// Decides how a reverse-mode retry resumes after an error attributed to `slot`, given that
/// `last_counted_slot` was the last slot fully processed. Returns the new
/// `(reverse_partial_resume, reverse_highest_remaining_epoch)`.
///
/// The subtlety: an error striking at an epoch slice's *tail* (after its final block was
/// emitted, before the clean end-of-epoch break) is attributed to the next slot — which
/// belongs to the next, **higher** epoch. Storing that as the partial-resume point poisons
/// the retry: the epoch-match check sees a foreign epoch, discards the resume marker, and
/// re-processes the entire slice from its start, double-emitting every slot in it. When the
/// resume point crosses above the epoch that was actually being processed, the slice is
/// complete — so mark the epoch done instead.
fn reverse_resume_after_error(
    slot: u64,
    last_counted_slot: u64,
    highest_remaining_epoch: Option<u64>,
) -> (Option<u64>, Option<u64>) {
    let resume_slot = if slot <= last_counted_slot {
        last_counted_slot.saturating_add(1)
    } else {
        slot
    };
    let last_epoch = slot_to_epoch(last_counted_slot);
    let error_epoch = slot_to_epoch(slot);
    if error_epoch >= last_epoch && slot_to_epoch(resume_slot) > last_epoch {
        // Tail case: everything in `last_epoch` was processed. Only decrement the
        // highest-remaining marker when it still points at that epoch (it may already point
        // lower, e.g. when the error arrived before the first block of an earlier epoch).
        let highest = if highest_remaining_epoch == Some(last_epoch) {
            // `checked_sub` makes "no epochs remaining" explicit as `None` — required for
            // epoch 0, where a saturating subtraction would silently stay at 0 and replay
            // the epoch. Dropping below the range's lowest epoch also completes the run.
            last_epoch.checked_sub(1)
        } else {
            highest_remaining_epoch
        };
        (None, highest)
    } else {
        (Some(resume_slot), highest_remaining_epoch)
    }
}

/// Per-thread restart pacing: repeated failures on one slot double the [`pace`] delay up to its
/// cap, and a failure on a new slot (forward progress) resets it.
struct RetryBackoff {
    last_slot: Option<u64>,
    consecutive: u32,
}

impl RetryBackoff {
    const fn new() -> Self {
        Self {
            last_slot: None,
            consecutive: 0,
        }
    }

    fn next_delay(&mut self, slot: u64) -> std::time::Duration {
        if self.last_slot == Some(slot) {
            self.consecutive = self.consecutive.saturating_add(1);
        } else {
            self.last_slot = Some(slot);
            self.consecutive = 0;
        }
        let pace = pace();
        pace.retry_backoff_base()
            .saturating_mul(1u32 << self.consecutive.min(10))
            .min(pace.retry_backoff_max())
    }
}

/// Errors that can occur while streaming the firehose. Errors that can occur while streaming
/// the firehose.
#[derive(Debug, Error)]
pub enum FirehoseError {
    /// HTTP client error surfaced from `reqwest`.
    Reqwest(reqwest::Error),
    /// Failure while reading the Old Faithful CAR header.
    ReadHeader(SharedError),
    /// Error emitted by the Solana Geyser plugin service.
    #[cfg(feature = "geyser")]
    GeyserPluginService(GeyserPluginServiceError),
    /// Transaction notifier could not be acquired from the Geyser service.
    #[cfg(feature = "geyser")]
    FailedToGetTransactionNotifier,
    /// Failure while reading data until the next block boundary.
    ReadUntilBlockError(SharedError),
    /// Failure while fetching an individual block.
    GetBlockError(SharedError),
    /// Failed to decode a node at the given index.
    NodeDecodingError(usize, SharedError),
    /// Error surfaced when querying the slot offset index.
    SlotOffsetIndexError(SlotOffsetIndexError),
    /// Failure while seeking to a slot within the Old Faithful CAR stream.
    SeekToSlotError(SharedError),
    /// Error surfaced during the plugin `on_load` stage.
    OnLoadError(SharedError),
    /// Error emitted while invoking the stats handler.
    OnStatsHandlerError(SharedError),
    /// Timeout reached while waiting for a firehose operation.
    OperationTimeout(&'static str),
    /// Deliberate connection recycle (not a failure): the thread restarts its stream to shed
    /// a throughput-clamped connection.
    ConnectionRecycled,
    /// The thread's slot range is fully processed (not a failure): routed through the retry
    /// loop so the thread can adopt stolen work or retire.
    RangeComplete,
    /// The HTTP stream ended (EOF) while the slot index proves present slots remain in the
    /// thread's range — the CDN closed the connection mid-transfer. Retryable; without this
    /// check a truncated stream is indistinguishable from a genuine end-of-epoch and the
    /// remaining slots would be silently lost.
    PrematureStreamEnd,
    /// Transaction handler returned an error.
    TransactionHandlerError(SharedError),
    /// Entry handler returned an error.
    EntryHandlerError(SharedError),
    /// Reward handler returned an error.
    RewardHandlerError(SharedError),
    /// Block handler returned an error.
    BlockHandlerError(SharedError),
}

unsafe impl Send for FirehoseError {}
unsafe impl Sync for FirehoseError {}

impl Display for FirehoseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirehoseError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            FirehoseError::ReadHeader(error) => {
                write!(f, "Error reading header: {}", error)
            }
            #[cfg(feature = "geyser")]
            FirehoseError::GeyserPluginService(geyser_plugin_service_error) => write!(
                f,
                "Error initializing geyser plugin service: {}",
                geyser_plugin_service_error
            ),
            #[cfg(feature = "geyser")]
            FirehoseError::FailedToGetTransactionNotifier => write!(
                f,
                "Failed to get transaction notifier from GeyserPluginService"
            ),
            FirehoseError::ReadUntilBlockError(error) => {
                write!(f, "Error reading until block: {}", error)
            }
            FirehoseError::GetBlockError(error) => write!(f, "Error getting block: {}", error),
            FirehoseError::NodeDecodingError(item_index, error) => {
                write!(
                    f,
                    "Error seeking, reading data from, or decoding data for data node {}: {}",
                    item_index, error
                )
            }
            FirehoseError::SlotOffsetIndexError(slot_offset_index_error) => write!(
                f,
                "Error getting info from slot offset index: {}",
                slot_offset_index_error
            ),
            FirehoseError::SeekToSlotError(error) => {
                write!(f, "Error seeking to slot: {}", error)
            }
            FirehoseError::OnLoadError(error) => write!(f, "Error on load: {}", error),
            FirehoseError::OnStatsHandlerError(error) => {
                write!(f, "Stats handler error: {}", error)
            }
            FirehoseError::OperationTimeout(op) => {
                write!(f, "Timeout while waiting for operation: {}", op)
            }
            FirehoseError::ConnectionRecycled => {
                write!(f, "connection recycled to refresh throughput")
            }
            FirehoseError::RangeComplete => {
                write!(f, "slot range complete")
            }
            FirehoseError::PrematureStreamEnd => {
                write!(
                    f,
                    "stream ended before the slot range was fully processed (connection closed mid-transfer)"
                )
            }
            FirehoseError::TransactionHandlerError(error) => {
                write!(f, "Transaction handler error: {}", error)
            }
            FirehoseError::EntryHandlerError(error) => {
                write!(f, "Entry handler error: {}", error)
            }
            FirehoseError::RewardHandlerError(error) => {
                write!(f, "Reward handler error: {}", error)
            }
            FirehoseError::BlockHandlerError(error) => {
                write!(f, "Block handler error: {}", error)
            }
        }
    }
}

impl From<reqwest::Error> for FirehoseError {
    fn from(e: reqwest::Error) -> Self {
        FirehoseError::Reqwest(e)
    }
}

#[cfg(feature = "geyser")]
impl From<GeyserPluginServiceError> for FirehoseError {
    fn from(e: GeyserPluginServiceError) -> Self {
        FirehoseError::GeyserPluginService(e)
    }
}

impl From<SlotOffsetIndexError> for FirehoseError {
    fn from(e: SlotOffsetIndexError) -> Self {
        FirehoseError::SlotOffsetIndexError(e)
    }
}

/// Per-thread progress information emitted by the firehose runner.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ThreadStats {
    /// Identifier of the worker thread reporting the stats.
    pub thread_id: usize,
    /// Timestamp captured when the thread began processing.
    pub start_time: std::time::Instant,
    /// Timestamp captured when the thread finished, if finished.
    pub finish_time: Option<std::time::Instant>,
    /// Slot range currently assigned to the thread (half-open, may shrink on restart).
    pub slot_range: Range<u64>,
    /// Original slot range assigned to the thread (half-open, never modified).
    pub initial_slot_range: Range<u64>,
    /// Latest slot processed by the thread.
    pub current_slot: u64,
    /// Total slots processed by the thread.
    pub slots_processed: u64,
    /// Number of blocks successfully processed.
    pub blocks_processed: u64,
    /// Number of slots skipped by the cluster leader.
    pub leader_skipped_slots: u64,
    /// Total transactions processed.
    pub transactions_processed: u64,
    /// Total entries processed.
    pub entries_processed: u64,
    /// Total rewards processed.
    pub rewards_processed: u64,
}

/// Aggregated firehose statistics covering all worker threads.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Stats {
    /// Per-thread statistics for the current update.
    pub thread_stats: ThreadStats,
    /// Timestamp captured when processing began.
    pub start_time: std::time::Instant,
    /// Timestamp captured when all processing finished, if finished.
    pub finish_time: Option<std::time::Instant>,
    /// Slot range currently being processed (half-open [start, end)).
    pub slot_range: Range<u64>,
    /// Aggregate slots processed across all threads.
    pub slots_processed: u64,
    /// Aggregate blocks processed across all threads.
    pub blocks_processed: u64,
    /// Aggregate skipped slots across all threads.
    pub leader_skipped_slots: u64,
    /// Aggregate transactions processed across all threads.
    pub transactions_processed: u64,
    /// Aggregate entries processed across all threads.
    pub entries_processed: u64,
    /// Aggregate rewards processed across all threads.
    pub rewards_processed: u64,
    /// Transactions processed since the previous stats pulse.
    pub transactions_since_last_pulse: u64,
    /// Blocks processed since the previous stats pulse.
    pub blocks_since_last_pulse: u64,
    /// Slots processed since the previous stats pulse.
    pub slots_since_last_pulse: u64,
    /// Elapsed time since the previous stats pulse.
    pub time_since_last_pulse: std::time::Duration,
}

/// Configuration for periodic stats emission via a [`Handler`] callback.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct StatsTracking<OnStats: Handler<Stats>> {
    /// Callback invoked whenever new stats are available.
    pub on_stats: OnStats,
    /// Emits a stats callback when the current slot is a multiple of this interval.
    pub tracking_interval_slots: u64,
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
async fn maybe_emit_stats<OnStats: Handler<Stats>>(
    stats_tracking: Option<&StatsTracking<OnStats>>,
    thread_index: usize,
    thread_stats: &ThreadStats,
    overall_slots_processed: &AtomicU64,
    overall_blocks_processed: &AtomicU64,
    overall_transactions_processed: &AtomicU64,
    overall_entries_processed: &AtomicU64,
    transactions_since_stats: &AtomicU64,
    blocks_since_stats: &AtomicU64,
    slots_since_stats: &AtomicU64,
    last_pulse: &Arc<AtomicU64>,
    base_instant: std::time::Instant,
) -> Result<(), (FirehoseError, u64)> {
    if let Some(stats_tracker) = stats_tracking {
        let total_slots = overall_slots_processed.load(Ordering::Relaxed);
        let total_blocks = overall_blocks_processed.load(Ordering::Relaxed);
        let total_transactions = overall_transactions_processed.load(Ordering::Relaxed);
        let total_entries = overall_entries_processed.load(Ordering::Relaxed);
        let now_nanos = base_instant.elapsed().as_nanos() as u64;
        let previous = last_pulse.swap(now_nanos, Ordering::Relaxed);
        let delta_nanos = now_nanos.saturating_sub(previous);
        let time_since_last_pulse = std::time::Duration::from_nanos(delta_nanos.max(1));
        let processed_transactions = transactions_since_stats.swap(0, Ordering::Relaxed);
        let processed_blocks = blocks_since_stats.swap(0, Ordering::Relaxed);
        let processed_slots = slots_since_stats.swap(0, Ordering::Relaxed);

        let stats = Stats {
            thread_stats: thread_stats.clone(),
            start_time: thread_stats.start_time,
            finish_time: thread_stats.finish_time,
            slot_range: thread_stats.slot_range.clone(),
            slots_processed: total_slots,
            blocks_processed: total_blocks,
            leader_skipped_slots: total_slots.saturating_sub(total_blocks),
            transactions_processed: total_transactions,
            entries_processed: total_entries,
            rewards_processed: thread_stats.rewards_processed,
            transactions_since_last_pulse: processed_transactions,
            blocks_since_last_pulse: processed_blocks,
            slots_since_last_pulse: processed_slots,
            time_since_last_pulse,
        };

        if let Err(e) = (stats_tracker.on_stats)(thread_index, stats).await {
            last_pulse.store(previous, Ordering::Relaxed);
            transactions_since_stats.fetch_add(processed_transactions, Ordering::Relaxed);
            blocks_since_stats.fetch_add(processed_blocks, Ordering::Relaxed);
            slots_since_stats.fetch_add(processed_slots, Ordering::Relaxed);
            return Err((
                FirehoseError::OnStatsHandlerError(e),
                thread_stats.current_slot,
            ));
        }
    }
    Ok(())
}

#[inline(always)]
fn fetch_add_if(tracking_enabled: bool, atomic: &AtomicU64, value: u64) {
    if tracking_enabled {
        atomic.fetch_add(value, Ordering::Relaxed);
    }
}

fn clear_pending_skip(
    map: &DashMap<usize, DashSet<u64, ahash::RandomState>, ahash::RandomState>,
    thread_id: usize,
    slot: u64,
) -> bool {
    map.get(&thread_id)
        .map(|set| set.remove(&slot).is_some())
        .unwrap_or(false)
}

fn decode_transaction_status_meta_from_frame(
    slot: u64,
    reassembled_metadata: Vec<u8>,
) -> Result<solana_transaction_status::TransactionStatusMeta, SharedError> {
    if reassembled_metadata.is_empty() {
        // Early epochs often omit metadata entirely.
        return Ok(solana_transaction_status::TransactionStatusMeta::default());
    }

    match utils::decompress_zstd(reassembled_metadata.as_slice()) {
        Ok(decompressed) => {
            decode_transaction_status_meta(slot, decompressed.as_slice()).map_err(|err| {
                Box::new(std::io::Error::other(format!(
                    "decode transaction metadata (slot {slot}): {err}"
                ))) as SharedError
            })
        }
        Err(decomp_err) => {
            // If the frame was not zstd-compressed (common for very early data), try to
            // decode the raw bytes directly before bailing.
            decode_transaction_status_meta(slot, reassembled_metadata.as_slice()).map_err(|err| {
                Box::new(std::io::Error::other(format!(
                    "transaction metadata not zstd-compressed for slot {slot}; raw decode failed (raw_err={err}, decompress_err={decomp_err})"
                ))) as SharedError
            })
        }
    }
}

#[derive(Debug)]
struct DecodedRewards {
    /// Rewards in the runtime's representation, ready to hand to block handlers and the geyser
    /// block metadata notifier without further conversion.
    rewards: KeyedRewardsAndNumPartitions,
    /// Whether the ledger recorded vote commission in basis points (SIMD-0291) rather than
    /// whole percent for this block. Forwarded to the geyser block metadata notifier so it
    /// renders `commission`/`commission_bps` the same way a live validator would have.
    commission_rate_in_basis_points: bool,
}

impl Default for DecodedRewards {
    fn default() -> Self {
        Self::empty()
    }
}

impl DecodedRewards {
    fn empty() -> Self {
        Self {
            rewards: KeyedRewardsAndNumPartitions {
                keyed_rewards: Vec::new(),
                num_partitions: None,
            },
            commission_rate_in_basis_points: false,
        }
    }

    /// Copies the rewards into the form conveyed to reward [`Handler`] callbacks.
    fn to_handler_rewards(&self) -> Vec<(Address, StakeRewardInfo)> {
        self.rewards
            .keyed_rewards
            .iter()
            .map(|(address, reward)| {
                (
                    *address,
                    StakeRewardInfo {
                        reward_type: reward.reward_type,
                        lamports: reward.lamports,
                        post_balance: reward.post_balance,
                        commission_bps: reward.commission_bps,
                    },
                )
            })
            .collect()
    }
}

fn decode_rewards_from_frame(
    slot: u64,
    reassembled_rewards: Vec<u8>,
) -> Result<DecodedRewards, SharedError> {
    if reassembled_rewards.is_empty() {
        // Early epochs sometimes omit rewards payloads entirely.
        return Ok(DecodedRewards::empty());
    }

    match utils::decompress_zstd(reassembled_rewards.as_slice()) {
        Ok(decompressed) => decode_rewards_from_bytes(slot, decompressed.as_slice()).map_err(
            |err| {
                Box::new(std::io::Error::other(format!(
                    "decode rewards (slot {slot}): {err}"
                ))) as SharedError
            },
        ),
        Err(decomp_err) => decode_rewards_from_bytes(slot, reassembled_rewards.as_slice()).map_err(
            |err| {
                Box::new(std::io::Error::other(format!(
                    "rewards not zstd-compressed for slot {slot}; raw decode failed (raw_err={err}, decompress_err={decomp_err})"
                ))) as SharedError
            },
        ),
    }
}

fn decode_rewards_from_bytes(slot: u64, bytes: &[u8]) -> Result<DecodedRewards, SharedError> {
    let epoch = slot_to_epoch(slot);
    let proto_attempt: Result<solana_storage_proto::convert::generated::Rewards, _> =
        prost_014::Message::decode(bytes);
    match proto_attempt {
        Ok(proto) => {
            let rewards = convert_proto_rewards(&proto).map_err(|err| {
                Box::new(std::io::Error::other(format!(
                    "convert rewards proto failed (epoch {epoch}): {err}"
                ))) as SharedError
            })?;
            Ok(DecodedRewards {
                rewards,
                commission_rate_in_basis_points: proto_commission_in_basis_points(&proto),
            })
        }
        Err(proto_err) => {
            let stored: solana_storage_proto::StoredExtendedRewards =
                bincode::deserialize(bytes).map_err(|bin_err| {
                    Box::new(std::io::Error::other(format!(
                        "protobuf decode rewards failed (epoch {epoch}); bincode failed too: {bin_err}; protobuf error: {proto_err}"
                    ))) as SharedError
                })?;
            let proto: solana_storage_proto::convert::generated::Rewards = stored.into();
            let rewards = convert_proto_rewards(&proto).map_err(|err| {
                Box::new(std::io::Error::other(format!(
                    "convert rewards bincode fallback failed (epoch {epoch}); protobuf error: {proto_err}; conversion error: {err}"
                ))) as SharedError
            })?;
            Ok(DecodedRewards {
                rewards,
                commission_rate_in_basis_points: proto_commission_in_basis_points(&proto),
            })
        }
    }
}

/// Returns `true` when the stored rewards carry a basis-point commission (SIMD-0291), i.e. the
/// block was produced after the `commission_rate_in_basis_points` feature activated.
fn proto_commission_in_basis_points(
    proto: &solana_storage_proto::convert::generated::Rewards,
) -> bool {
    proto
        .rewards
        .iter()
        .any(|reward| !reward.commission_bps.is_empty())
}

fn decode_transaction_status_meta(
    slot: u64,
    metadata_bytes: &[u8],
) -> Result<solana_transaction_status::TransactionStatusMeta, SharedError> {
    let epoch = slot_to_epoch(slot);
    let mut bincode_err: Option<String> = None;
    if epoch < BINCODE_EPOCH_CUTOFF {
        match bincode::deserialize::<solana_storage_proto::StoredTransactionStatusMeta>(
            metadata_bytes,
        ) {
            Ok(stored) => return Ok(stored.into()),
            Err(err) => {
                bincode_err = Some(err.to_string());
            }
        }
    }

    let bin_err_for_proto = bincode_err.clone();
    let proto: solana_storage_proto::convert::generated::TransactionStatusMeta =
        prost_014::Message::decode(metadata_bytes).map_err(|err| {
            // If we already tried bincode, surface both failures for easier debugging.
            if let Some(ref bin_err) = bin_err_for_proto {
                Box::new(std::io::Error::other(format!(
                    "protobuf decode transaction metadata failed (epoch {epoch}); bincode failed earlier: {bin_err}; protobuf error: {err}"
                ))) as SharedError
            } else {
                Box::new(std::io::Error::other(format!(
                    "protobuf decode transaction metadata: {err}"
                ))) as SharedError
            }
        })?;

    proto.try_into().map_err(|err| {
        if let Some(ref bin_err) = bincode_err {
            Box::new(std::io::Error::other(format!(
                "convert transaction metadata proto failed (epoch {epoch}); bincode failed earlier: {bin_err}; conversion error: {err}"
            ))) as SharedError
        } else {
            Box::new(std::io::Error::other(format!(
                "convert transaction metadata proto: {err}"
            ))) as SharedError
        }
    })
}

/// Firehose transaction payload passed to [`Handler`] callbacks.
#[derive(Debug, Clone)]
pub struct TransactionData {
    /// Slot that contains the transaction.
    pub slot: u64,
    /// Index of the transaction within the slot.
    pub transaction_slot_index: usize,
    /// Transaction signature.
    pub signature: solana_signature::Signature,
    /// Hash of the transaction message.
    pub message_hash: Hash,
    /// Indicates whether the transaction is a vote.
    pub is_vote: bool,
    /// Status metadata returned by the Solana runtime.
    pub transaction_status_meta: solana_transaction_status::TransactionStatusMeta,
    /// Fully decoded transaction.
    pub transaction: VersionedTransaction,
}

/// Block entry metadata passed to [`Handler`] callbacks.
#[derive(Debug, Clone)]
pub struct EntryData {
    /// Slot that generated the entry.
    pub slot: u64,
    /// Index of the entry within the slot.
    pub entry_index: usize,
    /// Range of transaction indexes covered by the entry.
    pub transaction_indexes: Range<usize>,
    /// Number of hashes associated with the entry.
    pub num_hashes: u64,
    /// Entry hash.
    pub hash: Hash,
}

/// Reward data conveyed to reward [`Handler`] callbacks.
#[derive(Debug, Clone)]
pub struct RewardsData {
    /// Slot the rewards correspond to.
    pub slot: u64,
    /// Reward recipients and their associated reward information.
    ///
    /// Agave 4.x no longer exports `solana_runtime`'s `RewardInfo`, so rewards are conveyed as
    /// [`solana_accounts_db::stake_rewards::StakeRewardInfo`], which has the same fields and
    /// converts into the runtime type.
    pub rewards: Vec<(Address, StakeRewardInfo)>,
}

/// Block-level data streamed to block handlers.
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum BlockData {
    /// Fully populated block payload with ledger metadata.
    Block {
        /// Parent slot number.
        parent_slot: u64,
        /// Parent block hash.
        parent_blockhash: Hash,
        /// Current block slot.
        slot: u64,
        /// Current block hash.
        blockhash: Hash,
        /// Rewards keyed by account and partition information.
        rewards: KeyedRewardsAndNumPartitions,
        /// Whether the stored rewards contain a basis-point commission (SIMD-0291).
        /// Commission values are normalized to basis points regardless of this flag.
        /// False when no stored reward contains a basis-point commission, including
        /// blocks without rewards.
        commission_rate_in_basis_points: bool,
        /// Optional Unix timestamp for the block.
        block_time: Option<i64>,
        /// Optional ledger block height.
        block_height: Option<u64>,
        /// Number of executed transactions in the block.
        executed_transaction_count: u64,
        /// Number of entries contained in the block.
        entry_count: u64,
        /// Raw Alpenglow block markers in block order. `None` before Alpenglow.
        /// The byte layout is on [`crate::jetstreamer_firehose::block::SlotMeta::block_markers`].
        block_markers: Option<Vec<Vec<u8>>>,
        /// Alpenglow block id, the double merkle root that certificates vote on.
        /// `None` before Alpenglow.
        block_id: Option<Hash>,
    },
    /// Marker indicating the slot appears skipped (either truly skipped or it is late and will
    /// arrive out of order).
    PossibleLeaderSkipped {
        /// Slot number that either lacked a block or may still arrive later.
        slot: u64,
    },
}

impl BlockData {
    /// Returns the slot associated with this block or skipped slot.
    #[inline(always)]
    pub const fn slot(&self) -> u64 {
        match self {
            BlockData::Block { slot, .. } => *slot,
            BlockData::PossibleLeaderSkipped { slot } => *slot,
        }
    }

    /// Returns `true` if this record currently represents a missing/possibly skipped slot.
    #[inline(always)]
    pub const fn was_skipped(&self) -> bool {
        matches!(self, BlockData::PossibleLeaderSkipped { .. })
    }

    /// Returns the optional block time when available.
    #[inline(always)]
    pub const fn block_time(&self) -> Option<i64> {
        match self {
            BlockData::Block { block_time, .. } => *block_time,
            BlockData::PossibleLeaderSkipped { .. } => None,
        }
    }
}

type HandlerResult = Result<(), SharedError>;
type HandlerFuture = BoxFuture<'static, HandlerResult>;

/// Asynchronous callback invoked for each firehose event of type `Data`.
pub trait Handler<Data>: Fn(usize, Data) -> HandlerFuture + Send + Sync + Clone + 'static {}

impl<Data, F> Handler<Data> for F where
    F: Fn(usize, Data) -> HandlerFuture + Send + Sync + Clone + 'static
{
}

/// Function pointer alias for [`Handler`] callbacks.
pub type HandlerFn<Data> = fn(usize, Data) -> HandlerFuture;
/// Convenience alias for block handlers accepted by [`firehose`].
pub type OnBlockFn = HandlerFn<BlockData>;
/// Convenience alias for transaction handlers accepted by [`firehose`].
pub type OnTxFn = HandlerFn<TransactionData>;
/// Convenience alias for entry handlers accepted by [`firehose`].
pub type OnEntryFn = HandlerFn<EntryData>;
/// Convenience alias for reward handlers accepted by [`firehose`].
pub type OnRewardFn = HandlerFn<RewardsData>;
/// Type alias for [`StatsTracking`] using simple function pointers.
pub type StatsTracker = StatsTracking<HandlerFn<Stats>>;
/// Convenience alias for firehose error handlers.
pub type OnErrorFn = HandlerFn<FirehoseErrorContext>;
/// Convenience alias for stats tracking handlers accepted by [`firehose`].
pub type OnStatsTrackingFn = StatsTracking<HandlerFn<Stats>>;

/// Metadata describing a firehose worker failure.
#[derive(Clone, Debug)]
pub struct FirehoseErrorContext {
    /// Thread index that encountered the error.
    pub thread_id: usize,
    /// Slot the worker was processing when the error surfaced.
    pub slot: u64,
    /// Epoch derived from the failing slot.
    pub epoch: u64,
    /// Stringified error payload for display/logging.
    pub error_message: String,
}

/// Streams blocks, transactions, entries, rewards, and stats to user-provided handlers.
///
/// The requested `slot_range` is half-open `[start, end)`. Recoverable data errors retry
/// the incomplete slot. Stats handler failures retry the pending pulse before reading more
/// data, preserving completed slot progress and avoiding duplicate data callbacks.
///
/// When `sequential` is `true`, the firehose uses one worker thread and opens epoch streams
/// with ripget's parallel windowed downloader. In this mode `threads` configures ripget range
/// concurrency rather than firehose worker partitioning.
///
/// `buffer_window_bytes` controls the ripget hot/cold window when `sequential` is enabled.
/// Pass `None` to use the default (`min(4 GiB, 15% of available RAM)`).
///
/// When `reverse` is `true` (sequential mode only), epochs in the requested range are
/// processed from highest to lowest. Within each epoch slots are still emitted in ascending
/// order because the underlying CAR archive can only be streamed forward.
#[inline]
#[allow(clippy::too_many_arguments)]
pub async fn firehose<OnBlock, OnTransaction, OnEntry, OnRewards, OnStats, OnError>(
    threads: u64,
    sequential: bool,
    reverse: bool,
    buffer_window_bytes: Option<u64>,
    slot_range: Range<u64>,
    on_block: Option<OnBlock>,
    on_tx: Option<OnTransaction>,
    on_entry: Option<OnEntry>,
    on_rewards: Option<OnRewards>,
    on_error: Option<OnError>,
    stats_tracking: Option<StatsTracking<OnStats>>,
    shutdown_signal: Option<broadcast::Receiver<()>>,
) -> Result<(), (FirehoseError, u64)>
where
    OnBlock: Handler<BlockData>,
    OnTransaction: Handler<TransactionData>,
    OnEntry: Handler<EntryData>,
    OnRewards: Handler<RewardsData>,
    OnStats: Handler<Stats>,
    OnError: Handler<FirehoseErrorContext>,
{
    if threads == 0 {
        return Err((
            FirehoseError::OnLoadError("Number of threads must be greater than 0".into()),
            slot_range.start,
        ));
    }
    let client = crate::jetstreamer_firehose::network::create_http_client();
    log::info!(target: LOG_MODULE, "starting firehose...");
    log::info!(target: LOG_MODULE, "index base url: {}", SLOT_OFFSET_INDEX.base_url());
    // Reverse mode implies sequential mode; activate it automatically when caller passed
    // `reverse: true` without `sequential: true`.
    let sequential = sequential || reverse;
    let firehose_threads = if sequential { 1 } else { threads };
    let sequential_download_threads = std::cmp::max(1, threads as usize);
    let sequential_buffer_window_bytes = buffer_window_bytes
        .filter(|value| *value >= 2)
        .unwrap_or_else(crate::jetstreamer_firehose::system::default_firehose_buffer_window_bytes);
    if sequential {
        log::info!(
            target: LOG_MODULE,
            "sequential mode enabled: firehose_threads=1, ripget_threads={}, ripget_window={}",
            sequential_download_threads,
            crate::jetstreamer_firehose::system::format_byte_size(sequential_buffer_window_bytes)
        );
    }
    let reverse_mode = reverse;
    if reverse_mode {
        log::info!(
            target: LOG_MODULE,
            "reverse mode enabled: epochs processed from highest to lowest"
        );
    }

    let slot_range = Arc::new(slot_range);

    // divide slot_range into n subranges
    let subranges = generate_subranges(&slot_range, firehose_threads);
    if firehose_threads > 1 {
        log::debug!(target: LOG_MODULE, "❄️ thread sub-ranges: {:?}", subranges);
    }

    let firehose_start = std::time::Instant::now();
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    if let Some(ref rx) = shutdown_signal {
        let mut rx = rx.resubscribe();
        let flag = shutdown_flag.clone();
        tokio::spawn(async move {
            if rx.recv().await.is_ok() {
                log::info!(target: LOG_MODULE, "shutdown signal received; notifying firehose threads");
                flag.store(true, Ordering::SeqCst);
            }
        });
    }

    // Build a shared ripget HTTP client so TCP connections survive across epoch transitions.
    let shared_ripget_client: Option<ripget::Client> = if sequential {
        Some(
            ripget::build_client(Some(&format!(
                "jetstreamer-firehose/{}",
                env!("CARGO_PKG_VERSION")
            )))
            .expect("failed to build ripget HTTP client"),
        )
    } else {
        None
    };

    let mut handles = Vec::new();
    // Shared per-thread error counters
    let error_counts: Arc<Vec<AtomicU32>> =
        Arc::new((0..subranges.len()).map(|_| AtomicU32::new(0)).collect());

    let overall_slots_processed: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let overall_blocks_processed: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let overall_transactions_processed: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let overall_entries_processed: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let pending_skipped_slots: Arc<
        DashMap<usize, DashSet<u64, ahash::RandomState>, ahash::RandomState>,
    > = Arc::new(DashMap::with_hasher(ahash::RandomState::new()));

    thread_activity::reset();
    let spawn_gate = if sequential {
        None
    } else {
        spawn_grace_from_env()
    };
    // Connection-recycle monitor: Cloudflare clamps long-lived connections while fresh ones
    // get full burst throughput, and a clamped-but-flowing thread stays green forever, so
    // health checks alone never rotate it. Every sweep, threads running well below the
    // fastest thread's rate are asked to reconnect (a clean restart with no backoff).
    let recycle_pct = recycle_threshold_pct();
    let thread_total = subranges.len();
    // Work-stealing ledger (owner-written telemetry for victim selection) plus one steal
    // inbox per thread for the split protocol itself.
    let work_registry: Arc<Vec<WorkSlice>> = Arc::new(
        subranges
            .iter()
            .map(|range| WorkSlice {
                start: AtomicU64::new(range.start),
                next: AtomicU64::new(range.start),
                end: AtomicU64::new(range.end),
            })
            .collect(),
    );
    let mut steal_inbox_receivers: Vec<Option<mpsc::UnboundedReceiver<StealRequest>>> = Vec::new();
    let mut steal_inbox_senders: Vec<mpsc::UnboundedSender<StealRequest>> = Vec::new();
    for _ in 0..thread_total {
        let (sender, receiver) = mpsc::unbounded_channel();
        steal_inbox_senders.push(sender);
        steal_inbox_receivers.push(Some(receiver));
    }
    let steal_inboxes: Arc<Vec<mpsc::UnboundedSender<StealRequest>>> =
        Arc::new(steal_inbox_senders);
    *ACTIVE_WORK_LEDGER.lock().unwrap() = Some(work_registry.clone());
    // Coverage journal: every completed assignment records the interval it actually
    // processed, and the end-of-run audit verifies the union covers the requested range.
    // This turns any silent slot loss (whatever the cause) into a loud, precise error.
    let coverage_log: Arc<std::sync::Mutex<Vec<(u64, u64)>>> =
        Arc::new(std::sync::Mutex::new(Vec::new()));
    let overall_start = subranges.first().map(|range| range.start).unwrap_or(0);
    let overall_end = subranges.last().map(|range| range.end).unwrap_or(0);
    let steal_lock = Arc::new(tokio::sync::Mutex::new(()));
    // Always on in threaded forward mode; sequential/reverse runs and single-thread runs
    // have nothing to steal.
    let work_stealing = !sequential && !reverse_mode && thread_total > 1;
    if work_stealing {
        log::info!(
            target: LOG_MODULE,
            "work stealing enabled: finished threads adopt half of the least-progressed thread's remaining work"
        );
    }
    let recycle_monitor = (recycle_pct > 0 && !sequential && thread_total > 1).then(|| {
        let shutdown_flag = shutdown_flag.clone();
        tokio::spawn(async move {
            let sweep = pace().recycle_interval();
            const MIN_STREAM_AGE_MS: u64 = 30_000;
            /// Rolling rotation, not a storm: a uniform clamp puts most of the fleet under
            /// the threshold at once, and recycling everyone simultaneously would zero
            /// throughput. Rotating the worst few per sweep cycles the whole fleet through
            /// fresh connections within a few minutes while the rest keep streaming.
            const MAX_RECYCLES_PER_SWEEP: usize = 16;
            let mut prev_counts: Vec<u64> = vec![0; thread_total];
            let mut primed = false;
            // Benchmark rate: the best 90th-percentile sweep rate observed this run. Using a
            // percentile means ~a tenth of the fleet must sustain a rate before it becomes
            // the bar — one anomalously fast thread can't set it.
            let mut reference: u64 = 0;
            loop {
                sleep(sweep).await;
                if shutdown_flag.load(Ordering::SeqCst) {
                    return;
                }
                let mut rates = vec![0u64; thread_total];
                for (thread, prev) in prev_counts.iter_mut().enumerate() {
                    let total = thread_activity::tx_count(thread);
                    rates[thread] = total.saturating_sub(*prev);
                    *prev = total;
                }
                // The first sweep only seeds the per-thread snapshots.
                if !primed {
                    primed = true;
                    continue;
                }
                let mut moving: Vec<u64> = rates.iter().copied().filter(|&r| r > 0).collect();
                if moving.is_empty() {
                    continue;
                }
                moving.sort_unstable();
                let p90 = moving[(moving.len() - 1) * 9 / 10];
                reference = reference.max(p90);
                let threshold = reference.saturating_mul(recycle_pct) / 100;
                // Worst offenders first, capped per sweep.
                let mut candidates: Vec<(u64, usize)> = rates
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|&(thread, rate)| {
                        !thread_activity::is_finished(thread)
                            && rate < threshold
                            // Give new connections time to prove themselves first.
                            && thread_activity::stream_age_ms(thread)
                                .is_some_and(|age| age >= MIN_STREAM_AGE_MS)
                    })
                    .map(|(thread, rate)| (rate, thread))
                    .collect();
                candidates.sort_unstable();
                let flagged = candidates.len().min(MAX_RECYCLES_PER_SWEEP);
                for &(_, thread) in candidates.iter().take(MAX_RECYCLES_PER_SWEEP) {
                    thread_activity::request_recycle(thread);
                }
                if flagged > 0 {
                    log::info!(
                        target: LOG_MODULE,
                        "recycle monitor: rotating {} of {} threads below {}% of the best observed rate",
                        flagged,
                        candidates.len(),
                        recycle_pct
                    );
                }
            }
        })
    });
    for (thread_index, mut slot_range) in subranges.into_iter().enumerate() {
        if thread_index > 0
            && let Some(grace) = spawn_gate
        {
            wait_for_green_threads(grace, &shutdown_flag, &handles).await;
        }
        let work_registry = work_registry.clone();
        let coverage_log = coverage_log.clone();
        let steal_lock = steal_lock.clone();
        let steal_inboxes = steal_inboxes.clone();
        let mut steal_inbox = steal_inbox_receivers[thread_index]
            .take()
            .expect("steal inbox taken once per thread");
        let error_counts = error_counts.clone();
        let client = client.clone();
        let on_block = on_block.clone();
        let on_tx = on_tx.clone();
        let on_entry = on_entry.clone();
        let on_reward = on_rewards.clone();
        let on_error = on_error.clone();
        let overall_slots_processed = overall_slots_processed.clone();
        let overall_blocks_processed = overall_blocks_processed.clone();
        let overall_transactions_processed = overall_transactions_processed.clone();
        let overall_entries_processed = overall_entries_processed.clone();
        let stats_tracking = stats_tracking.clone();
        let transactions_since_stats = Arc::new(AtomicU64::new(0));
        let blocks_since_stats = Arc::new(AtomicU64::new(0));
        let slots_since_stats = Arc::new(AtomicU64::new(0));
        let last_pulse = Arc::new(AtomicU64::new(0));
        let transactions_since_stats_cloned = transactions_since_stats.clone();
        let blocks_since_stats_cloned = blocks_since_stats.clone();
        let slots_since_stats_cloned = slots_since_stats.clone();
        let last_pulse_cloned = last_pulse.clone();
        let shutdown_flag = shutdown_flag.clone();
        let pending_skipped_slots = pending_skipped_slots.clone();
        let thread_shutdown_rx = shutdown_signal.as_ref().map(|rx| rx.resubscribe());
        let sequential_mode = sequential;
        let reverse_mode_local = reverse_mode;
        let ripget_threads = sequential_download_threads;
        let ripget_buffer_window_bytes = sequential_buffer_window_bytes;
        let ripget_client = shared_ripget_client.clone();

        let handle = tokio::spawn(async move {
            let transactions_since_stats = transactions_since_stats_cloned;
            let blocks_since_stats = blocks_since_stats_cloned;
            let slots_since_stats = slots_since_stats_cloned;
            let last_pulse = last_pulse_cloned;
            let mut shutdown_rx = thread_shutdown_rx;
            let start_time = firehose_start;
            last_pulse.store(
                firehose_start.elapsed().as_nanos() as u64,
                Ordering::Relaxed,
            );
            let log_target = format!("{}::T{:03}", LOG_MODULE, thread_index);
            let mut skip_until_index = None;
            let mut last_emitted_slot = slot_range.start.saturating_sub(1);
            let block_enabled = on_block.is_some();
            let tx_enabled = on_tx.is_some();
            let entry_enabled = on_entry.is_some();
            let reward_enabled = on_reward.is_some();
            let tracking_enabled = stats_tracking.is_some();
            if block_enabled {
                pending_skipped_slots
                    .entry(thread_index)
                    .or_insert_with(|| DashSet::with_hasher(ahash::RandomState::new()));
            }
            let mut last_counted_slot = slot_range.start.saturating_sub(1);
            // Reverse-mode state preserved across retries. `None` for the highest remaining
            // epoch explicitly means "every epoch is complete" — required so completing
            // epoch 0 is distinguishable from epoch 0 still pending.
            let mut reverse_partial_resume: Option<u64> = None;
            let mut reverse_highest_remaining_epoch: Option<u64> = if reverse_mode_local {
                Some(slot_to_epoch(slot_range.end.saturating_sub(1)))
            } else {
                None
            };
            let mut thread_stats = if tracking_enabled {
                Some(ThreadStats {
                    thread_id: thread_index,
                    start_time,
                    finish_time: None,
                    slot_range: slot_range.clone(),
                    initial_slot_range: slot_range.clone(),
                    current_slot: slot_range.start,
                    slots_processed: 0,
                    blocks_processed: 0,
                    leader_skipped_slots: 0,
                    transactions_processed: 0,
                    entries_processed: 0,
                    rewards_processed: 0,
                })
            } else {
                None
            };

            let mut pending_stats = false;
            let mut retry_backoff = RetryBackoff::new();
            // let mut triggered = false;
            while let Err((err, slot)) = async {
                let op_timeout = if sequential_mode {
                    OP_TIMEOUT_SEQUENTIAL
                } else {
                    OP_TIMEOUT
                };
                // Restart boundary is quiescent: answer any steal proposals that arrived
                // while the previous pass was ending.
                if work_stealing {
                    let resume_position = slot_range.start;
                    service_steal_inbox(
                        &mut steal_inbox,
                        &mut slot_range,
                        resume_position,
                        &work_registry[thread_index],
                        &log_target,
                        true,
                    );
                }
                if poll_shutdown(&shutdown_flag, &mut shutdown_rx) {
                    log::info!(
                        target: &log_target,
                        "shutdown requested; terminating firehose thread {}",
                        thread_index
                    );
                    return Ok(());
                }
                // A stats failure happens after the slot's data callbacks have succeeded.
                // Retry that pulse before opening another stream, including when the failed
                // pulse belonged to the final slot of the range or the final reverse epoch.
                if pending_stats && let Some(ref stats) = thread_stats {
                    maybe_emit_stats(
                        stats_tracking.as_ref(),
                        thread_index,
                        stats,
                        &overall_slots_processed,
                        &overall_blocks_processed,
                        &overall_transactions_processed,
                        &overall_entries_processed,
                        &transactions_since_stats,
                        &blocks_since_stats,
                        &slots_since_stats,
                        &last_pulse,
                        start_time,
                    )
                    .await?;
                    pending_stats = false;
                }
                if !reverse_mode_local && slot_range.is_empty() {
                    return Err((FirehoseError::RangeComplete, slot_range.end));
                }
                // Shield fresh connections from the recycle monitor while they warm up.
                thread_activity::note_stream_start(thread_index);
                let lowest_epoch = slot_to_epoch(slot_range.start);
                let highest_epoch = slot_to_epoch(slot_range.end - 1);
                let epoch_range = lowest_epoch..=highest_epoch;
                log::info!(
                    target: &log_target,
                    "slot range: {} (epoch {}) ... {} (epoch {})",
                    slot_range.start,
                    slot_to_epoch(slot_range.start),
                    slot_range.end,
                    slot_to_epoch(slot_range.end)
                );

                log::info!(target: &log_target, "❄️ starting firehose...");

                // for each epoch
                let mut current_slot: Option<u64> = None;
                let epoch_iter: Vec<u64> = if reverse_mode_local {
                    // All epochs already completed across previous retries?
                    let Some(highest_remaining) = reverse_highest_remaining_epoch else {
                        return Ok(());
                    };
                    if highest_remaining < lowest_epoch {
                        return Ok(());
                    }
                    (lowest_epoch..=highest_remaining).rev().collect()
                } else {
                    epoch_range.clone().collect()
                };
                for epoch_num in epoch_iter {
                    if poll_shutdown(&shutdown_flag, &mut shutdown_rx) {
                        log::info!(
                            target: &log_target,
                            "shutdown requested; terminating firehose thread {}",
                            thread_index
                        );
                        return Ok(());
                    }
                    log::info!(target: &log_target, "entering epoch {}", epoch_num);
                    let (epoch_start, epoch_end_inclusive) = epoch_to_slot_range(epoch_num);
                    let local_start = if reverse_mode_local {
                        match reverse_partial_resume {
                            Some(s) if slot_to_epoch(s) == epoch_num => {
                                std::cmp::max(epoch_start, s)
                            }
                            _ => std::cmp::max(slot_range.start, epoch_start),
                        }
                    } else {
                        std::cmp::max(slot_range.start, epoch_start)
                    };
                    let local_end_inclusive =
                        std::cmp::min(slot_range.end.saturating_sub(1), epoch_end_inclusive);
                    if local_start > local_end_inclusive {
                        log::debug!(
                            target: &log_target,
                            "epoch {} has no overlap with thread range ({}..{}), skipping",
                            epoch_num,
                            slot_range.start,
                            slot_range.end
                        );
                        continue;
                    }
                    let use_sequential_stream = sequential_mode && local_start == epoch_start;
                    // Pace the open outside the op timeout: queueing for the permit is not a stall.
                    wait_for_request_slot().await;
                    let stream = match timeout(op_timeout, async {
                        if use_sequential_stream {
                            fetch_epoch_stream_with_options(
                                epoch_num,
                                &client,
                                Some(FetchEpochStreamOptions {
                                    sequential: true,
                                    ripget_threads,
                                    buffer_window_bytes: ripget_buffer_window_bytes,
                                    ripget_client: ripget_client.clone(),
                                }),
                            )
                            .await
                        } else {
                            fetch_epoch_stream(epoch_num, &client).await
                        }
                    })
                    .await
                    {
                        Ok(stream) => stream,
                        Err(_) => {
                            return Err((
                                FirehoseError::OperationTimeout("fetch_epoch_stream"),
                                current_slot.unwrap_or(slot_range.start),
                            ));
                        }
                    };
                    let mut reader = NodeReader::new(stream);

                    let header_fut = reader.read_raw_header();
                    let header = match timeout(op_timeout, header_fut).await {
                        Ok(res) => res
                            .map_err(FirehoseError::ReadHeader)
                            .map_err(|e| (e, current_slot.unwrap_or(slot_range.start)))?,
                        Err(_) => {
                            return Err((
                                FirehoseError::OperationTimeout("read_raw_header"),
                                current_slot.unwrap_or(slot_range.start),
                            ));
                        }
                    };
                    log::debug!(target: &log_target, "read epoch {} header: {:?}", epoch_num, header);

                    let mut previous_blockhash = Hash::default();
                    let mut latest_entry_blockhash = Hash::default();
                    // Reset counters to align to the local epoch slice; prevents boundary slots
                    // from being treated as already-counted after a restart.
                    last_counted_slot = local_start.saturating_sub(1);
                    current_slot = None;
                    if reverse_mode_local {
                        // In reverse mode each epoch is processed forward independently;
                        // the cross-epoch monotonic dedup check would otherwise reject every
                        // slot below the previously processed (higher) epoch's range.
                        last_emitted_slot = local_start.saturating_sub(1);
                    }
                    if tracking_enabled
                        && let Some(ref mut stats) = thread_stats {
                            stats.current_slot = local_start;
                            stats.slot_range.start = local_start;
                        }

                    if local_start > epoch_start {
                        // Seek to the start of `local_start`'s data; the index maps each slot to
                        // the byte range containing all of its nodes (transactions, entries,
                        // rewards, block), and the seek skips forward over missing slots. Errors
                        // are attributed to `local_start` so retries invalidate and resume the
                        // epoch actually being sought. Acquire the global request-spacing permit
                        // before starting the timeout clock: with hundreds of threads the permit
                        // queue alone can exceed the op timeout, and that wait is pacing, not a
                        // stall.
                        reader.prime_seek_permit().await;
                        let seek_fut = reader.seek_to_slot(local_start);
                        match timeout(op_timeout, seek_fut).await {
                            Ok(res) => res.map_err(|e| (e, local_start))?,
                            Err(_) => {
                                return Err((
                                    FirehoseError::OperationTimeout("seek_to_slot"),
                                    local_start,
                                ));
                            }
                        }
                    }

                    // for each item in each block
                    let mut item_index = 0;
                    let mut displayed_skip_message = false;
                    loop {
                        if poll_shutdown(&shutdown_flag, &mut shutdown_rx) {
                            log::info!(
                                target: &log_target,
                                "shutdown requested; terminating firehose thread {}",
                                thread_index
                            );
                            return Ok(());
                        }
                        if thread_activity::take_recycle(thread_index) {
                            log::info!(
                                target: &log_target,
                                "recycling connection to refresh throughput"
                            );
                            return Err((
                                FirehoseError::ConnectionRecycled,
                                current_slot
                                    .map(|slot| slot.saturating_add(1))
                                    .unwrap_or(slot_range.start),
                            ));
                        }
                        let read_fut = reader.read_until_block();
                        let nodes = match timeout(op_timeout, read_fut).await {
                            Ok(result) => result
                                .map_err(FirehoseError::ReadUntilBlockError)
                                .map_err(|e| {
                                    (
                                        e,
                                        current_slot
                                            .map(|slot| slot.saturating_add(1))
                                            .unwrap_or(slot_range.start),
                                    )
                                })?,
                            Err(_) => {
                                log::warn!(target: &log_target, "timeout reading next block, retrying (will restart)...");
                                return Err((FirehoseError::OperationTimeout("read_until_block"), current_slot.map(|s| s + 1).unwrap_or(slot_range.start)));
                            }
                        };
                        thread_activity::note(thread_index);
                        // Quiescent point: no emission is in flight between batches, so this
                        // is where steal proposals are answered. A grant shrinks
                        // `slot_range.end`, and the `slot >= slot_range.end` guard below
                        // completes the range before any out-of-range data is emitted.
                        if work_stealing {
                            service_steal_inbox(
                                &mut steal_inbox,
                                &mut slot_range,
                                last_counted_slot.saturating_add(1),
                                &work_registry[thread_index],
                                &log_target,
                                true,
                            );
                        }
                        let stream_ended = nodes.is_empty()
                            || nodes
                                .0
                                .last()
                                .is_some_and(|last_node| !last_node.get_node().is_block());
                        if stream_ended {
                            // EOF is ambiguous: it can mean the genuine end of the epoch's
                            // data, or a connection the CDN closed mid-transfer. Consult the
                            // slot index: if any present slot remains in this thread's slice
                            // of the epoch, the stream was truncated and completing here
                            // would silently drop those slots.
                            let scan_end = local_end_inclusive.min(slot_range.end.saturating_sub(1));
                            if let Some(missing) =
                                crate::jetstreamer_firehose::index::next_present_slot(last_counted_slot, scan_end).await
                            {
                                log::warn!(
                                    target: &log_target,
                                    "stream ended prematurely in epoch {} — slot {} (and possibly more) still unprocessed; restarting",
                                    epoch_num,
                                    missing
                                );
                                return Err((
                                    FirehoseError::PrematureStreamEnd,
                                    last_counted_slot.saturating_add(1),
                                ));
                            }
                            log::info!(
                                target: &log_target,
                                "reached end of epoch {}",
                                epoch_num
                            );
                            break;
                        }
                        let block = nodes
                            .get_block()
                            .map_err(FirehoseError::GetBlockError)
                            .map_err(|e| (e, current_slot.unwrap_or(slot_range.start)))?;
                        log::debug!(
                            target: &log_target,
                            "read {} items from epoch {}, now at slot {}",
                            item_index,
                            epoch_num,
                            block.slot
                        );
                        let slot = block.slot;
                        if slot > local_end_inclusive {
                            log::debug!(
                                target: &log_target,
                                "reached end of local slice at slot {} (epoch {}), stopping",
                                slot,
                                epoch_num
                            );
                            break;
                        }
                        if slot >= slot_range.end {
                            log::info!(target: &log_target, "reached end of slot range at slot {}", slot);
                            // We use >= because slot_range is half-open [start, end), so any
                            // slot equal to end is out-of-range and must not be processed. Do
                            // not emit synthetic skipped slots here; another thread may own the
                            // boundary. In reverse mode we still have lower epochs to process,
                            // so just break out of this epoch's inner loop.
                            if reverse_mode_local {
                                break;
                            }
                            if block_enabled {
                                pending_skipped_slots.remove(&thread_index);
                            }
                            return Err((FirehoseError::RangeComplete, slot_range.end));
                        }
                        debug_assert!(slot < slot_range.end, "processing out-of-range slot {} (end {})", slot, slot_range.end);
                        if slot < slot_range.start {
                            if slot.saturating_add(1) == slot_range.start {
                                log::debug!(
                                    target: &log_target,
                                    "priming reader with preceding slot {}, skipping",
                                    slot
                                );
                            } else {
                                log::warn!(
                                    target: &log_target,
                                    "encountered slot {} before start of range {}, skipping",
                                    slot,
                                    slot_range.start
                                );
                            }
                            continue;
                        }
                        current_slot = Some(slot);
                        let mut entry_index: usize = 0;
                        let mut this_block_executed_transaction_count: u64 = 0;
                        let mut this_block_entry_count: u64 = 0;
                        let mut this_block_rewards = DecodedRewards::empty();

                        for node_with_cid in &nodes.0 {
                            item_index += 1;
                            if let Some(skip) = skip_until_index {
                                if item_index < skip {
                                    if !displayed_skip_message {
                                        log::info!(
                                            target: &log_target,
                                            "skipping until index {} (at {})",
                                            skip,
                                            item_index
                                        );
                                        displayed_skip_message = true;
                                    }
                                    continue;
                                } else {
                                    log::info!(
                                        target: &log_target,
                                        "reached target index {}, resuming...",
                                        skip
                                    );
                                    skip_until_index = None;
                                }
                            }
                            let node = node_with_cid.get_node();

                            if let Some(ref mut stats) = thread_stats {
                                stats.current_slot = slot;
                            }

                            let error_slot = current_slot.unwrap_or(slot_range.start);

                            use crate::jetstreamer_firehose::node::Node::*;
                            match node {
                                Transaction(tx) => {
                                    if tx_enabled
                                        && let Some(on_tx_cb) = on_tx.as_ref()
                                    {
                                        let error_slot = current_slot.unwrap_or(slot_range.start);
                                        let versioned_tx = tx.as_parsed().map_err(|err| {
                                            (
                                                FirehoseError::NodeDecodingError(item_index, err),
                                                error_slot,
                                            )
                                        })?;
                                        let reassembled_metadata = nodes
                                            .reassemble_dataframes(&tx.metadata)
                                            .map_err(|err| {
                                                (
                                                    FirehoseError::NodeDecodingError(item_index, err),
                                                    error_slot,
                                                )
                                            })?;

                                        let as_native_metadata = decode_transaction_status_meta_from_frame(
                                            block.slot,
                                            reassembled_metadata,
                                        )
                                        .map_err(|err| {
                                            (
                                                FirehoseError::NodeDecodingError(item_index, err),
                                                error_slot,
                                            )
                                        })?;

                                        let message_hash = {
                                            #[cfg(feature = "verify-transaction-signatures")]
                                            {
                                                versioned_tx.verify_and_hash_message().map_err(|err| {
                                                    (
                                                        FirehoseError::TransactionHandlerError(Box::new(err)),
                                                        error_slot,
                                                    )
                                                })?
                                            }
                                            #[cfg(not(feature = "verify-transaction-signatures"))]
                                            {
                                                versioned_tx.message.hash()
                                            }
                                        };
                                        let signature = versioned_tx
                                            .signatures
                                            .first()
                                            .ok_or_else(|| {
                                                Box::new(std::io::Error::new(
                                                    std::io::ErrorKind::InvalidData,
                                                    "transaction missing signature",
                                                )) as SharedError
                                            })
                                            .map_err(|err| {
                                                (
                                                    FirehoseError::NodeDecodingError(
                                                        item_index,
                                                        err,
                                                    ),
                                                    error_slot,
                                                )
                                            })?;
                                        let is_vote = is_simple_vote_transaction(&versioned_tx);

                                        on_tx_cb(
                                            thread_index,
                                            TransactionData {
                                                slot: block.slot,
                                                transaction_slot_index: tx.index.unwrap() as usize,
                                                signature: *signature,
                                                message_hash,
                                                is_vote,
                                                transaction_status_meta: as_native_metadata,
                                                transaction: versioned_tx,
                                            },
                                        )
                                        .await
                                        .map_err(|e| {
                                            (
                                                FirehoseError::TransactionHandlerError(e),
                                                error_slot,
                                            )
                                        })?;
                                    }
                                    fetch_add_if(
                                        tracking_enabled,
                                        &overall_transactions_processed,
                                        1,
                                    );
                                    if let Some(ref mut stats) = thread_stats {
                                        stats.transactions_processed += 1;
                                    }
                                    transactions_since_stats.fetch_add(1, Ordering::Relaxed);
                                    thread_activity::add_transactions(thread_index, 1);
                                }
                                Entry(entry) => {
                                    let entry_hash = Hash::from(entry.hash.to_bytes());
                                    let entry_transaction_count = entry.transactions.len();
                                    let entry_transaction_count_u64 = entry_transaction_count as u64;
                                    let starting_transaction_index_u64 =
                                        this_block_executed_transaction_count;
                                    latest_entry_blockhash = entry_hash;
                                    this_block_executed_transaction_count += entry_transaction_count_u64;
                                    this_block_entry_count += 1;

                                    if entry_enabled && let Some(on_entry_cb) = on_entry.as_ref() {
                                        let starting_transaction_index = usize::try_from(
                                            starting_transaction_index_u64,
                                        )
                                        .map_err(|err| {
                                            (
                                                FirehoseError::EntryHandlerError(Box::new(err)),
                                                error_slot,
                                            )
                                        })?;
                                        let transaction_indexes_end =
                                            starting_transaction_index + entry_transaction_count;
                                        on_entry_cb(
                                            thread_index,
                                            EntryData {
                                                slot: block.slot,
                                                entry_index,
                                                transaction_indexes: starting_transaction_index
                                                    ..transaction_indexes_end,
                                                num_hashes: entry.num_hashes,
                                                hash: entry_hash,
                                            },
                                        )
                                        .await
                                        .map_err(|e| {
                                            (
                                                FirehoseError::EntryHandlerError(e),
                                                error_slot,
                                            )
                                        })?;
                                    }
                                    entry_index += 1;
                                    fetch_add_if(
                                        tracking_enabled,
                                        &overall_entries_processed,
                                        1,
                                    );
                                    if let Some(ref mut stats) = thread_stats {
                                        stats.entries_processed += 1;
                                    }
                                }
                                Block(block) => {
                                    let prev_last_counted_slot = last_counted_slot;

                                    let next_expected_slot = prev_last_counted_slot.saturating_add(1);
                                    let skip_start_from_previous = last_counted_slot.saturating_add(1);
                                    let skip_start = skip_start_from_previous.max(next_expected_slot);

                                    let skipped_epoch = slot_to_epoch(last_counted_slot);
                                    for skipped_slot in skip_start..slot {
                                        if slot_to_epoch(skipped_slot) != skipped_epoch {
                                            break;
                                        }
                                        log::debug!(
                                            target: &log_target,
                                            "leader skipped slot {} (prev_counted {}, current slot {})",
                                            skipped_slot,
                                            prev_last_counted_slot,
                                            slot,
                                        );
                                        if block_enabled {
                                            pending_skipped_slots
                                                .entry(thread_index)
                                                .or_default()
                                                .insert(skipped_slot);
                                        }
                                        if block_enabled
                                            && let Some(on_block_cb) = on_block.as_ref()
                                            && skipped_slot > last_emitted_slot {
                                                on_block_cb(
                                                    thread_index,
                                                    BlockData::PossibleLeaderSkipped {
                                                        slot: skipped_slot,
                                                    },
                                                )
                                                .await
                                                .map_err(|e| {
                                                    (
                                                        FirehoseError::BlockHandlerError(e),
                                                        skipped_slot,
                                                    )
                                                })?;
                                                last_emitted_slot = skipped_slot;
                                            }
                                        if tracking_enabled {
                                            overall_slots_processed.fetch_add(1, Ordering::Relaxed);
                                            slots_since_stats.fetch_add(1, Ordering::Relaxed);
                                            if let Some(ref mut stats) = thread_stats {
                                                stats.leader_skipped_slots += 1;
                                                stats.slots_processed += 1;
                                                stats.current_slot = skipped_slot;
                                            }
                                        }
                                        last_counted_slot = skipped_slot;
                                    }

                                    let cleared_pending_skip = if block_enabled {
                                        clear_pending_skip(
                                            &pending_skipped_slots,
                                            thread_index,
                                            slot,
                                        )
                                    } else {
                                        false
                                    };

                                    if slot <= last_counted_slot && !cleared_pending_skip {
                                        log::debug!(
                                            target: &log_target,
                                            "duplicate block {}, already counted (last_counted={})",
                                            slot,
                                            last_counted_slot,
                                        );
                                        this_block_rewards = DecodedRewards::empty();
                                        continue;
                                    }

                                    if block_enabled {
                                        if let Some(on_block_cb) = on_block.as_ref() {
                                            let DecodedRewards {
                                                rewards,
                                                commission_rate_in_basis_points,
                                            } = std::mem::take(&mut this_block_rewards);
                                            if slot > last_emitted_slot {
                                                on_block_cb(
                                                    thread_index,
                                                    BlockData::Block {
                                                        parent_slot: block.meta.parent_slot,
                                                        parent_blockhash: previous_blockhash,
                                                        slot: block.slot,
                                                        blockhash: latest_entry_blockhash,
                                                        rewards,
                                                        commission_rate_in_basis_points,
                                                        block_time: Some(block.meta.blocktime as i64),
                                                        block_height: block.meta.block_height,
                                                        executed_transaction_count:
                                                            this_block_executed_transaction_count,
                                                        entry_count: this_block_entry_count,
                                                        block_markers: block
                                                            .meta
                                                            .block_markers
                                                            .clone(),
                                                        block_id: block
                                                            .meta
                                                            .block_id
                                                            .map(Hash::from),
                                                    },
                                                )
                                                .await
                                                .map_err(|e| {
                                                    (
                                                        FirehoseError::BlockHandlerError(e),
                                                        error_slot,
                                                    )
                                                })?;
                                                last_emitted_slot = slot;
                                            }
                                        }
                                    } else {
                                        this_block_rewards = DecodedRewards::empty();
                                    }
                                    previous_blockhash = latest_entry_blockhash;

                                    // Data delivery is complete. A later stats error must resume
                                    // after this slot so transactions cannot outlive their block callback.
                                    if slot > last_counted_slot {
                                        last_counted_slot = slot;
                                    }
                                    if work_stealing {
                                        work_registry[thread_index]
                                            .next
                                            .store(last_counted_slot.saturating_add(1), Ordering::SeqCst);
                                    }

                                    if tracking_enabled {
                                        overall_slots_processed.fetch_add(1, Ordering::Relaxed);
                                        overall_blocks_processed.fetch_add(1, Ordering::Relaxed);
                                        slots_since_stats.fetch_add(1, Ordering::Relaxed);
                                        blocks_since_stats.fetch_add(1, Ordering::Relaxed);
                                        if let Some(ref mut stats) = thread_stats {
                                            stats.blocks_processed += 1;
                                            stats.slots_processed += 1;
                                            stats.current_slot = slot;
                                        }

                                        if let (Some(stats_tracking_cfg), Some(thread_stats_ref)) =
                                            (&stats_tracking, thread_stats.as_mut())
                                            && slot % stats_tracking_cfg.tracking_interval_slots == 0 {
                                            pending_stats = true;
                                            maybe_emit_stats(
                                                stats_tracking.as_ref(),
                                                thread_index,
                                                thread_stats_ref,
                                                &overall_slots_processed,
                                                &overall_blocks_processed,
                                                &overall_transactions_processed,
                                                &overall_entries_processed,
                                                &transactions_since_stats,
                                                &blocks_since_stats,
                                                &slots_since_stats,
                                                &last_pulse,
                                                start_time,
                                            )
                                            .await?;
                                            pending_stats = false;
                                        }
                                    }
                                }
                                Subset(_subset) => (),
                                Epoch(_epoch) => (),
                                Rewards(rewards) => {
                                    if reward_enabled || block_enabled {
                                        let reassembled = nodes
                                            .reassemble_dataframes(&rewards.data)
                                            .map_err(|err| {
                                                (
                                                    FirehoseError::NodeDecodingError(item_index, err),
                                                    current_slot.unwrap_or(slot_range.start),
                                                )
                                            })?;
                                        if reassembled.is_empty() {
                                            this_block_rewards = DecodedRewards::empty();
                                            if reward_enabled
                                                && let Some(on_reward_cb) = on_reward.as_ref()
                                            {
                                                on_reward_cb(
                                                    thread_index,
                                                    RewardsData {
                                                        slot: block.slot,
                                                        rewards: Vec::new(),
                                                    },
                                                )
                                                .await
                                                .map_err(|e| {
                                                    (
                                                        FirehoseError::RewardHandlerError(e),
                                                        error_slot,
                                                    )
                                                })?;
                                            }
                                            continue;
                                        }

                                        let decoded_rewards =
                                            decode_rewards_from_frame(block.slot, reassembled)
                                                .map_err(|err| {
                                                    (
                                                        FirehoseError::NodeDecodingError(
                                                            item_index,
                                                            err,
                                                        ),
                                                        error_slot,
                                                    )
                                                })?;
                                        if reward_enabled
                                            && let Some(on_reward_cb) = on_reward.as_ref()
                                        {
                                            on_reward_cb(
                                                thread_index,
                                                RewardsData {
                                                    slot: block.slot,
                                                    rewards: decoded_rewards.to_handler_rewards(),
                                                },
                                            )
                                            .await
                                            .map_err(|e| {
                                                (
                                                    FirehoseError::RewardHandlerError(e),
                                                    error_slot,
                                                )
                                            })?;
                                        }
                                        this_block_rewards = decoded_rewards;
                                        if let Some(ref mut stats) = thread_stats {
                                            stats.rewards_processed +=
                                                this_block_rewards.rewards.keyed_rewards.len() as u64;
                                        }
                                    }
                                }
                                DataFrame(_data_frame) => (),
                            }
                        }
                        if !reverse_mode_local && block.slot == slot_range.end - 1 {
                            let finish_time = std::time::Instant::now();
                            let elapsed = finish_time.duration_since(start_time);
                            log::info!(target: &log_target, "processed slot {}", block.slot);
                            let elapsed_pretty = human_readable_duration(elapsed);
                            log::info!(
                                target: &log_target,
                                "processed {} slots across {} epochs in {}.",
                                slot_range.end - slot_range.start,
                                slot_to_epoch(slot_range.end) + 1 - slot_to_epoch(slot_range.start),
                                elapsed_pretty
                            );
                            log::info!(target: &log_target, "❄️ firehose thread completed its work.");
                            // On completion, report threads with non-zero error counts for
                            // visibility.
                            let summary: String = error_counts
                                .iter()
                                .enumerate()
                                .filter_map(|(i, c)| {
                                    let v = c.load(Ordering::Relaxed);
                                    if v > 0 {
                                        Some(format!("{:03}({})", i, v))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(", ");
                            if !summary.is_empty() {
                                log::debug!(target: &log_target, "threads with errors: {}", summary);
                            }
                            return Err((FirehoseError::RangeComplete, slot_range.end));
                        }
                    }
                    if reverse_mode_local {
                        // Mark this epoch as fully processed so retries skip it
                        // (`checked_sub` yields `None` at epoch 0: nothing remains).
                        if reverse_highest_remaining_epoch == Some(epoch_num) {
                            reverse_highest_remaining_epoch = epoch_num.checked_sub(1);
                        }
                        if matches!(
                            reverse_partial_resume,
                            Some(s) if slot_to_epoch(s) == epoch_num
                        ) {
                            reverse_partial_resume = None;
                        }
                    }
                    if let Some(expected_last_slot) = slot_range.end.checked_sub(1)
                        && last_counted_slot < expected_last_slot
                    {
                        // Do not synthesize skipped slots during final flush; another thread may
                        // cover the remaining range (especially across epoch boundaries).
                    }
                    if let Some(ref mut stats) = thread_stats {
                        stats.finish_time = Some(std::time::Instant::now());
                        pending_stats = true;
                        maybe_emit_stats(
                            stats_tracking.as_ref(),
                            thread_index,
                            stats,
                            &overall_slots_processed,
                            &overall_blocks_processed,
                            &overall_transactions_processed,
                            &overall_entries_processed,
                            &transactions_since_stats,
                            &blocks_since_stats,
                            &slots_since_stats,
                            &last_pulse,
                            start_time,
                        )
                        .await?;
                        pending_stats = false;
                    }
                    if block_enabled {
                        pending_skipped_slots.remove(&thread_index);
                    }
                    log::info!(target: &log_target, "thread {} has finished its work", thread_index);
                    }
                    Err((FirehoseError::RangeComplete, slot_range.end))
            }
            .await
            {
                if is_shutdown_error(&err) {
                    log::info!(
                        target: &log_target,
                        "shutdown requested; terminating firehose thread {}",
                        thread_index
                    );
                    break;
                }
                // Range completion arrives through the retry loop so the thread can adopt
                // stolen work (restarting the loop with a new range) or retire.
                if matches!(err, FirehoseError::RangeComplete) {
                    // Journal the interval this assignment actually processed (read before
                    // a steal adoption overwrites the ledger entry). Trailing slots between
                    // last_counted and the assignment end are left to the audit, which
                    // verifies such gaps contain no present slots.
                    if work_stealing {
                        let assignment_start =
                            work_registry[thread_index].start.load(Ordering::SeqCst);
                        coverage_log
                            .lock()
                            .unwrap()
                            .push((assignment_start, last_counted_slot.saturating_add(1)));
                    }
                    // This thread is done with its own range: publish "nothing remaining" so
                    // hunters stop targeting it, then drain any pending steal proposals with
                    // a refusal before going hunting itself.
                    if work_stealing {
                        work_registry[thread_index]
                            .next
                            .store(slot_range.end, Ordering::SeqCst);
                        let drained_position = slot_range.end;
                        service_steal_inbox(
                            &mut steal_inbox,
                            &mut slot_range,
                            drained_position,
                            &work_registry[thread_index],
                            &log_target,
                            false,
                        );
                    }
                    if work_stealing
                        && let Some((victim, stolen)) = request_steal(
                            &work_registry,
                            &steal_inboxes,
                            &mut steal_inbox,
                            thread_index,
                            &steal_lock,
                        )
                        .await
                    {
                        thread_activity::note_steal();
                        log::info!(
                            target: &log_target,
                            "🐾 stole {} slots ({}..{}) from thread {} (least progress)",
                            stolen.end - stolen.start,
                            stolen.start,
                            stolen.end,
                            victim
                        );
                        thread_activity::clear_finished(thread_index);
                        slot_range = stolen;
                        last_counted_slot = slot_range.start.saturating_sub(1);
                        last_emitted_slot = slot_range.start.saturating_sub(1);
                        reverse_partial_resume = None;
                        skip_until_index = None;
                        if let Some(ref mut stats) = thread_stats {
                            stats.slot_range = slot_range.clone();
                            stats.finish_time = None;
                        }
                        continue;
                    }
                    thread_activity::note_finished(thread_index);
                    break;
                }
                // A deliberate connection recycle is a clean restart, not a failure: skip the
                // error logging, error counter, on_error callback, and retry backoff.
                let recycled = matches!(err, FirehoseError::ConnectionRecycled);
                let epoch = slot_to_epoch(slot);
                let item_index = match &err {
                    FirehoseError::NodeDecodingError(item_index, _) => *item_index,
                    _ => 0,
                };
                let error_message = err.to_string();
                if recycled {
                    thread_activity::note_recycle();
                    log::info!(
                        target: &log_target,
                        "🧊 recycling connection; resuming from slot {} in epoch {}",
                        slot,
                        epoch
                    );
                } else {
                    if matches!(err, FirehoseError::OperationTimeout(_)) {
                        thread_activity::note_timeout();
                    }
                    log::error!(
                        target: &log_target,
                        "⚠️🧊 firehose encountered an error at slot {} in epoch {} and will retry:",
                        slot,
                        epoch
                    );
                    log::error!(target: &log_target, "{}", error_message);
                }
                if matches!(err, FirehoseError::SlotOffsetIndexError(_))
                    || error_message.contains("Unknown CID version")
                {
                    // Clear cached index data for this epoch to avoid retrying with a bad/partial index
                    // (or a bad seek offset that landed mid-stream).
                    SLOT_OFFSET_INDEX.invalidate_epoch(epoch);
                }
                if !recycled {
                    if let Some(on_error_cb) = on_error.clone() {
                        let context = FirehoseErrorContext {
                            thread_id: thread_index,
                            slot,
                            epoch,
                            error_message: error_message.clone(),
                        };
                        if let Err(handler_err) = on_error_cb(thread_index, context).await {
                            log::error!(
                                target: &log_target,
                                "on_error handler failed: {}",
                                handler_err
                            );
                        }
                    }
                    // Increment this thread's error counter
                    error_counts[thread_index].fetch_add(1, Ordering::Relaxed);
                    log::warn!(
                        target: &log_target,
                        "restarting from slot {} at index {}",
                        slot,
                        item_index,
                    );
                }
                // Data callback failures resume at the incomplete slot. Stats failures refer
                // to an already delivered slot, so data resumes at the following slot.
                if reverse_mode_local {
                    // In reverse mode, completed higher epochs are tracked via
                    // reverse_highest_remaining_epoch and the within-epoch resume slot lives in
                    // reverse_partial_resume; slot_range stays at its original bounds.
                    let (resume, highest) = reverse_resume_after_error(
                        slot,
                        last_counted_slot,
                        reverse_highest_remaining_epoch,
                    );
                    reverse_partial_resume = resume;
                    reverse_highest_remaining_epoch = highest;
                } else if slot <= last_counted_slot {
                    slot_range.start = last_counted_slot.saturating_add(1);
                } else {
                    slot_range.start = slot;
                }
                // Reset pulse timer to exclude downtime from next rate calc.
                last_pulse.store(start_time.elapsed().as_nanos() as u64, Ordering::Relaxed);
                if tracking_enabled
                    && let Some(ref mut stats_ref) = thread_stats {
                        stats_ref.slot_range.start = slot_range.start;
                        stats_ref.slot_range.end = slot_range.end;
                        // initial_slot_range remains unchanged for progress reporting.
                    }
                if block_enabled {
                    pending_skipped_slots.remove(&thread_index);
                }
                // `skip_until_index` is unsafe across retries because `item_index`
                // is reset to 0 each epoch restart. Keeping it can skip large portions
                // of the stream and silently drop slots.
                skip_until_index = None;
                if !recycled {
                    let backoff = retry_backoff.next_delay(slot);
                    log::warn!(
                        target: &log_target,
                        "backing off {:?} before restarting",
                        backoff
                    );
                    // Sleep in slices so the thread stays responsive to shutdown (and to
                    // steal proposals — a backing-off thread is quiescent and often the
                    // least-progressed, so it is a prime steal victim) during the wait.
                    let deadline = std::time::Instant::now() + backoff;
                    let mut shutdown_requested = false;
                    while std::time::Instant::now() < deadline {
                        if poll_shutdown(&shutdown_flag, &mut shutdown_rx) {
                            shutdown_requested = true;
                            break;
                        }
                        if work_stealing {
                            let resume_position = slot_range.start;
                            service_steal_inbox(
                                &mut steal_inbox,
                                &mut slot_range,
                                resume_position,
                                &work_registry[thread_index],
                                &log_target,
                                true,
                            );
                        }
                        sleep(std::time::Duration::from_millis(250)).await;
                    }
                    if shutdown_requested {
                        log::info!(
                            target: &log_target,
                            "shutdown requested; terminating firehose thread {}",
                            thread_index
                        );
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.await.unwrap();
    }
    if let Some(monitor) = recycle_monitor {
        monitor.abort();
    }
    // End-of-run coverage audit: the union of journaled intervals must cover every present
    // slot in the requested range. Skipped when the run was interrupted (holes are expected
    // then) or when work stealing (and thus journaling) was inactive.
    if work_stealing && !shutdown_flag.load(Ordering::SeqCst) {
        let mut covered = coverage_log.lock().unwrap().clone();
        covered.retain(|(start, end)| end > start);
        covered.sort_unstable();
        let mut holes: Vec<(u64, u64)> = Vec::new();
        let mut cursor = overall_start;
        for (start, end) in covered {
            if start > cursor {
                holes.push((cursor, start));
            }
            cursor = cursor.max(end);
        }
        if cursor < overall_end {
            holes.push((cursor, overall_end));
        }
        let mut real_holes = 0usize;
        for (hole_start, hole_end) in &holes {
            if let Some(missing) = crate::jetstreamer_firehose::index::next_present_slot(
                hole_start.saturating_sub(1),
                hole_end.saturating_sub(1),
            )
            .await
            {
                real_holes += 1;
                if real_holes <= 10 {
                    log::error!(
                        target: LOG_MODULE,
                        "🕳️ coverage audit: slots [{}, {}) were never processed (first present slot: {})",
                        hole_start,
                        hole_end,
                        missing
                    );
                }
            }
        }
        if real_holes == 0 {
            log::info!(
                target: LOG_MODULE,
                "coverage audit passed: every present slot in [{}, {}) was processed",
                overall_start,
                overall_end
            );
        } else {
            log::error!(
                target: LOG_MODULE,
                "🕳️ coverage audit FAILED: {} hole(s) containing unprocessed slots — output is incomplete; re-run the listed ranges",
                real_holes
            );
        }
    }
    if stats_tracking.is_some() {
        let elapsed = firehose_start.elapsed();
        let elapsed_secs = elapsed.as_secs_f64();
        let total_slots = overall_slots_processed.load(Ordering::Relaxed);
        let total_blocks = overall_blocks_processed.load(Ordering::Relaxed);
        let total_transactions = overall_transactions_processed.load(Ordering::Relaxed);
        let total_leader_skipped = total_slots.saturating_sub(total_blocks);
        let total_errors: u64 = error_counts
            .iter()
            .map(|counter| counter.load(Ordering::Relaxed) as u64)
            .sum();
        let overall_tps = if elapsed_secs > 0.0 {
            total_transactions as f64 / elapsed_secs
        } else {
            0.0
        };
        log::info!(
            target: LOG_MODULE,
            "firehose summary: elapsed={:.2}s, slots={}, blocks={}, leader_skipped={}, transactions={}, overall_tps={:.2}, total_errors={}",
            elapsed_secs,
            total_slots,
            total_blocks,
            total_leader_skipped,
            total_transactions,
            overall_tps,
            total_errors
        );
    }
    if shutdown_flag.load(Ordering::SeqCst) {
        log::info!(target: LOG_MODULE, "firehose shutdown complete; all threads exited cleanly.");
    } else {
        log::info!(target: LOG_MODULE, "❄️ firehose finished successfully.");
    }
    Ok(())
}

/// Builds a Geyser-backed firehose and returns a slot notification stream.
///
/// Requires the `geyser` feature. Loads Agave Geyser plugins in-process and replays archived
/// blocks through their notifier interfaces. The provided `slot_range` is treated as a
/// half-open interval `[start, end)`, and the thread will restart from the last processed
/// slot on recoverable errors to maintain coverage.
#[cfg(feature = "geyser")]
#[allow(clippy::result_large_err)]
pub fn firehose_geyser(
    rt: Arc<tokio::runtime::Runtime>,
    slot_range: Range<u64>,
    geyser_config_files: Option<&[PathBuf]>,
    index_base_url: &Url,
    client: &Client,
    on_load: impl Future<Output = Result<(), SharedError>> + Send + 'static,
    threads: u64,
) -> Result<Receiver<SlotNotification>, (FirehoseError, u64)> {
    if threads == 0 {
        return Err((
            FirehoseError::OnLoadError("Number of threads must be greater than 0".into()),
            slot_range.start,
        ));
    }
    log::info!(target: LOG_MODULE, "starting firehose...");
    log::info!(target: LOG_MODULE, "index base url: {}", index_base_url);
    let (confirmed_bank_sender, confirmed_bank_receiver) = unbounded();
    let mut entry_notifier_maybe = None;
    let mut block_meta_notifier_maybe = None;
    let mut transaction_notifier_maybe = None;
    if let Some(geyser_config_files) = geyser_config_files {
        log::debug!(target: LOG_MODULE, "geyser config files: {:?}", geyser_config_files);

        let service =
            solana_geyser_plugin_manager::geyser_plugin_service::GeyserPluginService::new(
                confirmed_bank_receiver.clone(),
                true,
                geyser_config_files,
            )
            .map_err(|e| (e.into(), slot_range.start))?;

        transaction_notifier_maybe = Some(
            service
                .get_transaction_notifier()
                .ok_or(FirehoseError::FailedToGetTransactionNotifier)
                .map_err(|e| (e, slot_range.start))?,
        );

        entry_notifier_maybe = service.get_entry_notifier();
        block_meta_notifier_maybe = service.get_block_metadata_notifier();

        log::debug!(target: LOG_MODULE, "geyser plugin service initialized.");
    }

    if entry_notifier_maybe.is_some() {
        log::debug!(target: LOG_MODULE, "entry notifications enabled")
    } else {
        log::debug!(target: LOG_MODULE, "none of the plugins have enabled entry notifications")
    }
    log::info!(target: LOG_MODULE, "running on_load...");
    rt.spawn(on_load);

    let slot_range = Arc::new(slot_range);
    let transaction_notifier_maybe = Arc::new(transaction_notifier_maybe);
    let entry_notifier_maybe = Arc::new(entry_notifier_maybe);
    let block_meta_notifier_maybe = Arc::new(block_meta_notifier_maybe);
    let confirmed_bank_sender = Arc::new(confirmed_bank_sender);

    // divide slot_range into n subranges
    let subranges = generate_subranges(&slot_range, threads);
    if threads > 1 {
        log::info!(target: LOG_MODULE, "❄️ thread sub-ranges: {:?}", subranges);
    }

    let mut handles = Vec::new();
    // Shared per-thread error counters
    let error_counts: Arc<Vec<AtomicU32>> =
        Arc::new((0..subranges.len()).map(|_| AtomicU32::new(0)).collect());

    for (i, slot_range) in subranges.into_iter().enumerate() {
        let transaction_notifier_maybe = (*transaction_notifier_maybe).clone();
        let entry_notifier_maybe = (*entry_notifier_maybe).clone();
        let block_meta_notifier_maybe = (*block_meta_notifier_maybe).clone();
        let confirmed_bank_sender = (*confirmed_bank_sender).clone();
        let client = client.clone();
        let error_counts = error_counts.clone();

        let rt_clone = rt.clone();

        let handle = std::thread::spawn(move || {
            rt_clone.block_on(async {
                firehose_geyser_thread(
                    slot_range,
                    transaction_notifier_maybe,
                    entry_notifier_maybe,
                    block_meta_notifier_maybe,
                    confirmed_bank_sender,
                    &client,
                    if threads > 1 { Some(i) } else { None },
                    error_counts,
                )
                .await
                .unwrap();
            });
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    log::info!(target: LOG_MODULE, "❄️ firehose finished successfully.");
    if let Some(block_meta_notifier) = block_meta_notifier_maybe.as_ref() {
        block_meta_notifier.notify_block_metadata(
            u64::MAX,
            "unload",
            u64::MAX,
            u64::MAX,
            "unload",
            &KeyedRewardsAndNumPartitions {
                keyed_rewards: vec![],
                num_partitions: None,
            },
            None,
            None,
            0,
            0,
            false,
        );
    }
    Ok(confirmed_bank_receiver)
}

#[cfg(feature = "geyser")]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::result_large_err)]
async fn firehose_geyser_thread(
    mut slot_range: Range<u64>,
    transaction_notifier_maybe: Option<Arc<dyn TransactionNotifier + Send + Sync + 'static>>,
    entry_notifier_maybe: Option<Arc<dyn EntryNotifier + Send + Sync + 'static>>,
    block_meta_notifier_maybe: Option<Arc<dyn BlockMetadataNotifier + Send + Sync + 'static>>,
    confirmed_bank_sender: Sender<SlotNotification>,
    client: &Client,
    thread_index: Option<usize>,
    error_counts: Arc<Vec<AtomicU32>>,
) -> Result<(), (FirehoseError, u64)> {
    let start_time = std::time::Instant::now();
    let log_target = if let Some(thread_index) = thread_index {
        format!("{}::T{:03}", LOG_MODULE, thread_index)
    } else {
        LOG_MODULE.to_string()
    };
    let initial_slot_range = slot_range.clone();
    let mut skip_until_index = None;
    let mut last_counted_slot = slot_range.start.saturating_sub(1);
    let mut retry_backoff = RetryBackoff::new();
    // let mut triggered = false;
    while let Err((err, slot)) = async {
            let epoch_range = slot_to_epoch(slot_range.start)..=slot_to_epoch(slot_range.end - 1);
            log::info!(
                target: &log_target,
                "slot range: {} (epoch {}) ... {} (epoch {})",
                slot_range.start,
                slot_to_epoch(slot_range.start),
                slot_range.end,
                slot_to_epoch(slot_range.end)
            );

            log::info!(target: &log_target, "❄️ starting firehose...");

            // for each epoch
            let mut current_slot: Option<u64> = None;
            for epoch_num in epoch_range.clone() {
                log::info!(target: &log_target, "entering epoch {}", epoch_num);
                wait_for_request_slot().await;
                let stream = match timeout(OP_TIMEOUT, fetch_epoch_stream(epoch_num, client)).await {
                    Ok(stream) => stream,
                    Err(_) => {
                        return Err((FirehoseError::OperationTimeout("fetch_epoch_stream"), current_slot.unwrap_or(slot_range.start)));
                    }
                };
                let mut reader = NodeReader::new(stream);

                let header_fut = reader.read_raw_header();
                let header = match timeout(OP_TIMEOUT, header_fut).await {
                    Ok(res) => res
                        .map_err(FirehoseError::ReadHeader)
                        .map_err(|e| (e, current_slot.unwrap_or(slot_range.start)))?,
                    Err(_) => {
                        return Err((FirehoseError::OperationTimeout("read_raw_header"), current_slot.unwrap_or(slot_range.start)));
                    }
                };
                log::debug!(target: &log_target, "read epoch {} header: {:?}", epoch_num, header);

                let (epoch_start, epoch_end_inclusive) = epoch_to_slot_range(epoch_num);
                let local_start = std::cmp::max(slot_range.start, epoch_start);
                let local_end_inclusive =
                    std::cmp::min(slot_range.end.saturating_sub(1), epoch_end_inclusive);
                if local_start > local_end_inclusive {
                    log::debug!(
                        target: &log_target,
                        "epoch {} has no overlap with thread range ({}..{}), skipping",
                        epoch_num,
                        slot_range.start,
                        slot_range.end
                    );
                    continue;
                }

                let mut todo_previous_blockhash = Hash::default();
                let mut todo_latest_entry_blockhash = Hash::default();
                // Reset counters to align to the local epoch slice; prevents boundary slots
                // from being treated as already-counted after a restart.
                last_counted_slot = local_start.saturating_sub(1);
                current_slot = None;

                if local_start > epoch_start {
                    // Seek to the start of `local_start`'s data; the index maps each slot to
                    // the byte range containing all of its nodes (transactions, entries,
                    // rewards, block), and the seek skips forward over missing slots. Errors
                    // are attributed to `local_start` so retries invalidate and resume the
                    // epoch actually being sought. Acquire the global request-spacing permit
                    // before starting the timeout clock: with hundreds of threads the permit
                    // queue alone can exceed the op timeout, and that wait is pacing, not a
                    // stall.
                    reader.prime_seek_permit().await;
                    let seek_fut = reader.seek_to_slot(local_start);
                    match timeout(OP_TIMEOUT, seek_fut).await {
                        Ok(res) => res.map_err(|e| (e, local_start))?,
                        Err(_) => {
                            return Err((
                                FirehoseError::OperationTimeout("seek_to_slot"),
                                local_start,
                            ));
                        }
                    }
                }

                // for each item in each block
                let mut item_index = 0;
                let mut displayed_skip_message = false;
                loop {
                    let read_fut = reader.read_until_block();
                    let nodes = match timeout(OP_TIMEOUT, read_fut).await {
                        Ok(result) => result
                            .map_err(FirehoseError::ReadUntilBlockError)
                            .map_err(|e| (e, current_slot.unwrap_or(slot_range.start)))?,
                        Err(_) => {
                            log::warn!(target: &log_target, "timeout reading next block, retrying (will restart)...");
                            let restart_slot =
                                current_slot.map(|s| s + 1).unwrap_or(slot_range.start);
                            return Err((
                                FirehoseError::OperationTimeout("read_until_block"),
                                restart_slot,
                            ));
                        }
                    };
                    thread_activity::note(thread_index.unwrap_or(0));
                    let stream_ended = nodes.is_empty()
                        || nodes
                            .0
                            .last()
                            .is_some_and(|last_node| !last_node.get_node().is_block());
                    if stream_ended {
                        // EOF is ambiguous (genuine epoch end vs a connection the CDN closed
                        // mid-transfer); consult the slot index before completing.
                        let scan_end = local_end_inclusive.min(slot_range.end.saturating_sub(1));
                        if let Some(missing) =
                            crate::jetstreamer_firehose::index::next_present_slot(last_counted_slot, scan_end).await
                        {
                            log::warn!(
                                target: &log_target,
                                "stream ended prematurely in epoch {} — slot {} (and possibly more) still unprocessed; restarting",
                                epoch_num,
                                missing
                            );
                            return Err((
                                FirehoseError::PrematureStreamEnd,
                                last_counted_slot.saturating_add(1),
                            ));
                        }
                        log::info!(target: &log_target, "reached end of epoch {}", epoch_num);
                        break;
                    }
                    let block = nodes
                        .get_block()
                        .map_err(FirehoseError::GetBlockError)
                        .map_err(|e| (e, current_slot.unwrap_or(slot_range.start)))?;
                    log::debug!(
                        target: &log_target,
                        "read {} items from epoch {}, now at slot {}",
                        item_index,
                        epoch_num,
                        block.slot
                    );
                    let slot = block.slot;
                    if slot > local_end_inclusive {
                        log::debug!(
                            target: &log_target,
                            "reached end of local slice at slot {} (epoch {}), stopping",
                            slot,
                            epoch_num
                        );
                        break;
                    }
                    if slot >= slot_range.end {
                        log::info!(target: &log_target, "reached end of slot range at slot {}", slot);
                        // Return early to terminate the firehose thread cleanly. We use >=
                        // because slot_range is half-open [start, end), so any slot equal to
                        // end is out-of-range and must not be processed.
                        return Ok(());
                    }
                    debug_assert!(slot < slot_range.end, "processing out-of-range slot {} (end {})", slot, slot_range.end);
                    if slot < local_start {
                        if slot.saturating_add(1) == local_start {
                            log::debug!(
                                target: &log_target,
                                "priming reader with preceding slot {}, skipping",
                                slot
                            );
                        } else {
                            log::warn!(
                                target: &log_target,
                                "encountered slot {} before start of range {}, skipping",
                                slot,
                                local_start
                            );
                        }
                        continue;
                    }
                    current_slot = Some(slot);
                    let mut entry_index: usize = 0;
                    let mut this_block_executed_transaction_count: u64 = 0;
                    let mut this_block_entry_count: u64 = 0;
                    let mut this_block_rewards = DecodedRewards::empty();

                    if slot <= last_counted_slot {
                        log::debug!(
                            target: &log_target,
                            "duplicate block {}, already counted (last_counted={})",
                            slot,
                            last_counted_slot,
                        );
                        continue;
                    }

                    nodes.each(|node_with_cid| -> Result<(), SharedError> {
                        item_index += 1;
                        // if item_index == 100000 && !triggered { log::info!("simulating
                        //     error"); triggered = true; return
                        //     Err(Box::new(GeyserReplayError::NodeDecodingError(item_index,
                        //     Box::new(std::io::Error::new( std::io::ErrorKind::Other,
                        //         "simulated error", )), ))); }
                        if let Some(skip) = skip_until_index {
                            if item_index < skip {
                                if !displayed_skip_message {
                                    log::info!(
                                        target: &log_target,
                                        "skipping until index {} (at {})",
                                        skip,
                                        item_index
                                    );
                                    displayed_skip_message = true;
                                }
                                return Ok(());
                            } else {
                                log::info!(
                                    target: &log_target,
                                    "reached target index {}, resuming...",
                                    skip
                                );
                                skip_until_index = None;
                            }
                        }
                        let node = node_with_cid.get_node();

                        use crate::jetstreamer_firehose::node::Node::*;
                        match node {
                            Transaction(tx) => {
                                let versioned_tx = tx.as_parsed()?;
                                let reassembled_metadata = nodes.reassemble_dataframes(&tx.metadata)?;

                                let as_native_metadata = decode_transaction_status_meta_from_frame(
                                    block.slot,
                                    reassembled_metadata,
                                )?;

                                let message_hash = {
                                    #[cfg(feature = "verify-transaction-signatures")]
                                    {
                                        versioned_tx.verify_and_hash_message()?
                                    }
                                    #[cfg(not(feature = "verify-transaction-signatures"))]
                                    {
                                        // Signature verification is optional because it is
                                        // extremely expensive at replay scale.
                                        versioned_tx.message.hash()
                                    }
                                };
                                let signature = versioned_tx
                                    .signatures
                                    .first()
                                    .ok_or_else(|| {
                                        Box::new(std::io::Error::new(
                                            std::io::ErrorKind::InvalidData,
                                            "transaction missing signature",
                                        )) as SharedError
                                    })?;
                                let is_vote = is_simple_vote_transaction(&versioned_tx);

                                if let Some(transaction_notifier) = transaction_notifier_maybe.as_ref() {
                                    transaction_notifier.notify_transaction(
                                        block.slot,
                                        // Replayed from the archive: one bank per slot.
                                        block.slot,
                                        tx.index.unwrap() as usize,
                                        signature,
                                        &message_hash,
                                        is_vote,
                                        &as_native_metadata,
                                        &versioned_tx,
                                    );
                                }

                            }
                            Entry(entry) => {
                                let entry_hash = Hash::from(entry.hash.to_bytes());
                                let entry_transaction_count = entry.transactions.len();
                                let entry_transaction_count_u64 = entry_transaction_count as u64;
                                let starting_transaction_index =
                                    usize::try_from(this_block_executed_transaction_count).map_err(|_| {
                                        Box::new(std::io::Error::other(
                                            "transaction index exceeds usize range",
                                        )) as SharedError
                                    })?;
                                todo_latest_entry_blockhash = entry_hash;
                                this_block_executed_transaction_count += entry_transaction_count_u64;
                                this_block_entry_count += 1;
                                if entry_notifier_maybe.is_none() {
                                    return Ok(());
                                }
                                let entry_notifier = entry_notifier_maybe.as_ref().unwrap();
                                let entry_summary = solana_entry::entry::EntrySummary {
                                    num_hashes: entry.num_hashes,
                                    hash: Hash::from(entry.hash.to_bytes()),
                                    num_transactions: entry_transaction_count_u64,
                                };
                                entry_notifier.notify_entry(
                                    block.slot,
                                    block.slot,
                                    entry_index,
                                    &entry_summary,
                                    starting_transaction_index,
                                );
                                entry_index += 1;
                            }
                            Block(block) => {
                                let notification = SlotNotification::Root((block.slot, block.meta.parent_slot, block.slot));
                                confirmed_bank_sender.send(notification).unwrap();

                                if block_meta_notifier_maybe.is_none() {
                                    last_counted_slot = block.slot;
                                    return Ok(());
                                }
                                let decoded_rewards = std::mem::take(&mut this_block_rewards);
                                let commission_rate_in_basis_points =
                                    decoded_rewards.commission_rate_in_basis_points;
                                let block_meta_notifier = block_meta_notifier_maybe.as_ref().unwrap();
                                block_meta_notifier.notify_block_metadata(
                                    block.meta.parent_slot,
                                    todo_previous_blockhash.to_string().as_str(),
                                    block.slot,
                                    block.slot,
                                    todo_latest_entry_blockhash.to_string().as_str(),
                                    &decoded_rewards.rewards,
                                    Some(block.meta.blocktime as i64),
                                    block.meta.block_height,
                                    this_block_executed_transaction_count,
                                    this_block_entry_count,
                                    commission_rate_in_basis_points,
                                );
                                todo_previous_blockhash = todo_latest_entry_blockhash;
                                last_counted_slot = block.slot;
                                std::thread::yield_now();
                            }
                            Subset(_subset) => (),
                            Epoch(_epoch) => (),
                            Rewards(rewards) => {
                                let reassembled = nodes.reassemble_dataframes(&rewards.data)?;
                                if !reassembled.is_empty() {
                                    this_block_rewards = decode_rewards_from_frame(
                                        block.slot,
                                        reassembled,
                                    )?;
                                } else {
                                    this_block_rewards = DecodedRewards::empty();
                                }
                            }
                            DataFrame(_data_frame) => (),
                        }
                        Ok(())
                    })
                .map_err(|e| FirehoseError::NodeDecodingError(item_index, e)).map_err(|e| (e, current_slot.unwrap_or(slot_range.start)))?;
                    if block.slot == slot_range.end - 1 {
                        let finish_time = std::time::Instant::now();
                        let elapsed = finish_time.duration_since(start_time);
                        log::info!(target: &log_target, "processed slot {}", block.slot);
                        let elapsed_pretty = human_readable_duration(elapsed);
                        log::info!(
                            target: &log_target,
                            "processed {} slots across {} epochs in {}.",
                            initial_slot_range.end - initial_slot_range.start,
                            slot_to_epoch(initial_slot_range.end)
                                + 1
                                - slot_to_epoch(initial_slot_range.start),
                            elapsed_pretty
                        );
                        log::info!(target: &log_target, "❄️ firehose thread completed its work.");
                        thread_activity::note_finished(thread_index.unwrap_or(0));
                        // On completion, report threads with non-zero error counts for
                        // visibility.
                        let summary: String = error_counts
                            .iter()
                            .enumerate()
                            .filter_map(|(i, c)| {
                                let v = c.load(Ordering::Relaxed);
                                if v > 0 { Some(format!("{:03}({})", i, v)) } else { None }
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        if !summary.is_empty() {
                            log::debug!(target: &log_target, "threads with errors: {}", summary);
                        }
                        return Ok(());
                    }
                }
            }
            Ok(())
}
.await
{
        if is_shutdown_error(&err) {
            log::info!(
                target: &log_target,
                "shutdown requested; terminating firehose thread {:?}",
                thread_index
            );
            return Ok(());
        }
        log::error!(
            target: &log_target,
            "⚠️🧊 firehose encountered an error at slot {} in epoch {} and will roll back one slot and retry:",
            slot,
            slot_to_epoch(slot)
            );
            log::error!(target: &log_target, "{}", err);
            let error_message = err.to_string();
            if matches!(err, FirehoseError::SlotOffsetIndexError(_))
                || error_message.contains("Unknown CID version")
            {
                // Clear cached index data for this epoch to avoid retrying with a bad/partial index
                // (or a bad seek offset that landed mid-stream).
                SLOT_OFFSET_INDEX.invalidate_epoch(slot_to_epoch(slot));
            }
            let item_index = match err {
                FirehoseError::NodeDecodingError(item_index, _) => item_index,
                _ => 0,
            };
            // Increment this thread's error counter
            let idx = thread_index.unwrap_or(0);
            error_counts[idx].fetch_add(1, Ordering::Relaxed);
            log::warn!(
                target: &log_target,
                "restarting from slot {} at index {}",
                slot,
                item_index,
            );
            // Update slot range to resume from the failed slot, not the original start.
            // If the failing slot was already fully processed, resume from the next slot.
            if slot <= last_counted_slot {
                slot_range.start = last_counted_slot.saturating_add(1);
            } else {
                slot_range.start = slot;
            }
            // `skip_until_index` is unsafe across retries because `item_index`
            // is reset to 0 each epoch restart. Keeping it can skip large portions
            // of the stream and silently drop slots.
            skip_until_index = None;
            let backoff = retry_backoff.next_delay(slot);
            log::warn!(
                target: &log_target,
                "backing off {:?} before restarting",
                backoff
            );
            sleep(backoff).await;
}
    Ok(())
}

#[inline]
fn is_simple_vote_transaction(versioned_tx: &VersionedTransaction) -> bool {
    if !(1..=2).contains(&versioned_tx.signatures.len()) {
        return false;
    }

    if !matches!(
        versioned_tx.version(),
        solana_transaction::versioned::TransactionVersion::Legacy(_)
    ) {
        return false;
    }

    let instructions = versioned_tx.message.instructions();
    if instructions.len() != 1 {
        return false;
    }

    let program_index = instructions[0].program_id_index as usize;
    versioned_tx
        .message
        .static_account_keys()
        .get(program_index)
        .map(|program_id| program_id == &vote_program_id())
        .unwrap_or(false)
}

#[inline(always)]
fn convert_proto_rewards(
    proto_rewards: &solana_storage_proto::convert::generated::Rewards,
) -> Result<KeyedRewardsAndNumPartitions, SharedError> {
    let mut keyed_rewards = Vec::with_capacity(proto_rewards.rewards.len());
    for proto_reward in proto_rewards.rewards.iter() {
        let reward = StakeRewardInfo {
            reward_type: match proto_reward.reward_type - 1 {
                0 => RewardType::Fee,
                1 => RewardType::Rent,
                2 => RewardType::Staking,
                3 => RewardType::Voting,
                4 => RewardType::DeactivatedStake,
                5 => RewardType::VATDebit,
                typ => {
                    return Err(Box::new(std::io::Error::other(format!(
                        "unsupported reward type {}",
                        typ
                    ))));
                }
            },
            lamports: proto_reward.lamports,
            post_balance: proto_reward.post_balance,
            // Blocks recorded before SIMD-0291 only carry a whole-percent commission; scale it
            // so the runtime's basis-point field is populated either way.
            commission_bps: proto_reward.commission_bps.parse::<u16>().ok().or_else(|| {
                proto_reward
                    .commission
                    .parse::<u8>()
                    .ok()
                    .map(|percent| u16::from(percent) * 100)
            }),
        };
        let pubkey = proto_reward
            .pubkey
            .parse::<Address>()
            .map_err(|err| Box::new(err) as SharedError)?;
        keyed_rewards.push((pubkey, reward.into()));
    }
    Ok(KeyedRewardsAndNumPartitions {
        keyed_rewards,
        num_partitions: proto_rewards
            .num_partitions
            .as_ref()
            .map(|p| p.num_partitions),
    })
}

#[inline]
/// Splits `slot_range` into nearly-even sub-ranges for the given thread count.
pub fn generate_subranges(slot_range: &Range<u64>, threads: u64) -> Vec<Range<u64>> {
    let total = slot_range.end - slot_range.start;
    let slots_per_thread = total / threads;
    let remainder = total % threads;

    let ranges: Vec<Range<u64>> = (0..threads)
        .map(|i| {
            // Distribute remainder slots to the first `remainder` threads
            let extra_slot = if i < remainder { 1 } else { 0 };
            let start = slot_range.start + i * slots_per_thread + i.min(remainder);
            let end = start + slots_per_thread + extra_slot;
            start..end
        })
        .collect();

    // Verify that ranges cover all slots exactly
    let total_covered: u64 = ranges.iter().map(|r| r.end - r.start).sum();
    assert_eq!(
        total_covered, total,
        "Range generation failed: {} threads should cover {} slots but only cover {}",
        threads, total, total_covered
    );

    // Verify no gaps between ranges
    for i in 1..ranges.len() {
        assert_eq!(
            ranges[i - 1].end,
            ranges[i].start,
            "Gap found between thread {} (ends at {}) and thread {} (starts at {})",
            i - 1,
            ranges[i - 1].end,
            i,
            ranges[i].start
        );
    }

    log::info!(
        target: LOG_MODULE,
        "Generated {} thread ranges covering {} slots total",
        threads,
        total_covered
    );
    ranges
}

fn human_readable_duration(duration: std::time::Duration) -> String {
    if duration.is_zero() {
        return "0s".into();
    }
    let total_secs = duration.as_secs();
    if total_secs < 60 {
        let secs_f = duration.as_secs_f64();
        if total_secs == 0 {
            format!("{:.2}s", secs_f)
        } else if duration.subsec_millis() == 0 {
            format!("{}s", total_secs)
        } else {
            format!("{:.2}s", secs_f)
        }
    } else {
        let mut secs = total_secs;
        let days = secs / 86_400;
        secs %= 86_400;
        let hours = secs / 3_600;
        secs %= 3_600;
        let minutes = secs / 60;
        secs %= 60;
        if days > 0 {
            if hours > 0 {
                format!("{days}d{hours}h")
            } else {
                format!("{days}d")
            }
        } else if hours > 0 {
            if minutes > 0 {
                format!("{hours}h{minutes}m")
            } else {
                format!("{hours}h")
            }
        } else if minutes > 0 {
            if secs > 0 {
                format!("{minutes}m{secs}s")
            } else {
                format!("{minutes}m")
            }
        } else {
            format!("{secs}s")
        }
    }
}
