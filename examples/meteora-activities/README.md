# Carbon Pipeline Example

This project demonstrates how to set up and run a Carbon pipeline that processes Solana transactions. It uses the `RpcTransactionCrawler` to fetch Solana transactions related to a specific program and processes these transactions using the `MeteoraDlmmDecoder`. The example demonstrates how to decode Meteora DLMM instructions and implement a custom processor for handling various types of events.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/meteora-activities
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variables:

```env
RPC_URL=...
```

This `RPC_URL` should point to the RPC endpoint you want to use for Solana transaction crawling.

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

This will start the transaction crawler, and the pipeline will begin processing transactions for the specified program ID (`METEORA_PROGRAM_ID`).

## Metrics

The example includes a basic metrics setup using `LogMetrics`. You can extend this by implementing your own metrics and passing them to the pipeline.
