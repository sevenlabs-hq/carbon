# Carbon

Carbon is a lightweight indexing framework for Solana.

It provides a modular pipeline for building end-to-end indexers by streaming on-chain data, decoding updates, and processing them through composable handlers.

---

## Components

### Pipeline

The core orchestration layer. It connects datasources, pipes, and processors into a single execution flow.

### Datasources

Async producers that feed updates into the pipeline.

Each update is one of:

- `AccountUpdate`
- `TransactionUpdate`
- `AccountDeletion`
- `BlockDetails`

### Pipes

Type-specific routing layers between datasources and processors.

- **Account Pipes** — decode and process account updates.
- **Account Deletion Pipes** — handle account deletion events.
- **Instruction Pipes** — decode transaction instructions one by one.
- **Transaction Pipes** — decode full transactions via an `InstructionDecoderCollection` and process them as a unit.
- **Block Details Pipes** — process block-level metadata.

---

## Metrics

Pipeline metrics are collected via a global `MetricsRegistry`.

Two official sinks are available:

| Crate                       | Description                                          | Setup  |
| --------------------------- | ---------------------------------------------------- | ------ |
| `carbon-log-metrics`        | Logs metrics to stdout for debugging and development | Easy   |
| `carbon-prometheus-metrics` | Exposes `/metrics` endpoint for Prometheus scraping  | Medium |

---

## Usage

### Basic Setup

```rust
use {
    carbon_core::{
        error::CarbonResult,
        pipeline::Pipeline,
    },
    carbon_log_metrics::LogMetrics,
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    carbon_token_program_decoder::{TokenProgramDecoder, PROGRAM_ID},
    solana_client::rpc_config::RpcBlockSubscribeFilter,
    std::{env, sync::Arc},
};

#[tokio::main]
async fn main() -> CarbonResult<()> {
    let datasource = RpcBlockSubscribe::new(
        env::var("RPC_WS_URL").expect("RPC_WS_URL"),
        Filters::new(
            RpcBlockSubscribeFilter::MentionsAccountOrProgram(
                PROGRAM_ID.to_string(),
            ),
            None,
        ),
    );

    Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(TokenProgramDecoder, MyProcessor)
        .build()?
        .run()
        .await
}
```

For real-world usage patterns, see [`examples/`](examples/) — each example is a complete working pipeline.

---

## Generating Decoders from IDL

Decoders convert raw Solana account or instruction data into strongly typed Rust structures.

Carbon includes a CLI that generates decoders from:

- Anchor IDLs
- Codama IDLs
- On-chain program addresses

### CLI Installation

```sh
npm install -g @sevenlabs-hq/carbon-cli
```

Or run directly:

```sh
npx @sevenlabs-hq/carbon-cli
```

### CLI Usage

```sh
carbon-cli parse [OPTIONS]
carbon-cli scaffold [OPTIONS]
```

### Example: Generate Decoder from Program

```sh
carbon-cli parse \
  --idl LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo \
  --url mainnet-beta \
  --out-dir ./decoders
```

### Example: Scaffold Project

```sh
carbon-cli scaffold \
  --name my-indexer \
  --idl ./idl.json \
  --data-source yellowstone-grpc
```

---

## Implementing Processors

Processors are async handlers that receive typed data from the pipeline.

```rust
use carbon_core::{
    account::AccountProcessorInputType,
    error::CarbonResult,
    processor::Processor,
};

pub struct MyAccountProcessor;

impl Processor<AccountProcessorInputType<'_, MyCustomAccountData>>
    for MyAccountProcessor
{
    async fn process(
        &mut self,
        data: &AccountProcessorInputType<'_, MyCustomAccountData>,
    ) -> CarbonResult<()> {
        // user logic here

        Ok(())
    }
}
```

---

## Datasources

Carbon supports multiple datasource types depending on performance and infrastructure needs.

### Solana RPC

Simple and low-cost sources using standard RPC:

- `carbon-rpc-block-subscribe-datasource`
- `carbon-rpc-program-subscribe-datasource`
- `carbon-rpc-transaction-crawler-datasource`
- `carbon-rpc-block-crawler-datasource`
- `carbon-rpc-gpa-datasource`

Example: [`block-subscribe-rpc`](examples/block-subscribe-rpc), [`gpa-rpc`](examples/gpa-rpc), [`transaction-crawler-rpc`](examples/transaction-crawler-rpc)

### Helius

Streaming APIs powered by Helius infrastructure.

- `carbon-helius-atlas-ws-datasource`
- `carbon-helius-laserstream-datasource`
- `carbon-helius-gpa-v2-datasource`
- `carbon-helius-gtfa-datasource`

Example: Helius variants live alongside the RPC examples — see [`yellowstone-grpc`](examples/yellowstone-grpc) (LaserStream), [`gpa-rpc`](examples/gpa-rpc) (gPA v2), and [`transaction-crawler-rpc`](examples/transaction-crawler-rpc) (GTFA).

### Geyser gRPC

Direct streaming from Geyser-enabled nodes.

- `carbon-yellowstone-grpc-datasource`
- `carbon-jito-shredstream-grpc-datasource`

Example: [`yellowstone-grpc`](examples/yellowstone-grpc)

### Historical / Archive

Snapshot-based or backfill sources:

- `carbon-validator-snapshot-datasource`
- `carbon-jetstreamer-datasource`

Example: [`snapshot-validator`](examples/snapshot-validator), [`jetstreamer`](examples/jetstreamer)

### Adapter

Bring your own stream:

- `carbon-stream-message-datasource`

Example: [`custom-datasource`](examples/custom-datasource)

---

### Custom Datasource

```rust
use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, Update, UpdateType},
        error::CarbonResult,
    },
    tokio_util::sync::CancellationToken,
};

pub struct MyDatasource;

#[async_trait]
impl Datasource for MyDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        // stream updates here
        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![
            UpdateType::AccountUpdate,
            UpdateType::Transaction,
        ]
    }
}
```

A full working example is available in
[`examples/custom-datasource`](examples/custom-datasource).

---

## Program Decoders

Carbon ships with pre-generated decoders for widely used Solana programs.

| Crate                                     | Program                  | Program ID                                     |
| ----------------------------------------- | ------------------------ | ---------------------------------------------- |
| `carbon-token-program-decoder`            | Token Program            | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`  |
| `carbon-token-2022-decoder`               | Token 2022               | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`  |
| `carbon-associated-token-account-decoder` | Associated Token Account | `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` |
| `carbon-system-program-decoder`           | System Program           | `11111111111111111111111111111111`             |
| `carbon-stake-program-decoder`            | Stake Program            | `Stake11111111111111111111111111111111111111`  |
| `carbon-address-lookup-table-decoder`     | Address Lookup Table     | `AddressLookupTab1e1111111111111111111111111`  |
| `carbon-jupiter-swap-decoder`             | Jupiter Swap             | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4`  |
| `carbon-raydium-amm-v4-decoder`           | Raydium AMM V4           | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| `carbon-raydium-clmm-decoder`             | Raydium CLMM             | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| `carbon-raydium-cpmm-decoder`             | Raydium CPMM             | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| `carbon-orca-whirlpool-decoder`           | Orca Whirlpool           | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc`  |
| `carbon-meteora-dlmm-decoder`             | Meteora DLMM             | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo`  |
| `carbon-pumpfun-decoder`                  | Pumpfun                  | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`  |
| `carbon-pump-swap-decoder`                | PumpSwap                 | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`  |
| `carbon-moonshot-decoder`                 | Moonshot                 | `MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG`  |
| `carbon-mpl-token-metadata-decoder`       | MPL Token Metadata       | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`  |
| `carbon-mpl-core-decoder`                 | MPL Core                 | `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d` |

<details>
<summary><strong>Show 46 more decoders</strong></summary>

| Crate                                          | Program                            | Program ID                                        |
| ---------------------------------------------- | ---------------------------------- | ------------------------------------------------- |
| `carbon-bonkswap-decoder`                      | Bonkswap                           | `BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p`    |
| `carbon-boop-decoder`                          | Boop                               | `boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4`     |
| `carbon-bubblegum-decoder`                     | Bubblegum                          | `BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY`    |
| `carbon-circle-message-transmitter-v2-decoder` | Circle CCTP Message Transmitter V2 | `CCTPV2Sm4AdWt5296sk4P66VBZ7bEhcARwFaaS9YPbeC`    |
| `carbon-circle-token-messenger-v2-decoder`     | Circle CCTP Token Messenger V2     | `CCTPV2vPZJS2u2BBsUoscuikbYjnpFmbFsvVuJdgUMQe`    |
| `carbon-dflow-aggregator-v4-decoder`           | Dflow Aggregator V4                | `DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH`    |
| `carbon-drift-v2-decoder`                      | Drift V2                           | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH`     |
| `carbon-fluxbeam-decoder`                      | Fluxbeam                           | `FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X`    |
| `carbon-gavel-decoder`                         | Gavel                              | `srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi`     |
| `carbon-heaven-decoder`                        | Heaven                             | `HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o`    |
| `carbon-jupiter-dca-decoder`                   | Jupiter DCA                        | `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M`    |
| `carbon-jupiter-lend-decoder`                  | Jupiter Lend                       | `jup3YeL8QhtSx1e253b2FDvsMNC87fDrgcZa9KfAB5Fkkkk` |
| `carbon-jupiter-limit-order-decoder`           | Jupiter Limit Order                | `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu`     |
| `carbon-jupiter-limit-order-2-decoder`         | Jupiter Limit Order 2              | `j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X`     |
| `carbon-jupiter-perpetuals-decoder`            | Jupiter Perpetuals                 | `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu`     |
| `carbon-kamino-farms-decoder`                  | Kamino Farms                       | `FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr`    |
| `carbon-kamino-lending-decoder`                | Kamino Lending                     | `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD`     |
| `carbon-kamino-limit-order-decoder`            | Kamino Limit Order                 | `LiMoM9rMhrdYrfzUCxQppvxCSG1FcrUK9G8uLq4A1GF`     |
| `carbon-kamino-vault-decoder`                  | Kamino Vault                       | `kvauTFR8qm1dhniz6pYuBZkuene3Hfrs1VQhVRgCNrr`     |
| `carbon-lifinity-amm-v2-decoder`               | Lifinity AMM V2                    | `2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c`    |
| `carbon-marginfi-v2-decoder`                   | Marginfi V2                        | `MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA`     |
| `carbon-marinade-finance-decoder`              | Marinade Finance                   | `MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD`     |
| `carbon-memo-program-decoder`                  | SPL Memo                           | `Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo`     |
| `carbon-meteora-damm-v2-decoder`               | Meteora DAMM V2                    | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG`     |
| `carbon-meteora-dbc-decoder`                   | Meteora DBC                        | `dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN`     |
| `carbon-meteora-pools-decoder`                 | Meteora Pools                      | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB`    |
| `carbon-meteora-vault-decoder`                 | Meteora Vault                      | `24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi`    |
| `carbon-name-service-decoder`                  | SPL Name Service                   | `namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX`     |
| `carbon-okx-dex-decoder`                       | OKX DEX                            | `6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma`    |
| `carbon-onchain-labs-dex-v2-decoder`           | OnChain Labs DEX V2                | `proVF4pMXVaYqmy4NjniPh4pqKNfMmsihgd4wdkCX3u`     |
| `carbon-openbook-v2-decoder`                   | Openbook V2                        | `opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb`     |
| `carbon-pancake-swap-decoder`                  | Pancake Swap                       | `HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq`    |
| `carbon-phoenix-v1-decoder`                    | Phoenix V1                         | `PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY`     |
| `carbon-pump-fees-decoder`                     | Pump Fees                          | `pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ`     |
| `carbon-raydium-launchpad-decoder`             | Raydium Launchpad                  | `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj`     |
| `carbon-raydium-liquidity-locking-decoder`     | Raydium Liquidity Locking          | `LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE`     |
| `carbon-raydium-stable-swap-decoder`           | Raydium Stable Swap                | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h`    |
| `carbon-sharky-decoder`                        | SharkyFi                           | `SHARKobtfF1bHhxD2eqftjHBdVSCbKo9JtgK71FhELP`     |
| `carbon-solayer-restaking-program-decoder`     | Solayer Restaking                  | `sSo1iU21jBrU9VaJ8PJib1MtorefUV4fzC9GURa2KNn`     |
| `carbon-stabble-stable-swap-decoder`           | Stabble Stable Swap                | `swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ`     |
| `carbon-stabble-weighted-swap-decoder`         | Stabble Weighted Swap              | `swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW`     |
| `carbon-swig-decoder`                          | Swig                               | `swigypWHEksbC64pWKwah1WTeh9JXwx8H1rJHLdbQMB`     |
| `carbon-vertigo-decoder`                       | Vertigo                            | `vrTGoBuy5rYSxAfV3jaRJWHH6nN9WK4NRExGxsk1bCJ`     |
| `carbon-virtuals-decoder`                      | Virtuals                           | `5U3EU2ubXtK84QcRjWVmYt9RaDyA8gKxdUrPFXmZyaki`    |
| `carbon-wavebreak-decoder`                     | Wavebreak                          | `waveQX2yP3H1pVU8djGvEHmYg8uamQ84AuyGtpsrXTF`     |
| `carbon-zeta-decoder`                          | Zeta                               | `ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD`     |

</details>

---

## License

We are under the [MIT license](https://github.com/sevenlabs-hq/carbon/tree/main/LICENSE.md).
