<div align="center">
  <h1>Carbon</h1>
  <p><strong>Rust framework for building Solana indexers and data pipelines.</strong></p>

  <p>
    <a href="https://crates.io/crates/carbon-core"><img alt="Crates.io" src="https://img.shields.io/crates/v/carbon-core"></a>
    <a href="https://www.npmjs.com/package/@sevenlabs-hq/carbon-cli"><img alt="npm" src="https://img.shields.io/npm/v/@sevenlabs-hq/carbon-cli"></a>
    <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-MIT-blue"></a>
  </p>

  <p>
    <a href="#-overview">Overview</a> ·
    <a href="#-quick-start">Quick Start</a> ·
    <a href="#-capabilities">Capabilities</a> ·
    <a href="#-maintained-packages">Packages</a> ·
    <a href="#-examples">Examples</a>
  </p>
</div>

---

## ✨ Overview

Carbon is built around a pipeline: datasources stream Solana updates into the
runtime, decoders turn raw account and transaction data into typed Rust
structures, and processors handle the decoded output in order to build data
pipelines and indexers for your applications. Each part is modular, so you can
swap RPC streams for Geyser streams, generated decoders for custom ones, or
logging processors for Postgres, GraphQL, analytics, bots, and other
application-specific sinks.

---

## 🚀 Quick Start

Start with a decoder. Use one of Carbon's existing program decoders, or generate
one from your program IDL:

```sh
npx @sevenlabs-hq/carbon-cli parse \
  --idl ./idl.json \
  --out-dir ./my-program-decoder \
  --name my-program
```

Then add a datasource and wire them together in a pipeline:

```rust
use carbon_core::{error::CarbonResult, pipeline::Pipeline};
use carbon_log_metrics::LogMetrics;
use carbon_my_program_decoder::{MyProgramDecoder, PROGRAM_ID};
use carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe};
use solana_client::rpc_config::RpcBlockSubscribeFilter;

#[tokio::main]
async fn main() -> CarbonResult<()> {
    let datasource = RpcBlockSubscribe::new(
        "wss://api.mainnet-beta.solana.com".to_string(),
        Filters::new(
            RpcBlockSubscribeFilter::MentionsAccountOrProgram(PROGRAM_ID.to_string()),
            None,
        ),
    );

    Pipeline::builder()
        .datasource(datasource)
        .metrics(std::sync::Arc::new(LogMetrics::new()))
        .instruction(MyProgramDecoder, MyProcessor)
        .build()?
        .run()
        .await
}
```

Then implement a processor for the typed instruction data you care about:

```rust
use carbon_core::{
    error::CarbonResult,
    instruction::InstructionProcessorInputType,
    processor::Processor,
};
use carbon_my_program_decoder::instructions::MyProgramInstruction;

struct MyProcessor;

impl Processor<InstructionProcessorInputType<'_, MyProgramInstruction>>
    for MyProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, MyProgramInstruction>,
    ) -> CarbonResult<()> {
        log::info!("instruction: {:?}", input.decoded_instruction);
        Ok(())
    }
}
```

The same pattern works with any decoder and any datasource: import the decoder
for the Solana program you care about, import the datasource that matches your
latency or backfill requirements, then implement a processor for the typed data
you want to handle.

---

## 🧰 Capabilities

- Stream transactions, account updates, account deletions, and block metadata.
- Decode instructions, nested CPIs, accounts, and emitted events into typed Rust
  structures.
- Build real-time indexers, historical backfills, snapshot loaders, and hybrid
  data pipelines with the same processor model.
- Combine multiple datasources and route updates with filters.
- Generate decoder crates from Anchor or Codama IDLs.
- Persist decoded data with typed Postgres rows or generic JSONB processors.
- Expose indexed data through GraphQL using the built-in Juniper and Axum
  helpers.
- Export pipeline metrics through logs or Prometheus.

---

## 📦 Maintained Packages

Carbon includes maintained datasources and decoders so most indexers can start
from existing building blocks instead of custom ingestion or decoding code.

### 🔌 Datasources

Datasource crates cover the common ways Solana data is consumed:

- **Geyser streams** for low-latency production indexers.
- **Solana RPC** for simple setups and public RPC compatibility.
- **Historical and snapshot sources** for backfills, range replay, and loading
  current account state.
- **Hosted provider APIs** (e.g. Helius) for provider-specific streaming and
  historical access.
- **Adapter datasources** for bringing your own message stream.

See [`datasources/`](datasources) for the full list.

### 🧬 Decoders

Decoder crates cover widely used Solana programs across core SPL programs,
DeFi, NFTs, infrastructure, and more. They expose typed accounts, instructions,
events, and optional serde, Postgres, and GraphQL integrations.

Use an existing decoder when one is available, or generate one from an Anchor or
Codama IDL with `carbon-cli parse`.

See [`decoders/`](decoders) for the full list.

---

## 🧪 Examples

The [`examples/`](examples) directory contains runnable indexers for the common
ways Carbon is used: Geyser streaming, public RPC streaming, transaction
backfills, account-state loading, validator snapshots, custom datasources, and
Postgres-backed GraphQL APIs.

Each example is a workspace crate and can be run from the repository root:

```sh
cargo run --release -p block-subscribe-rpc-carbon-example
```

---

## 🤝 Contributing

Contributions are welcome. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the
development setup, local checks, and contribution guidelines.

## 📄 License

Carbon is licensed under the [MIT license](LICENSE).
