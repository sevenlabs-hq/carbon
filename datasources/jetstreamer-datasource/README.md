# Carbon Jetstreamer Datasource

Bounded-range historical backfill over [Jetstreamer](https://github.com/anza-xyz/jetstreamer),
Anza's high-throughput reader for the Old Faithful CAR archives. Streams a slot
or epoch range through Carbon's pipeline as `Transaction` and `BlockDetails`
updates, with upstream-side filtering and multi-threaded fetch.

> [!IMPORTANT]
> This crate is **not published to crates.io**. Transaction V1 parsing and the
> Agave 4.2 stack that Carbon 2 requires landed in
> [anza-xyz/jetstreamer#95](https://github.com/anza-xyz/jetstreamer/pull/95),
> which is not in any published Jetstreamer release yet — the latest is `0.7.0`,
> still on Solana v3. `jetstreamer-firehose` is therefore pinned to a git rev in
> the workspace manifest, and Cargo cannot publish a crate with a git
> dependency. Use it from a git checkout of Carbon until upstream cuts a release.

## Usage

```rust
use carbon_jetstreamer_datasource::{
    filter::{JetstreamerFilter, TransactionFilter},
    range::JetstreamerRange,
    JetstreamerDatasource,
};

let datasource = JetstreamerDatasource::new_with_old_faithful_mainnet(
    JetstreamerRange::Slot(415_500_000, 415_931_999),
    JetstreamerFilter {
        include_transactions: true,
        include_blocks: false,
        transaction_filters: vec![TransactionFilter {
            vote: Some(false),
            failed: Some(false),
            account_include: HashSet::from([program_id]),
            account_exclude: HashSet::new(),
            account_required: HashSet::new(),
        }],
    },
    4,          // worker threads
    Some(100),  // stats tracking interval, in slots
);
```

`JetstreamerRange::Epoch(epoch)` backfills a whole epoch instead of an explicit
slot window.

### Sequential mode

Jetstreamer can stream epochs through a single worker using ripget's windowed
downloader, which is often faster for large contiguous ranges:

```rust
let datasource = datasource
    .with_sequential(true)
    .with_reverse(true)                          // newest epoch first
    .with_buffer_window_bytes(Some(2 << 30));    // 2 GiB hot window
```

In sequential mode `threads` configures ripget range concurrency rather than
firehose worker partitioning. `reverse` and `buffer_window_bytes` only take
effect when `sequential` is set; slots within an epoch are always emitted in
ascending order, because a CAR archive can only be streamed forward.

## Configuration

Jetstreamer reads its archive location and cluster from the process
environment. `archive_url` and `network` on the datasource set
`JETSTREAMER_COMPACT_INDEX_BASE_URL` and `JETSTREAMER_NETWORK` for you, but
because those are process-global, prefer exporting them in the environment when
the pipeline hosts more than one datasource.

Defaults target the Solana Foundation's public Old Faithful mainnet archive.

## Rewards and SIMD-0291

SIMD-0291 replaced the whole-percent `commission` on rewards with
`commission_bps`. Jetstreamer normalizes both ledger eras into basis points, so
this datasource populates `Reward::commission_bps` verbatim and derives the
legacy `Reward::commission` from it. For pre-SIMD-0291 blocks that derivation
inverts the scaling exactly; for later blocks the legacy field is the truncated
whole-percent view.

## Metrics

| Metric | Type | Meaning |
| --- | --- | --- |
| `jetstreamer_blocks_sent_total` | counter | Block details forwarded |
| `jetstreamer_transactions_sent_total` | counter | Transactions forwarded |
| `jetstreamer_transactions_filtered_in_total` | counter | Transactions that passed filters |
| `jetstreamer_transactions_filtered_out_total` | counter | Transactions rejected by filters |
| `jetstreamer_firehose_errors_total` | counter | Recoverable firehose worker errors |
| `jetstreamer_internal_slots_processed` | gauge | Slots processed, from Jetstreamer stats |
| `jetstreamer_internal_blocks_processed` | gauge | Blocks processed, from Jetstreamer stats |
| `jetstreamer_internal_transactions_processed` | gauge | Transactions processed, from Jetstreamer stats |

Stats-derived gauges only update when `tracking_interval_slots` is set.
