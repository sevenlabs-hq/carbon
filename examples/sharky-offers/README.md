# Carbon Pipeline Example

This project demonstrates how to set up and run a Carbon pipeline that processes Solana accounts using the Sharky decoder. It leverages the custom `GpaBackfillDatasource` to fetch program accounts and the `RpcProgramSubscribe` as a data source to track live account updates for the specified program ID. The pipeline is designed to process account updates using the `SharkyAccountProcessor`.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/sharky-activities
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variables:

```env
RPC_URL=...
RUST_LOG=...
```

- `RPC_URL` should point to the Solana RPC URL you want to use for Solana program accounts crawling and live updates.

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

This will start both the backfill and live account subscription processes, and the pipeline will begin processing accounts for the specified program ID (`SHARKY_PROGRAM_ID`).
