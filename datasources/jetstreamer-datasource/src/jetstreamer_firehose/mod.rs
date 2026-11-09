#![deny(missing_docs)]
//! Core data structures and streaming utilities for Jetstreamer firehose processing.
//!
//! # Overview
//! The firehose crate streams data live over the network directly from Project Yellowstone's
//! [Old Faithful](https://old-faithful.net/) archive of CAR files, which hosts the complete
//! history of every Solana transaction. Data only flows outward from Old Faithful to your
//! local consumer; nothing is ever uploaded back to the archive. With sufficient CPU and
//! network headroom the pipeline can exceed 2.7 million transactions per second while decoding
//! the stream for analysis and backfilling workloads.
//!
//! Firehose is the foundation that powers
//! [`jetstreamer`](https://crates.io/crates/jetstreamer) and
//! [`jetstreamer-plugin`](https://crates.io/crates/jetstreamer-plugin), but it can also be
//! consumed directly to build bespoke replay pipelines. The crate exposes:
//! - Async readers for Old Faithful CAR archives via [`firehose`].
//! - Rich data models for blocks, entries, rewards, and transactions.
//! - Epoch helpers for reasoning about slot ranges and availability windows.
//!
//! # External Geyser plugins
//! Loading an external Agave plugin through `firehose_geyser` requires the optional
//! `geyser` feature and the root-level RocksDB patches documented in the
//! [crate README](https://github.com/vaultic-win/jetstreamer/tree/main/jetstreamer-firehose#external-geyser-plugins).
//! Default streaming and Jetstreamer plugins do not require those patches.
//!
//! # Configuration
//! Several environment variables influence how the firehose locates and caches data:
//! - `JETSTREAMER_HTTP_BASE_URL` (default `https://files.old-faithful.net`): base URL or
//!   `s3://bucket/prefix` for CAR snapshots. Change this to point at a private mirror.
//! - `JETSTREAMER_COMPACT_INDEX_BASE_URL` (default `https://files.old-faithful.net`): base URL
//!   for slot index artifacts (the per-epoch `slot-ranges.raw` file, plus the deprecated
//!   legacy compactindexes used as a fallback). Point this at your own mirror to reduce load
//!   on the public Old Faithful deployment. Also supports `s3://` URIs.
//! - `JETSTREAMER_FORCE_LEGACY_INDEX`: set to `1` to skip the slot-ranges index and resolve
//!   slot offsets using the deprecated legacy compactindex pair.
//! - `JETSTREAMER_ARCHIVE_BASE`: fallback URL/URI that applies to both CARs and compact
//!   indexes when the more specific knobs are unset.
//! - `JETSTREAMER_ARCHIVE_BACKEND` (default `http`): set to `s3` to force the S3 transport even
//!   when the resolved URL still points at `https://`.
//! - `JETSTREAMER_S3_BUCKET`, `JETSTREAMER_S3_PREFIX`, `JETSTREAMER_S3_INDEX_PREFIX`,
//!   `JETSTREAMER_S3_REGION`, `JETSTREAMER_S3_ENDPOINT`, `JETSTREAMER_S3_ACCESS_KEY`,
//!   `JETSTREAMER_S3_SECRET_KEY`, `JETSTREAMER_S3_SESSION_TOKEN`: credentials and overrides
//!   applied when the S3 backend is active.
//! - `JETSTREAMER_NETWORK` (default `mainnet`): suffix appended to cache namespaces and index
//!   filenames so you can swap between clusters without purging local state.
//! - `JETSTREAMER_TURBO` (default `false`): when truthy, runs at full speed instead of the
//!   polite default. See [`pace`].
//! - `JETSTREAMER_SPAWN_PENDING` (default `4`, `24` in turbo mode): maximum not-yet-green
//!   threads in flight during the health-gated thread ramp; `1` reproduces a strict
//!   one-at-a-time ramp.
//! - `JETSTREAMER_SPAWN_GRACE_SECS` (default `30`): how long the launch gate waits for
//!   sluggish (never stalled) threads before spawning anyway; `0` disables launch gating.
//! - `JETSTREAMER_RECYCLE_PCT` (default `50`): connection-recycle threshold as a percent of
//!   the best observed p90 per-thread rate; `0` disables recycling.
//! - `JETSTREAMER_NETWORK_CAPACITY_MB` (default `1000`): assumed network throughput in megabytes
//!   per second used when sizing the firehose thread pool. Increase or decrease to match your
//!   host's effective bandwidth.
//!
//! S3 transports are compiled behind the `s3-backend` Cargo feature. Enable that feature when
//! building if you plan to stream from `s3://` URIs instead of HTTP mirrors.
//!
//! Sequential-mode ripget buffering is configured on the [`firehose::firehose`] call via
//! `buffer_window_bytes`. Pass `None` to use the built-in default (`min(4 GiB, 15% of available
//! RAM)`). If you're using [`jetstreamer`](https://crates.io/crates/jetstreamer)'s binary runner,
//! that layer exposes `JETSTREAMER_BUFFER_WINDOW` and forwards the parsed value here.
//!
//! # Limitations
//! Old Faithful currently publishes blocks, transactions, epochs, and reward metadata but does
//! not ship account updates. The firehose mirrors that limitation; plan on a separate data
//! source if you require account updates.
//!
//! # Epoch Feature Availability
//! Old Faithful snapshots expose different metadata as the Solana protocol evolved. Use the
//! table below to decide which replay windows fit your requirements:
//!
//! | Epoch range | Slot range    | Comment |
//! |-------------|---------------|--------------------------------------------------|
//! | 0–156       | 0–?           | Incompatible with modern Geyser plugins          |
//! | 157+        | ?             | Compatible with modern Geyser plugins            |
//! | 0–449       | 0–194184610   | CU tracking not available (reported as `0`)      |
//! | 450+        | 194184611+    | CU tracking fully available                      |
//!
//! Detailed helpers for translating between epochs and slots live in the [`epochs`] module.
//!
//! # Ordering Guarantees
//! Because [`firehose`] spawns parallel threads that process different subranges of the
//! overall slot range at the same time, while each thread sees a purely sequential view of
//! transactions, downstream services such as databases that consume this data will see writes
//! in a fairly arbitrary order, so you should design your database tables and shared data
//! structures accordingly. The best pattern is to aggregate data on some interval on a
//! thread-local, per-thread basis and periodically flush the aggregated data to the shared
//! downstream service or data structure.
//!
//! # Examples
//! Run the firehose with handlers for every data type:
//! ```no_run
//! use futures_util::FutureExt;
//! use jetstreamer_firehose::{
//!     epochs,
//!     firehose::{self, BlockData, EntryData, RewardsData, Stats, StatsTracking, TransactionData},
//! };
//!
//! fn block_handler() -> impl firehose::Handler<BlockData> {
//!     move |_thread_id, block| async move {
//!         println!("block slot {}", block.slot());
//!         Ok(())
//!     }
//!     .boxed()
//! }
//!
//! fn tx_handler() -> impl firehose::Handler<TransactionData> {
//!     move |_thread_id, tx| async move {
//!         println!("tx {} in slot {}", tx.signature, tx.slot);
//!         Ok(())
//!     }
//!     .boxed()
//! }
//!
//! fn entry_handler() -> impl firehose::Handler<EntryData> {
//!     move |_thread_id, entry| async move {
//!         println!("entry {} covering transactions {:?}", entry.entry_index, entry.transaction_indexes);
//!         Ok(())
//!     }
//!     .boxed()
//! }
//!
//! fn reward_handler() -> impl firehose::Handler<RewardsData> {
//!     move |_thread_id, rewards| async move {
//!         println!("rewards in slot {} -> {} accounts", rewards.slot, rewards.rewards.len());
//!         Ok(())
//!     }
//!     .boxed()
//! }
//!
//! fn stats_handler() -> impl firehose::Handler<Stats> {
//!     move |_thread_id, stats| async move {
//!         println!("processed {} slots so far", stats.slots_processed);
//!         Ok(())
//!     }
//!     .boxed()
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let stats = StatsTracking {
//!         on_stats: stats_handler(),
//!         tracking_interval_slots: 100,
//!     };
//!
//!     let (start, _) = epochs::epoch_to_slot_range(800);
//!     let (_, end_inclusive) = epochs::epoch_to_slot_range(805);
//!     let slot_range = start..(end_inclusive + 1);
//!
//!     firehose::firehose(
//!         4,
//!         false,
//!         false,
//!         None,
//!         slot_range,
//!         Some(block_handler()),
//!         Some(tx_handler()),
//!         Some(entry_handler()),
//!         Some(reward_handler()),
//!         None::<firehose::OnErrorFn>,
//!         Some(stats),
//!         None,
//!     )
//!     .await
//!     .map_err(|(err, slot)| -> Box<dyn std::error::Error> {
//!         format!("firehose failed at slot {slot}: {err}").into()
//!     })?;
//!     Ok(())
//! }
//! ```

/// Shared boxed error type propagated across firehose modules.
pub type SharedError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Backend configuration for archive mirrors (HTTP and S3).
pub mod archive;
/// Types for decoding block-level records emitted by the firehose.
pub mod block;
/// Encodes and decodes arbitrary binary [`DataFrame`](dataframe::DataFrame) nodes.
pub mod dataframe;
/// Parsing and serialization helpers for [`Entry`](entry::Entry) nodes.
pub mod entry;
/// Structures for the top-level [`Epoch`](epoch::Epoch) node type.
pub mod epoch;
/// Epoch utilities such as [`epoch_to_slot_range`](epochs::epoch_to_slot_range).
pub mod epochs;
/// Streaming interface for fetching and parsing firehose blocks.
pub mod firehose;
/// Slot offset index client for locating blocks in Old Faithful CAR archives.
pub mod index;
/// Helpers for working with network metadata and endpoints.
pub mod network;
/// Core node tree definitions shared across firehose types.
pub mod node;
/// Reader utilities for decoding Old Faithful CAR node streams.
pub mod node_reader;
/// Request pacing profiles: polite by default, turbo for full speed.
pub mod pace;
/// Reward decoding primitives and helpers.
pub mod rewards;
/// Utilities for working with subset nodes.
pub mod subset;
/// System heuristics for sizing the firehose runtime.
pub mod system;
/// Transaction decoding and helpers.
pub mod transaction;
/// Shared helpers used throughout the firehose crate.
pub mod utils;

pub use firehose::{BlockData, EntryData, RewardsData, TransactionData};

/// Log target prefix used across the firehose crate.
pub const LOG_MODULE: &str = "jetstreamer::firehose";
