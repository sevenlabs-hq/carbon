# Postgres Sink Example

This example demonstrates persisting decoded SPL Token account updates to Postgres as JSON via Carbon's `postgres` feature (`PostgresJsonAccountProcessor`). It shows the JSON-blob persistence pattern (decoded accounts in a JSONB column, no per-program schema), `sqlx` connection pooling for async writes without blocking the pipeline, and `sqlx::migrate!` for idempotent schema setup at startup.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd carbon/examples/postgres-sink
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of the example:

```env
DATABASE_URL=postgres://carbon:carbon@localhost:5432/carbon_example
GEYSER_URL=https://your-yellowstone-endpoint:443
X_TOKEN=your-auth-token
RUST_LOG=info
```

- `DATABASE_URL`: Postgres connection string. Required. Default matches the bundled `compose.yaml`.
- `GEYSER_URL`: Yellowstone Geyser gRPC endpoint URL. Required.
- `X_TOKEN`: auth token for Yellowstone. Optional; required if your provider enforces it.
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
cargo run --release -p postgres-sink-carbon-example
```

The schema is applied on first run via `sqlx::migrate!`. The pipeline runs indefinitely — exit with ctrl-C. Inspect the table directly with:

```sh
psql postgres://carbon:carbon@localhost:5432/carbon_example \
  -c "SELECT __pubkey, __slot, data->>'type' AS kind FROM accounts LIMIT 5;"
```
