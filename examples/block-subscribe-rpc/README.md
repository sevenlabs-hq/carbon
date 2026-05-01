# Block Subscribe RPC Example

This example demonstrates a real-time Carbon pipeline over the public Solana RPC `blockSubscribe` method — no Geyser endpoint required. It subscribes to confirmed Raydium CLMM transactions, decodes each instruction, and wires both an instruction processor and a per-block `block_details` processor against the same upstream.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/block-subscribe-rpc
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example:

```env
RPC_WS_URL=wss://api.mainnet-beta.solana.com
RUST_LOG=info
```

- `RPC_WS_URL`: Solana RPC WebSocket endpoint that exposes `blockSubscribe`. Falls back to public mainnet-beta if unset, but that endpoint is rate-limited; use a dedicated provider (Helius, QuickNode, Triton, Shyft) for production volume.
- `RUST_LOG`: log level. `info` shows decoded swap / open-position events plus per-block headers; `debug` shows every Raydium CLMM instruction.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p block-subscribe-rpc-carbon-example
```

The pipeline streams indefinitely — exit with ctrl-C. You'll see two interleaved log streams: per-block headers (slot/hash/parent/time) and decoded instructions. If you only see headers and no instructions, the RPC may not be returning full transaction details.
