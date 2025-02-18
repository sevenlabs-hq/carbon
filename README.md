# Carbon

Carbon is a lightweight indexing framework on Solana. It provides a modular pipeline for sourcing data, decoding updates and processing them in order to build end-to-end indexers.

## Components

### Pipeline

The core of the framework. It orchestrates data flow from data sources through indexing pipes.

### Datasources

A consumable datasource that will provide updates to the pipeline. These can either be `AccountUpdate`, `TransactionUpdate` or `AccountDeletion`.

### Pipes

Process specific updates:

- **Account Pipes** handle account updates. Each contains an `AccountDecoder` and a `Processor`.
- **Account Deletion Pipes** handle account deletions. Each contains a `Processor`.
- **Instruction Pipes** handle transaction updates, instruction by instruction. Each contains an `InstructionDecoder` and a `Processor`.
- **Transaction Pipes** handle transaction updates, after schema-matching the whole transaction. Each contains a `Schema` and a `Processor`.

### Metrics

Collect and report on pipeline performance and operational data.

Our premade metrics crates assist with common use cases:
| Crate Name | Description | Ease of Setup |
|------------|-------------|---------------|
| `carbon-log-metrics` | Logs useful program info to the terminal | Easy |
| `carbon-prometheus-metrics` | Provides a way of exporting default and custom metrics to a Prometheus server | Medium |

## Usage

### Basic Setup

```rs
use carbon_core::pipeline::Pipeline;
use carbon_rpc_block_subscribe_datasource::{RpcBlockSubscribe, Filters};
use solana_client::{
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
};
use crate::{
    MyAccountDecoder, MyAccountProcessor,
    MyInstructionDecoder, MyInstructionProcessor,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = Pipeline::builder()
        .datasource(
            RpcBlockSubscribe::new(
                env::var("RPC_URL")?,
                Filters::new(RpcBlockSubscribeFilter::MentionsAccountOrProgram(env::var("MY_PROGRAM_ID")?), None)
            )
        )
        .instruction(MyInstructionDecoder::new(), MyInstructionProcessor)
        .metrics(Arc::new(LogMetrics::new()))
        .build()?;

    pipeline.run().await?;

    Ok(())
}
```

### Generating Decoders from IDL

Decoders implementations allow the pipeline to input raw account or instruction data and to receive deserialized account or instruction data. They are the backbone of indexing with Carbon.

Carbon provides a CLI tool to generate decoders based on IDL files (Anchor, Codama) or from a provided program address with a network specified to fetch an on-chain PDA IDL. This can significantly speed up the process of creating custom decoders for your Solana programs.

#### CLI Installation

You can install the Carbon CLI by downloading the pre-built binary for your operating system:

##### Linux

```sh
curl -LO https://github.com/sevenlabs-hq/carbon/releases/latest/download/carbon-cli-linux-amd64
chmod +x carbon-cli-linux-amd64
sudo mv carbon-cli-linux-amd64 /usr/local/bin/carbon-cli
```

##### macOS

```sh
curl -LO https://github.com/sevenlabs-hq/carbon/releases/latest/download/carbon-cli-macos-amd64
chmod +x carbon-cli-macos-amd64
sudo mv carbon-cli-macos-amd64 /usr/local/bin/carbon-cli
```

##### Windows

1. Download the latest release from https://github.com/sevenlabs-hq/carbon/releases/latest/download/carbon-cli-windows-amd64.exe
2. Rename the downloaded file to `carbon-cli.exe`
3. Move the file to a directory in your PATH

Alternatively, you can build from source using Cargo:

```sh
cargo install --git https://github.com/sevenlabs-hq/carbon.git carbon-cli
```

#### CLI Usage

```sh
$ carbon-cli parse [OPTIONS] --idl <IDL> --output <OUTPUT>
```

#### Options

- `-i, --idl <IDL>`: Path to an IDL json file or a Solana program address.
- `-o, --output <OUTPUT>`: Path to the desired output directory.
- `-C, --as-crate`: Generate a directory or a crate.
- `--codama`: The IDL json file to parse is a Codama IDL.
- `-e, --event-hints`: Comma-separated names of defined types to parse as CPI Events (for '--codama' option only).
- `-u, --url`: Network URL to fetch the IDL from. Required if input is a program address.
- `-h, --help`: Print help information.

#### Examples

- To generate a decoder from an IDL file:

```sh
$ carbon-cli parse --idl my_program.json --output ./src/decoders
```

This will parse the my_program.json IDL file and generate the corresponding decoder code in the ./src/decoders directory.

- To generate a decoder from an Anchor PDA IDL, specify a program address (Meteora DLMM program in this case):

```sh
$ carbon-cli parse --idl LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo -u mainnet-beta --output ./src/decoders
```

This will fetch Meteora DLMM program's IDL from chain and generate the corresponding decoder code in the ./src/decoders directory.

- To generate a decoder from a Codama IDL:

```sh
$ carbon-cli parse --idl my_program_codama.json --output ./src/decoders --codama
```

This will parse the my_program_codama.json Codama IDL file and generate the corresponding decoder code in the ./src/decoders directory.

**Note**: in order to parse CPI Events for a provided Codama IDL, add `--event-hints` option with comma-separated names of corresponding defined Codama types:

```sh
$ carbon-cli parse --idl my_program_codama.json --output ./src/decoders --codama --event-hints event1,event2,event3
```

### Implementing Processors

```rs
use carbon_core::account::{AccountDecoder, AccountMetadata, AccountProcessorInputType, DecodedAccount};
use crate::MyCustomAccountData;

struct MyAccountProcessor;

#[async_trait]
impl Processor for MyAccountProcessor {
    type InputType = AccountProcessorInputType<MyCustomAccountData>;

    async fn process(
        &mut self,
        input: Self::InputType,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        // Implement processing logic
    }
}
```

### Implementing a Datasource

For most use cases, we recommend chosing from one of our datasource crates:
| Crate Name | Description | Affordability | Ease of Setup |
|------------|-------------|---------------|----------------|
| `carbon-block-subscribe` | Uses `blockSubscribe` with Solana WS JSON RPC to listen to real-time on-chain transactions | Cheap (just RPC) | Easy |
| `carbon-program-subscribe` | Uses `programSubscribe` with Solana WS JSON RPC to listen to real-time on-chain account updates | Cheap (just RPC) | Easy |
| `carbon-transaction-crawler` | Crawls historical successful transactions for a specific address in reverse chronological order using Solana JSON RPC | Cheap (just RPC) | Easy |
| `carbon-helius-atlas-ws` | Utilizes Helius Geyser-enhanced WebSocket for streaming account and transaction updates | Medium (Helius Plan) | Medium |
| `carbon-yellowstone-grpc` | Subscribes to a Yellowstone gRPC Geyser plugin enhanced full node to stream account and transaction updates | Expensive (Geyser Fullnode) | Complex |

You can still implement custom datasources in the following manner:

```rs
use carbon_core::datasource::{Datasource, Update, UpdateType};

struct MyDataSource;

#[async_trait]
impl Datasource for MyDataSource {
    async fn consume(
        &self,
        sender: &tokio::sync::mpsc::UnboundedSender<Update>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        // Implement data fetching and sending logic
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate, UpdateType::Transaction]
    }
}
```

### Available Program Decoders

Decoders for most popular Solana programs are published and maintained:
| Crate Name | Description | Program ID |
|------------|-------------|------------|
| `carbon-jupiter-dca-decoder` | Jupiter DCA Program Decoder | DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M |
| `carbon-jupiter-limit-order-decoder` | Jupiter Limit Order Program Decoder | jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu |
| `carbon-jupiter-limit-order-2-decoder` | Jupiter Limit Order 2 Program Decoder | j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X |
| `carbon-jupiter-swap-decoder` | Jupiter Swap Program Decoder | JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 |
| `carbon-meteora-dlmm-decoder` | Meteora DLMM Program Decoder | LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo |
| `carbon-mpl-core-decoder` | MPL Core Program Decoder | CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d |
| `carbon-mpl-token-metadata-decoder` | MPL Token Metadata Program Decoder | metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s |
| `carbon-orca-whirlpool-decoder` | Orca Whirlpool Program Decoder | whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc |
| `carbon-pumpfun-decoder` | Pumpfun Program Decoder | 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P |
| `carbon-raydium-amm-v4-decoder` | Raydium AMM V4 Program Decoder | 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 |
| `carbon-system-program-decoder` | System Program Decoder | 11111111111111111111111111111111 |
| `carbon-token-program-decoder` | Token Program Decoder | TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA |
| `carbon-kamino-lend-decoder` | Kamino Lend Decoder | KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD |
| `carbon-kamino-vault-decoder` | Kamino Vault Decoder | kvauTFR8qm1dhniz6pYuBZkuene3Hfrs1VQhVRgCNrr |

## License

We are under the [MIT license](https://github.com/sevenlabs-hq/carbon/tree/main/LICENSE.md).
