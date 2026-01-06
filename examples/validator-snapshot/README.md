# Validator Snapshot Example

This example demonstrates how to use the validator snapshot datasource to load and process accounts from a Solana validator snapshot. It supports filtering by program owners and/or specific account pubkeys.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/validator-snapshot
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variables:

```env
VALIDATOR_URL=https://example.com/snapshot
# OR
SNAPSHOT_PATH=/path/to/ledger

PROGRAM_OWNERS=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
ACCOUNT_IDS=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
METRICS_FLUSH_INTERVAL=10
```

- `VALIDATOR_URL` or `SNAPSHOT_PATH`: Snapshot source (one required)
- `PROGRAM_OWNERS`: Comma-separated list of program owner pubkeys (optional)
- `ACCOUNT_IDS`: Comma-separated list of specific account pubkeys (optional)
- At least one of `PROGRAM_OWNERS` or `ACCOUNT_IDS` must be set

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

This will load the snapshot and process all accounts matching the specified filters, logging each account with its details.
