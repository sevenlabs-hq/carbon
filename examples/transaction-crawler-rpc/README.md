# Transaction Crawler RPC Example

This example demonstrates how to backfill Meteora DLMM activity by polling a program's transaction history backwards from the newest signature. It polls **by program address, not by slot range** — the right shape when you only care about one program's transactions.

## Variants

This crate ships two interchangeable variants in [`src/variants.rs`](src/variants.rs):

- `variants::rpc(...)` — plain Solana RPC. Uses `getSignaturesForAddress` + `getTransaction`. Default in `main.rs`.
- `variants::helius_gtfa(...)` — Helius GTFA, the hosted equivalent. Adds built-in slot/time/status filters that aren't available on plain RPC.

Swap by editing the line in `main.rs`:

```rust
let datasource = variants::rpc(METEORA_PROGRAM_ID);
let datasource = variants::helius_gtfa(METEORA_PROGRAM_ID);
```

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/transaction-crawler-rpc
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example. Set the variables for whichever variant you're running:

```env
# RPC variant (default)
RPC_URL=https://api.mainnet-beta.solana.com
# Optional: stop the backfill once this signature is reached.
# UNTIL_SIGNATURE=4xK9j8Cxi...c9Pq

# Helius GTFA variant
# HELIUS_RPC_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_KEY

RUST_LOG=info
```

- `RPC_URL`: Solana RPC HTTP endpoint that supports `getSignaturesForAddress` and `getTransaction`. Required for the RPC variant. Public mainnet works but rate-limits hard against active programs — use a dedicated provider.
- `UNTIL_SIGNATURE`: signature to bound the backfill at. Optional. Without it the RPC variant runs until ctrl-C.
- `HELIUS_RPC_URL`: Helius RPC URL with API key embedded. Required for the GTFA variant.
- `RUST_LOG`: log level. `info` shows decoded Meteora DLMM instructions.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p transaction-crawler-rpc-carbon-example
```

The RPC variant walks signatures backwards in time, polling every 5 seconds for new ones. It only auto-exits if you set `UNTIL_SIGNATURE`. The GTFA variant paginates the slot window configured in `variants.rs` and exits naturally when drained.
