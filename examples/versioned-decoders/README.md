# Versioned Decoders Example

This example demonstrates how to index a program that has been upgraded with a breaking IDL change. Same program ID, two on-chain layouts (V1 → V2 at a known slot). Each decoder gets its own `.instruction()` registration with a `SlotRangeFilter` pinning it to the slots its layout actually applies to. A single processor handles both versions by resolving them to a shared output type.

The example uses **real Codama-generated decoder crates** (`decoder-v1`, `decoder-v2`) and a **mock datasource** so it runs with zero infrastructure.

## What It Shows

A fictional DEX program upgrades at **slot 500, tx_index 10**, adding a `fee_tier: u8` field to its `Swap` instruction:

- **V1** `Swap { amount_in: u64, min_amount_out: u64 }`
- **V2** `Swap { amount_in: u64, min_amount_out: u64, fee_tier: u8 }`

The pipeline registers both decoders with `SlotRangeFilter` so each instruction hits exactly one decoder based on when it occurred.

## Why two `.instruction()` registrations instead of `InstructionDecoderCollection`?

`InstructionDecoderCollection` requires `Eq + Hash + Serialize` on every inner instruction enum. Codama-generated decoders only derive `Clone + Debug + PartialEq`, so two `.instruction()` registrations with slot-based routing is the path that compiles today.

## Structure

```
examples/versioned-decoders/
├── Cargo.toml          — the indexer (versioned-decoders-carbon-example)
├── src/main.rs         — MockDatasource + SlotRangeFilter pipeline
├── decoder-v1/         — generated: carbon-simple-dex-decoder-v1
├── decoder-v2/         — generated: carbon-simple-dex-decoder-v2
└── idls/
    ├── dex_v1.json     — Anchor IDL: swap (2 args)
    └── dex_v2.json     — Anchor IDL: swap (3 args, added fee_tier)
```

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/versioned-decoders
```

### Step 2: Set Environment Variables

No environment variables are required. Optionally:

```env
RUST_LOG=info
```

- `RUST_LOG`: log level. `info` shows the decoded V1 / V2 swaps as the synthetic transactions stream through the pipeline.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p versioned-decoders-carbon-example
```

The mock datasource emits four synthetic transactions straddling the upgrade boundary at slot 500 / tx_idx 10, then exits. Both decoders run against every instruction; only the one whose layout matches yields data, and the slot filter ensures only the version deployed at that slot is allowed through.

## Expected Output

```
INFO  === Versioned Decoders Example ===
INFO  Program upgrade at slot=500, tx_index=10
INFO  [V1] Swap | slot=499 tx_idx=Some(0) | amount_in=1000000 min_amount_out=990000
INFO  [V1] Swap | slot=500 tx_idx=Some(9) | amount_in=2000000 min_amount_out=1980000
INFO  [V2] Swap | slot=500 tx_idx=Some(10) | amount_in=3000000 min_amount_out=2970000 fee_tier=5
INFO  [V2] Swap | slot=501 tx_idx=Some(0) | amount_in=4000000 min_amount_out=3960000 fee_tier=10
INFO  Done.
```

## Routing Logic

```
slot=499, tx_idx=0    → V1 filter: slot < 500 ✅  |  V2 filter: slot < 500 ❌
slot=500, tx_idx=9    → V1 filter: slot=500, idx < 10 ✅  |  V2 filter: idx < 10 ❌
slot=500, tx_idx=10   → V1 filter: slot=500, idx >= 10 ❌  |  V2 filter: idx >= 10 ✅  ← upgrade point
slot=501, tx_idx=0    → V1 filter: slot > 500 ❌  |  V2 filter: slot > 500 ✅
```

## SlotRangeFilter API

```rust
// Open-ended from slot S, tx index I (inclusive lower bound)
SlotRangeFilter::from(slot, Some(tx_index))

// Open-ended up to slot S, tx index I (exclusive upper bound at boundary)
SlotRangeFilter::to(slot, Some(tx_index))

// Closed range between two points
SlotRangeFilter::between(from_slot, Some(from_tx_index), to_slot, Some(to_tx_index))
```

Boundary semantics:
- At `from_slot`: accepts `tx_index >= from_tx_index`
- At `to_slot`: accepts `tx_index < to_tx_index`
- Pass `None` for tx_index to match any transaction in that slot

## Regenerating the Decoders

The two decoder crates were generated from the IDLs in `idls/` using the Codama CLI. To regenerate (e.g. after editing an IDL), run from the carbon repo root after `pnpm build`:

```sh
# V1 decoder
node packages/cli/dist/cli.js parse \
  --idl examples/versioned-decoders/idls/dex_v1.json \
  --out-dir examples/versioned-decoders/decoder-v1 \
  --name simple-dex \
  --version-name v1 \
  --as-crate \
  --standard anchor \
  --with-postgres false \
  --with-graphql false \
  --standalone true

# V2 decoder
node packages/cli/dist/cli.js parse \
  --idl examples/versioned-decoders/idls/dex_v2.json \
  --out-dir examples/versioned-decoders/decoder-v2 \
  --name simple-dex \
  --version-name v2 \
  --as-crate \
  --standard anchor \
  --with-postgres false \
  --with-graphql false \
  --standalone true
```

The `--version-name` flag appends `-v1` / `-v2` to the crate name so both can coexist in the workspace. After regeneration, edit each decoder's `Cargo.toml` to:

1. Remove the `[workspace]` line the CLI adds (conflicts with the parent workspace).
2. Switch dependencies to `{ workspace = true }` form.

The discriminator for an Anchor instruction is the first 8 bytes of `sha256("global:<instruction_name>")`. Compute with:

```sh
node -e "const c=require('crypto');console.log(Array.from(c.createHash('sha256').update('global:swap').digest().slice(0,8)))"
```
