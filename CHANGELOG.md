# Changelog

## [2.0.0] - 2026-09-04

Carbon 2.0 is the Agave 4 compatibility release. It upgrades the supported Solana dependency cohort, adds end-to-end Transaction V1 handling, and standardizes account-closure delivery without introducing the larger datasource and pipeline redesign planned for Carbon 3.

### Highlights

- Upgraded the workspace to the Agave 4.2-compatible Solana crates and Rust 1.96.1.
- Added Transaction V1 conversion and instruction traversal, including runtime inner instructions and V1 account metadata.
- Added strict Yellowstone protobuf conversion for legacy, V0, and V1 transactions.
- Classified zero-lamport account updates as `Update::AccountDeletion` consistently while preserving the final `Account` state.
- Added an optional starting slot to Helius LaserStream and opt-in tonic client reconnection to Yellowstone.
- Added native Codama event rendering, event-CPI discriminator exports, encoded discriminator handling, and base58 instruction-metadata serialization.
- Hardened instruction handling for short log events and invalid CPI stack heights, and sanitized exported Prometheus metric names.
- Updated generated projects from `@sevenlabs-hq/carbon-cli` and `@sevenlabs-hq/carbon-codama-renderer` 0.13 to depend on Carbon 2 crates.
- Corrected the CLI package's ESM and CommonJS export paths.
- Made the CLI reject GraphQL scaffolds without the Postgres storage layer they require.
- Fixed template lookup through the renderer's CommonJS entry point.
- Raised the npm packages' runtime requirement to Node.js 20.18.0.

### Migration from Carbon 1

#### Toolchain and dependencies

Carbon 2 requires Rust 1.96.1. The companion 0.13 CLI, renderer, and version-registry packages require Node.js 20.18.0 or newer. Update every directly declared Carbon crate to `2.0.0` and keep direct Solana dependencies compatible with the Agave 4.2 cohort. Do not mix Carbon 2 with Carbon 1 datasource or decoder crates in one dependency graph.

If Carbon types appear in your public API, update imports and fix type mismatches after resolving the new Solana crates. A single version of each Solana type crate should resolve in the final graph.

#### Transaction source configuration

Caller-owned RPC block configurations must opt into Transaction V1 and request a binary transaction encoding. Carbon cannot reconstruct the original transaction from `Json` or `JsonParsed` payloads.

```rust
RpcBlockSubscribeConfig {
    encoding: Some(UiTransactionEncoding::Base64),
    transaction_details: Some(TransactionDetails::Full),
    max_supported_transaction_version: Some(1),
    ..RpcBlockSubscribeConfig::default()
}
```

Use the equivalent fields on `RpcBlockConfig` when constructing `RpcBlockCrawler`. Carbon's RPC transaction crawler and Helius GTFA datasource set V1 and Base64 internally.

For Helius Atlas transaction subscriptions, override the SDK's JSON-parsed default and keep V1 enabled:

```rust
TransactionSubscribeOptions {
    encoding: Some(UiEnhancedTransactionEncoding::Base64),
    max_supported_transaction_version: Some(1),
    ..TransactionSubscribeOptions::default()
}
```

#### Account deletions

`AccountDeletion` now contains the source's final `account: Account`. Custom datasources should construct an `AccountUpdate` and call `into_update()` so every zero-lamport account is normalized at the same boundary.

The former `account_deletions_tracked` constructor argument and fields were removed. Remove that argument when constructing:

- `HeliusWebsocket`
- `LaserStreamGeyserClient`
- `YellowstoneGrpcGeyserClient`
- `StreamMessageClient`

If you construct `AccountDeletion` directly, provide its new `account` field.

#### Datasource configuration changes

- `LaserStreamClientConfig::new` has a new trailing `from_slot: Option<u64>` argument. Pass `None` to preserve the previous starting behavior, or use `LaserStreamClientConfig::default()` and set the field.
- Yellowstone tonic client reconnection is disabled by default. Opt in with `YellowstoneGrpcClientConfig::with_reconnect(ReconnectConfig)` when the upstream supports replay appropriate for your pipeline.

#### SPL Token account helpers

The `Mint::decode`, `Multisig::decode`, and `Token::decode` convenience methods in `carbon-token-program-decoder` were removed. Direct callers should unpack with `solana_program_pack::Pack` and convert the result:

```rust
let token = spl_token_interface::state::Account::unpack(data)
    .ok()
    .map(carbon_token_program_decoder::accounts::token::Token::from);
```

Normal decoding through `TokenProgramDecoder` requires no special migration.

### Carbon 2 datasource set

The Carbon 2 publication contains these twelve datasource crates:

- Helius Atlas WS, GPA V2, GTFA, and LaserStream
- RPC block crawler, block subscribe, GPA, program subscribe, and transaction crawler
- Stream message
- Validator snapshot
- Yellowstone gRPC

`carbon-jito-shredstream-grpc-datasource` and `carbon-jetstreamer-datasource` are not published on the 2.0 line. Jito Shredstream does not yet support Transaction V1, while Jetstreamer does not yet support the Solana v4 stack. Existing versions remain available, but they must not be selected as Carbon 2 dependencies.
