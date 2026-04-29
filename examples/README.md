# Carbon Examples

This directory contains 9 standalone Carbon pipelines, each demonstrating a distinct capability of the framework — real-time streaming, historical backfill, account snapshotting, multi-version decoding, persistence with a query layer, and custom datasource implementation. Pick the example closest to your use case, copy the directory, and adapt it to your own program.

## Where to Start

If you're not sure which one fits, find your use case below:

| What you're building                                         | Start with                                           |
| ------------------------------------------------------------ | ---------------------------------------------------- |
| Real-time pipeline (gRPC)                                    | [`yellowstone-grpc`](yellowstone-grpc)               |
| Real-time pipeline (no Geyser, public RPC)                   | [`block-subscribe-rpc`](block-subscribe-rpc)         |
| Bounded-range historical backfill via archive                | [`jetstreamer`](jetstreamer)                         |
| Per-program transaction history backfill                     | [`transaction-crawler-rpc`](transaction-crawler-rpc) |
| Loading current state via RPC `getProgramAccounts`           | [`gpa-rpc`](gpa-rpc)                                 |
| Loading state from a validator snapshot file                 | [`snapshot-validator`](snapshot-validator)           |
| Indexing a program upgraded with a breaking IDL change           | [`versioned-decoders`](versioned-decoders)           |
| Persisting decoded data to Postgres and serving it via GraphQL | [`postgres-graphql`](postgres-graphql)             |
| Implementing your own `Datasource`                           | [`custom-datasource`](custom-datasource)             |

## Variants

A few examples ship with interchangeable upstream sources, defined side-by-side in `src/variants.rs`. The processor and pipeline wiring stay identical across variants — to swap, edit the single line in `main.rs` that constructs the datasource.

| Crate                                                | Default                       | Alternatives                                                                                                                        |
| ---------------------------------------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| [`yellowstone-grpc`](yellowstone-grpc)               | Yellowstone Geyser gRPC       | Helius LaserStream gRPC (`replay_enabled` for missed-slot replay); Jito Shredstream gRPC (pre-confirmation, no source-side filters) |
| [`transaction-crawler-rpc`](transaction-crawler-rpc) | RPC `getSignaturesForAddress` | Helius GTFA (hosted, with built-in slot/time/status filters)                                                                        |
| [`gpa-rpc`](gpa-rpc)                                 | RPC `getProgramAccounts`      | Helius gPA v2 (paginated, `changed_since_slot` for incremental syncs)                                                               |

## Running

Each example is a workspace member, so you can run any of them from the repo root:

```sh
cd examples/<example-name>
cargo run --release -p <example-name>-carbon-example
```

Required environment variables — and any external infrastructure (e.g. Postgres for `postgres-graphql`) — are documented in each example's `README.md`.
