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

No environment variables are required — the default constructor uses the Solana Foundation's public Old Faithful endpoint. Optionally:

```env
RUST_LOG=info
```

- `RUST_LOG`: log level. `info` shows decoded Token-2022 instructions and the final `backfill complete` message.

The slot range, transaction filter, and worker count are configured directly in [`src/main.rs`](src/main.rs). Edit `JetstreamerRange::Slot(start, end)` to change the window — default is roughly one epoch's worth of data (~432k slots).

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

The pipeline processes the configured slot range and exits via `ProcessPending`. Throughput depends on Jetstream's rate limits and the worker count — for a full epoch expect tens of minutes wall time. Narrow the range in `main.rs` for a quick smoke test.
