# snapshot-validator

Loads accounts from a Solana validator snapshot — either from a remote tarball URL or a locally-extracted ledger directory. Filters at the snapshot layer by program owner and/or specific account pubkey, decodes each match through a generic passthrough decoder, and also wires the `account_deletions` pipe (the snapshot source produces both). Exits cleanly via `ProcessPending` when the snapshot drains.

## Setup Instructions

### Step 1: Clone the Repository

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/snapshot-validator
```

### Step 2: Set Environment Variables

Create a `.env` file in the example directory:

```env
# Snapshot source — set ONE of:
VALIDATOR_URL=https://api.mainnet-beta.solana.com/snapshot.tar.bz2
# SNAPSHOT_PATH=/path/to/extracted/ledger

# Filters — set at least ONE of PROGRAM_OWNERS or ACCOUNT_IDS:
PROGRAM_OWNERS=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
# ACCOUNT_IDS=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v

RUST_LOG=info
```

- `VALIDATOR_URL` — URL of a validator snapshot tarball. Downloaded and extracted on the fly.
- `SNAPSHOT_PATH` — local path to an already-extracted ledger directory (containing `genesis.tar.bz2`, `snapshots/`, `accounts/`).
- One of `VALIDATOR_URL` or `SNAPSHOT_PATH` is required.
- `PROGRAM_OWNERS` — comma-separated list of program owner pubkeys. Only accounts owned by one of these programs are loaded.
- `ACCOUNT_IDS` — comma-separated list of specific account pubkeys to load.
- At least one of `PROGRAM_OWNERS` or `ACCOUNT_IDS` is required.
- `RUST_LOG` — `info` shows each loaded account and any deletions.

Mainnet snapshots are ~50–80GB compressed — make sure you have the disk and memory.

### Step 3: Build

```sh
cargo build --release
```

### Step 4: Run

```sh
cargo run --release -p snapshot-validator-carbon-example
```
