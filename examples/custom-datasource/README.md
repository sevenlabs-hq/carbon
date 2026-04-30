# Custom Datasource Example

This example demonstrates a complete custom `Datasource` implementation: an HTTP-polling source over public Solana RPC. It polls `getSignaturesForAddress` for the Jupiter v6 program at a fixed cadence, fetches each transaction with `getTransaction`, and emits `TransactionUpdate`s into the pipeline. The lesson is the **shape** of a real custom datasource — every concern a production source has to handle: cancellation at sleep boundaries, watermark pagination, transient-failure backoff, and emitting custom `Counter` metrics into Carbon's registry.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/custom-datasource
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example:

```env
RPC_URL=https://api.mainnet-beta.solana.com
RUST_LOG=info
```

- `RPC_URL`: Solana RPC HTTP endpoint that supports `getSignaturesForAddress` and `getTransaction`. Required. Public mainnet works for the example; production loads need a dedicated provider.
- `RUST_LOG`: log level. `info` shows decoded Jupiter swap instructions plus the periodic counters.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p custom-datasource-carbon-example
```

The pipeline polls Jupiter v6 signatures every 5 seconds and fetches each transaction. Runs indefinitely — exit with ctrl-C. If you hit public mainnet rate limits (frequent under load), reduce `batch_limit` in `src/main.rs` or use a dedicated RPC.
