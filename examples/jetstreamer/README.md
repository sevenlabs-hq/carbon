# Jetstreamer Example

This example demonstrates how to backfill historical Token-2022 activity over a slot range via the Jetstream API (Solana Foundation's Old Faithful archive). It uses upstream-side transaction filtering (only matching transactions are returned), multi-threaded fetch for higher throughput than serial RPC polling, and exits cleanly via `ProcessPending` when the slot range drains.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/jetstreamer
```

### Step 2: Set Environment Variables

No environment variables are required — the default constructor uses the Solana Foundation's public Old Faithful endpoint. Override the backfill window and fetch concurrency with:

```env
START_SLOT=415500000
END_SLOT=415931999
WORKER_COUNT=4
PAGE_SIZE=100
RUST_LOG=info
```

- `START_SLOT`: first slot to backfill. Defaults to `415500000`.
- `END_SLOT`: last slot to backfill. Defaults to `415931999`.
- `WORKER_COUNT`: parallel fetch workers. Defaults to `4`.
- `PAGE_SIZE`: page size passed to Jetstreamer. Defaults to `100`.
- `RUST_LOG`: log level. `info` shows decoded Token-2022 instructions and the final `backfill complete` message.

The transaction filter is still configured directly in [`src/main.rs`](src/main.rs). The default slot window is roughly one epoch's worth of data (~432k slots).

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p jetstreamer-carbon-example
```

The pipeline processes the configured slot range and exits via `ProcessPending`. Throughput depends on Jetstream's rate limits and `WORKER_COUNT` — for a full epoch expect tens of minutes wall time. Narrow `START_SLOT` / `END_SLOT` for a quick smoke test.
