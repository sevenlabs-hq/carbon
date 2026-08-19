# Carbon HyperSync Datasource

Bounded historical backfill over [HyperSync](https://docs.envio.dev/docs/HyperSync/overview),
Envio's columnar Solana archive, with the program filter pushed into the server. Only matched
transactions cross the network: for a program appearing in a few percent of transactions this
moves one to two orders of magnitude fewer bytes than crawling blocks and filtering client-side.

```rust
use carbon_hypersync_datasource::HyperSyncDatasource;
use carbon_pumpfun_decoder::{PumpfunDecoder, PROGRAM_ID as PUMPFUN};

let datasource = HyperSyncDatasource::new_mainnet_history(
    std::env::var("HYPERSYNC_TOKEN").ok(),
    430_000_000,          // from_slot, inclusive
    430_001_000,          // to_slot, exclusive
    vec![PUMPFUN],
);

carbon_core::pipeline::Pipeline::builder()
    .datasource(datasource)
    .instruction(PumpfunDecoder, MyProcessor)
    .build()?
    .run()
    .await?;
```

The range runs to completion and the pipeline shuts down cleanly, like
`carbon-jetstreamer-datasource`. Transactions are delivered complete: every instruction of a
matched transaction is fetched and reassembled, with inner instructions regrouped from the
archive's exact CPI paths, so instruction nesting and `absolute_path` values reproduce what a
Yellowstone or block-crawler source produces.

## Where it fits among the backfill sources

| Source | Reach | Filtering | Cost profile |
|---|---|---|---|
| `carbon-jetstreamer-datasource` | genesis to head minus ~2 epochs | client-side, after download | whole-chain bytes for the range |
| `carbon-rpc-block-crawler-datasource` | RPC retention | client-side, after download | one `getBlock` per slot |
| `carbon-rpc-transaction-crawler-datasource` | RPC retention | server-side by address | one `getTransaction` per signature |
| this crate | last ~5 months (public endpoint) | server-side by program | matched transactions only |

The natural composition for deep history is Jetstreamer below the HyperSync floor and this
datasource above it; the archive behind Jetstreamer lags the head by roughly two epochs, which
is exactly the window HyperSync serves.

## Fidelity limits

Stated in full in the crate docs; the short list:

- Only **successful** transactions are served (the archive stores no instruction rows for failed
  ones; the block crawler also skips failed transactions).
- Vote transactions are excluded, and `TransactionUpdate.index` is the dense non-vote rank.
- The message header is synthesized (`num_required_signatures` from the signature count, readonly
  counts zero), so `AccountMeta` writability flags are approximate. No decoder in this repository
  reads them.
- `meta.log_messages`, balances, token balances, `return_data` and rewards are not populated in v1.

## Environment

The public endpoint requires a bearer token, free at <https://envio.dev>. Pass it as
`bearer_token`; the example reads `HYPERSYNC_TOKEN`.
