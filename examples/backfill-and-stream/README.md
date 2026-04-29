# Backfill and Stream Example

This example demonstrates the production cold-start pattern: backfill historical events via Jetstream's archive, then keep going on a Yellowstone live tail. Both sources feed the same processor, with `DeduplicationFilter` resolving the overlap window between Jetstream's tail and Yellowstone's confirmed tip. `channel_buffer_size` is widened to absorb Jetstream's catch-up burst without backpressuring the live tail.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/backfill-and-stream
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example:

```env
GEYSER_URL=https://your-yellowstone-endpoint:443
X_TOKEN=your-yellowstone-auth-token
RUST_LOG=info
```

- `GEYSER_URL`: Yellowstone Geyser gRPC endpoint URL. Required.
- `X_TOKEN`: auth token for Yellowstone. Optional; required if your provider enforces it.
- `RUST_LOG`: log level. `info` shows decoded events from both legs.

The Jetstream slot range and Old Faithful endpoint are configured directly in [`src/main.rs`](src/main.rs). Adjust `JetstreamerRange::Slot(...)` to match the historical window you want, or switch to `JetstreamerRange::Epoch(N)` for a full epoch.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p backfill-and-stream-carbon-example
```

Jetstream backfills the configured slot range while Yellowstone streams live in parallel. The `DeduplicationFilter` ensures each event is processed exactly once across the cutover. The pipeline runs indefinitely on the live leg — exit with ctrl-C.
