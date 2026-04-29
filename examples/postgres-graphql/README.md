# Postgres + GraphQL Example

This example demonstrates the canonical "production sink + query layer" pattern: a Yellowstone Geyser gRPC stream feeds decoded SPL Token account updates into Postgres (via Carbon's `postgres` feature and `PostgresJsonAccountProcessor`), and a Juniper-powered GraphQL API on `:8080` (via Carbon's `graphql` feature) serves the persisted state by querying the Postgres `accounts` table directly. The HTTP server starts immediately so first writes are queryable as soon as they arrive.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/postgres-graphql
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example:

```env
DATABASE_URL=postgres://carbon:carbon@localhost:5432/carbon_example
GEYSER_URL=https://your-yellowstone-endpoint:443
X_TOKEN=your-auth-token
BIND_ADDR=0.0.0.0:8080
RUST_LOG=info
```

- `DATABASE_URL`: Postgres connection string. Required. Default matches the bundled `compose.yaml`.
- `GEYSER_URL`: Yellowstone Geyser gRPC endpoint URL. Required.
- `X_TOKEN`: auth token for Yellowstone. Optional; required if your provider enforces it.
- `BIND_ADDR`: address the GraphQL endpoint is served on. Falls back to `0.0.0.0:8080` if unset.
- `RUST_LOG`: log level. `info` for periodic upsert metrics.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

If you don't already have a Postgres instance, start the bundled one first:

```sh
docker compose up -d
```

Then run the pipeline:

```sh
cargo run --release -p postgres-graphql-carbon-example
```

The schema is applied on first run via `sqlx::migrate!`. The pipeline runs indefinitely — exit with ctrl-C.

## Querying

Open the bundled GraphiQL playground at [http://localhost:8080/graphiql](http://localhost:8080/graphiql) and try:

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

Or look up a specific mint by pubkey:

```graphql
{
  mint(pubkey: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") {
    pubkey
    slot
    decimals
    supply
  }
}
```

Both queries hit Postgres directly via `sqlx`, reading from the `accounts` JSONB column. Empty results for the first few seconds are expected — the pipeline is still receiving its first updates from Yellowstone.

You can also inspect the table directly:

```sh
psql postgres://carbon:carbon@localhost:5432/carbon_example \
  -c "SELECT __pubkey, __slot, data->>'type' AS kind FROM accounts LIMIT 5;"
```
