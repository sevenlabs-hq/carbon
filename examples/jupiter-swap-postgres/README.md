## Jupiter Swap Postgres Example

This example uses `RpcBlockCrawler` to fetch Solana blocks over a slot range, decodes Jupiter swap instructions with `JupiterSwapDecoder`, and writes the decoded instruction data into Postgres.

### Environment

Create `examples/jupiter-swap-postgres/.env` with:

```env
RPC_URL=https://your-solana-rpc
DATABASE_URL=postgres://user:password@host:5432/database
LOG_LEVEL=info
BLOCK_CRAWLER_START_SLOT=<SLOT>
BLOCK_CRAWLER_END_SLOT=<SLOT>
BLOCK_CRAWLER_MAX_CONCURRENT_REQUESTS=1
BLOCK_CRAWLER_CHANNEL_BUFFER_SIZE=10
```

### Run

```sh
cargo run -p carbon-jupiter-swap-postgres-example
```

On startup, the example connects to Postgres, applies the Jupiter swap instruction migration, then begins crawling the requested slot range.

#### Questions

Telegram: [t.me/QuanDeFi](https://t.me/QuanDeFi)  
Email: q@quandefi.co
