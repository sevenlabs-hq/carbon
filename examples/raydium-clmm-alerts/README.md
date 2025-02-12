# Carbon Pipeline Example

This project demonstrates how to set up and run a Carbon pipeline that processes Solana transactions. It uses the `RpcBlockSubscribe` to fetch Solana transactions related to a specific program and processes these transactions using the `RaydiumClmmDecoder`. The example demonstrates how to decode Raydium CLMM instructions and implement a custom processor for handling various types of events.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/raydium-clmm-alerts
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variables:

```env
RPC_WS_URL=...
```

This `RPC_WS_URL` should point to the RPC Websocket endpoint you want to use for Solana block subscribing.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building the project, you can run the pipeline using:

```sh
cargo run --release
```

This will start the block subscriber, and the pipeline will begin processing transactions for the specified program ID (`RAYDIUM_CLMM_PROGRAM_ID`).

## Metrics

The example includes a basic metrics setup using `LogMetrics`. You can extend this by implementing your own metrics and passing them to the pipeline.
