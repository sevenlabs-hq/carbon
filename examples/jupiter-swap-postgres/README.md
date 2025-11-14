## Jupiter Swap → Postgres Pipeline Example

This example wires a Carbon pipeline that:

1. Listens to Jupiter swap transactions on Solana mainnet using either the `RpcTransactionCrawler` (default) or `RpcBlockCrawler` datasource.
2. Decodes swap instructions with `carbon-jupiter-swap-decoder`.
3. Persists a structured view of each swap and its route metadata into Postgres using sqlx migrations.

The goal of this document is to walk through the full setup (from Docker + Postgres to `.env` wiring) and describe exactly which tables and columns are populated.

#### Pipeline Overview

```bash
+---------------------------+
|  Jupiter Program (JUP6L…) |
|  Swap instructions on     |
|  Solana mainnet           |
+-------------+-------------+
              |
              |  getSignaturesForAddress /
              |  getTransaction / getSlot / getblock
              v
+-------------+-------------+
|        Solana RPC         |
|         endpoint          |
+------+------+-------------+
       |      |
       |      |
       |      +---------------------------+
       |                                  |
       v                                  v
+------+---------------+        +---------+--------------+
| RpcTransactionCrawler|        | RpcBlockCrawler        |
| (historical polling) |        | (live block streaming) |
+----------+-----------+        +-----------+------------+
           |                                |
           +--------------+-----------------+
                          |
                          v
            +-------------+--------------+
            | carbon-jupiter-swap-decoder|
            |  + JupiterSwapProcessor    |
            +-------------+--------------+
                          |
                          v
            +-------------+--------------+
            | JupiterSwapRepository      |
            | (sqlx migrations)          |
            +-------------+--------------+
                          |
                          v
   +----------------------+------+------------------------------+
   | Postgres (Docker)                                        |
   | Tables populated:                                        |
   |  • jupiter_route_instructions                            |
   |  • jupiter_route_instruction_accounts                    |
   |  • jupiter_route_plan_steps                              |
   |  • jupiter_swap_hops                                     |
   |  • jupiter_swap_event_envelopes                          |
   |  • venue_labels                                          |
   |  • mint_reference_data                                   |
   +----------------------------------------------------------+
```

---

### 1. Prerequisites

| Requirement | Why it is needed |
|-------------|------------------|
| **Ubuntu** | Same OS used for testing |
| **Docker** | To run Postgres locally without polluting your host system. |
| **Postgres** | (Via Docker) used to store the decoded data. |
| **Rust toolchain (1.82+)** | Installs `rustc` and `cargo`, which build and run the example binary. |
| **System libs: `build-essential`, `pkg-config`, `libssl-dev`** | Needed by sqlx and other crates that compile native dependencies. |
| **Access to a Solana RPC endpoint** | The crawler relies on HTTP `getSignaturesForAddress`, `getTransaction`, `getSlot` and `getBlock` . |

Install Docker first (https://docs.docker.com/get-docker/). Once Docker is running you can provision a Postgres container.

---

### 2. Provision Postgres with Docker

```bash
# Creates persistent storage for Postgres
docker volume create pgdata

# Pulls the Postgres container and runs it locally, exposing port 5432
docker run -d --name pg \
  -e POSTGRES_USER=USERNAME \
  -e POSTGRES_PASSWORD=PASSWORD \
  -e POSTGRES_DB=DATABASE_NAME \
  -p 127.0.0.1:5432:5432 \
  -v pgdata:/var/lib/postgresql/data \
  --health-cmd='pg_isready -U USERNAME || exit 1' \
  postgres:17

# (Optional) to connect with psql via docker exec for admin & query tasks
docker exec -it pg psql -U USERNAME -d DATABASE_NAME
```

> **Notes**
> - The environment values (`USERNAME` / `PASSWORD` / `DATABSE_NAME`) must match what you put into `.env`.
> - The `pg-data` volume keeps data between container restarts. Remove the volume to wipe the DB.

---

### 3. Wire Environment Variables

Inside `examples/jupiter-swap-postgres/.env` you need:

```env
DATABASE_URL=postgres://USERNAME:PASSWORD@127.0.0.1:5432/DATABASE_NAME
RPC_URL=https://<your-rpc-provider-url>
DATASOURCE=rpc_transaction_crawler # or rpc_block_crawler
# RATE_LIMIT is optional, defaults to 10 requests/sec to work with free tier RPC nodes
RATE_LIMIT=10
```

* `DATABASE_URL` must line up with the container credentials `postgresql://[user[:password]@][host][:port][/dbname]`.
* `RPC_URL` should be an HTTPS endpoint that allows `getSignaturesForAddress/getTransaction`. (Triton, Alchemy, Helius etc.)
* `DATASOURCE` toggles which Carbon datasource powers the pipeline. Leave it at `rpc_transaction_crawler` for the historical/default behavior (signature polling), or switch to `rpc_block_crawler` to stream blocks as they are produced. The block crawler automatically starts from the most recent confirmed slot and uses its built-in defaults (100 ms interval, 10 concurrent requests) so you get roughly the same 10 req/s throughput as the transaction crawler without tweaking extra variables.
* `RATE_LIMIT` throttles only the transaction crawler path. It is ignored by the block crawler because `getBlock` requests are self-throttled by the interval/concurrency settings above.

The binary loads this `.env` file via `dotenv::dotenv().ok();` so you do **not** need to export anything globally.

---

### 4. Run the Example

```bash
cd examples/jupiter-swap-postgres
cargo run
```

What happens during startup:

1. Logging is set up (stdout + rotating `/logs/run-*.log` files).
2. A sqlx migration creates all Jupiter-specific tables if they do not already exist.
3. The datasource selected via `DATASOURCE` is initialized:
   - `RpcTransactionCrawler` polls for signatures mentioning the Jupiter program (`JUP6L...`) when backfilling via HTTP.
   - `RpcBlockCrawler` fetches the most recent slot via `getSlot` and immediately begins streaming new blocks forward using the same ~10 req/s pacing.
4. For each decoded instruction, `JupiterSwapProcessor` writes structured records via `JupiterSwapRepository`.

You can inspect progress / warnings via the generated log file or by watching stdout. To confirm data is flowing, connect with `psql` and query the tables listed below.

---

### 5. Data Model

Once the pipeline runs you should see the following tables (listed in dependency order):

#### `jupiter_route_instructions`
| Column | Meaning |
|--------|---------|
| `__signature`, `__instruction_index`, `__stack_height` | Composite primary key identifying the precise instruction within a transaction. |
| `__slot` | Slot in which the transaction landed. |
| `variant` | Jupiter instruction variant (e.g., `Route`, `RouteV2`, `SharedAccountsRoute`). |
| `shared_accounts_id` | The shared accounts ID emitted by Jupiter for shared routes (optional). |
| `in_amount` / `out_amount` | Raw u64 amounts (lamports / token base units) for in/out legs when available. |
| `quoted_in_amount` / `quoted_out_amount` | Quoted amounts from the route plan. |
| `slippage_bps`, `platform_fee_bps`, `positive_slippage_bps` | Risk/fee parameters inside the instruction. |
| `source_mint`, `destination_mint` | Inferred public keys for the route source and destination mints. |
| `route_plan` | JSONB representation of the route plan exactly as Jupiter emitted it. |
| `route_plan_version` | Distinguishes V1 vs V2 plan structures. |
| `created_at`, `updated_at` | Timestamps managed by the repository. |

#### `jupiter_route_instruction_accounts`
Contains every `AccountMeta` supplied to the instruction (position, pubkey, signer, writable flags). Useful for auditing who paid fees, which pools were touched, etc.

#### `jupiter_route_plan_steps`
Each row represents a normalized step in Jupiter’s route plan (step index, swap variant, accounts used, JSON payload). This roughly mirrors the logical hops Jupiter intends to execute.

#### `jupiter_swap_hops`
Represents *actual* on-chain swap events observed in the decoded instruction stream. Key columns:

| Column | Notes |
|--------|-------|
| `hop_index` | Order of the hop within the instruction. |
| `event_type` | Enumerated event (currently `SwapEvent`). |
| `input_mint` / `output_mint` | Token mints involved in this hop. |
| `input_amount` / `output_amount` | Amounts in base units (BigDecimal). |
| `input_amount_decimal` / `output_amount_decimal`, `price` | Optional normalized representations (set once mint decimals are known). |
| `swap_variant` / `swap_json` | The venue type (Raydium, Orca, Pump, etc.) and the JSON event payload. |
| `venue_label`, `venue_category` | Human-readable labels derived from `venue_labels`. |
| `metadata` | JSON describing event index, normalization state, links to route steps, etc. |

#### `jupiter_swap_event_envelopes`
Raw JSON envelopes for every swap-related event, keyed by signature/instruction/stack height/event index. This is useful for replay or reprocessing.

#### `venue_labels`
Simple reference table mapping each `swap_variant` to a label/category (auto-populated the first time a variant appears).

#### `mint_reference_data`
Holds decimals + symbol for mints as they are discovered. When a hop references a mint that we do not know, the hop is marked `normalization_pending` until decimals are backfilled.

---

### 6. Typical Workflow

1. **Start Postgres** (`docker start pg`).
2. **Confirm RPC credentials** are valid (e.g., curl `{"jsonrpc":"2.0","id":1,"method":"getHealth"}`).
3. **Adjust `.env`** if you want a different `DATASOURCE`, `RATE_LIMIT`, database host, or RPC endpoint.
4. **Run the pipeline** (`cargo run`). Leave it running to keep ingesting swaps; restart as needed.
5. **Inspect data** with `psql` queries such as:
   ```sql
   SELECT variant, COUNT(*) FROM jupiter_route_instructions GROUP BY 1;
   SELECT swap_variant, COUNT(*) FROM jupiter_swap_hops GROUP BY 1 ORDER BY 2 DESC;
   ```

---

### 7. Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| `Failed to run migrations: password authentication failed` | `.env` creds do not match container env | Update `DATABASE_URL` or recreate the container with matching `POSTGRES_*` values. |
| RPC related warnings or errors in logs | RPC rate-limiting or methods not supported | Lower `RATE_LIMIT`, upgrade RPC plan or change RPC provider. |
| Block crawler logs `Failed to fetch the most recent slot...` | Temporary RPC hiccup retrieving `getSlot` | Re-run after the RPC endpoint recovers or switch back to `rpc_transaction_crawler`. |
| `0 processed` forever | RPC endpoint not returning Jupiter transactions | Verify RPC connection, run `curl getSignaturesForAddress JUP6...`, upgrade RPC plan or switch providers. |

This README should give you everything needed to run, debug, and understand the data emitted by `examples/jupiter-swap-postgres`.

#### Contact
Email: q@quandefi.co \
Telegram: [@QuanDeFi](https://t.me/QuanDeFi)
