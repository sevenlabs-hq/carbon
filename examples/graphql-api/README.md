# GraphQL API Example

This example demonstrates a real-time Carbon pipeline that listens for Token-2022 mint accounts via a Yellowstone Geyser gRPC stream, holds the latest state in memory, and exposes it over a Juniper-powered GraphQL API on `:8080`. Built on Carbon's `graphql` feature, which ships pre-wired Juniper + Axum integration. The GraphQL server starts immediately so first writes from the pipeline are queryable as soon as they arrive.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/graphql-api
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example:

```env
GEYSER_URL=https://your-yellowstone-endpoint:443
X_TOKEN=your-auth-token
BIND_ADDR=0.0.0.0:8080
RUST_LOG=info
```

- `GEYSER_URL`: Yellowstone gRPC endpoint URL. Required.
- `X_TOKEN`: auth token for Yellowstone. Optional; required if your provider enforces it.
- `BIND_ADDR`: address the GraphQL endpoint is served on. Falls back to `0.0.0.0:8080` if unset.
- `RUST_LOG`: log level. `info` shows periodic pipeline counters.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building, run the pipeline using:

```sh
cargo run --release -p graphql-api-carbon-example
```

The server starts immediately and the pipeline begins indexing. Open the bundled GraphiQL playground at [http://localhost:8080/graphiql](http://localhost:8080/graphiql) and try:

```graphql
{
  mints(limit: 5) {
    pubkey
    slot
    decimals
    supply
  }
}
```

If `mints` is empty for the first few seconds, that's expected — the pipeline is still receiving its first updates from Yellowstone. Runs indefinitely — exit with ctrl-C.
