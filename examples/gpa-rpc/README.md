# gPA RPC Example

This example demonstrates a one-shot account-state snapshot via RPC `getProgramAccounts`. It fetches every account owned by the Marginfi v2 program, decodes each into the typed `MarginfiV2Account` enum, and exits cleanly when the source drains.

## Variants

This crate ships two interchangeable variants in [`src/variants.rs`](src/variants.rs):

- `variants::rpc(...)` — plain `getProgramAccounts` against any RPC. Default in `main.rs`.
- `variants::helius_gpa_v2(...)` — Helius's enhanced gPA endpoint. Paginated; adds `changed_since_slot` for incremental syncs.

Swap by editing the line in `main.rs`:

```rust
let datasource = variants::rpc(MARGINFI_PROGRAM_ID);
// let datasource = variants::helius_gpa_v2(MARGINFI_PROGRAM_ID);
```

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/gpa-rpc
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example. Set the variable for whichever variant you're running:

```env
# RPC variant (default)
RPC_URL=https://api.mainnet-beta.solana.com

# Helius gPA v2 variant
# HELIUS_RPC_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_KEY

RUST_LOG=info
```

- `RPC_URL`: Solana RPC HTTP endpoint that supports `getProgramAccounts`. Required for the RPC variant. Public mainnet works for small/medium programs but is rate-limited; production GPA against large programs needs a dedicated provider.
- `HELIUS_RPC_URL`: Helius RPC URL with API key embedded. Required for the Helius gPA v2 variant.
- `RUST_LOG`: log level. `info` shows each decoded Marginfi account.

For programs with millions of accounts (e.g. SPL Token), use [`snapshot-validator`](../snapshot-validator) — RPC GPA is impractical at that scale.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p gpa-rpc-carbon-example
```

The pipeline loads every matching account once, then exits with `snapshot complete`.
