# Yellowstone gRPC Example

This example demonstrates a real-time Carbon pipeline backed by a Yellowstone Geyser gRPC stream — the canonical streaming setup. It subscribes to confirmed Jupiter v6 transactions on mainnet and decodes each instruction into the typed `JupiterSwapInstruction` enum.

## Variants

This crate ships three interchangeable upstream variants in [`src/variants.rs`](src/variants.rs):

- `variants::yellowstone(...)` — vanilla Yellowstone Geyser gRPC. Default in `main.rs`.
- `variants::laserstream(...)` — Helius LaserStream gRPC. Adds the `replay_enabled` flag for replaying missed slots after a reconnect.
- `variants::jito_shredstream()` — Jito Shredstream gRPC. Streams pre-confirmation transactions from raw shreds.

Swap by editing the line in `main.rs`:

```rust
let datasource = variants::yellowstone(transaction_filters);
let datasource = variants::laserstream(transaction_filters);
let datasource = variants::jito_shredstream();
```

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/yellowstone-grpc
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example and set the variables for whichever variant you're running:

```env
# Yellowstone variant (default)
GEYSER_URL=https://your-yellowstone-endpoint:443
X_TOKEN=your-auth-token

# LaserStream variant
# LASERSTREAM_URL=https://laserstream-mainnet-ewr.helius-rpc.com
# API_KEY=your-helius-api-key

# Jito Shredstream variant
# JITO_SHREDSTREAM_URL=http://your-jito-shredstream-proxy:port

RUST_LOG=info
```

- `GEYSER_URL`: Yellowstone gRPC endpoint URL. Required for the Yellowstone variant.
- `X_TOKEN`: auth token for Yellowstone. Required if your provider enforces it (optional otherwise).
- `LASERSTREAM_URL`: Helius LaserStream gRPC endpoint. Required for the LaserStream variant.
- `API_KEY`: Helius API key. Required for the LaserStream variant.
- `JITO_SHREDSTREAM_URL`: Jito Shredstream proxy gRPC endpoint. Required for the Jito Shredstream variant.
- `RUST_LOG`: log level (`info` shows decoded swaps plus periodic pipeline counters).

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p yellowstone-grpc-carbon-example
```

The pipeline streams indefinitely — exit with ctrl-C. `LogMetrics` interleaves periodic counters (received, processed, queue depth, processing time) between decoded events.