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

Carbon provides a CLI tool to generate decoders based on IDL files. This can significantly speed up the process of creating custom decoders for your Solana programs.

#### CLI Usage

```sh
$ carbon-cli parse [OPTIONS] --idl <IDL> --output <OUTPUT>
```

#### Options

- `-i, --idl <IDL>`: Path to the IDL json file.
- `-o, --output <OUTPUT>`: Path to the desired output directory.
- `-C, --as-crate`: Generate a directory or a crate.
- `-h, --help`: Print help information.

#### Example

To generate a decoder from an IDL file:

```sh
$ carbon-cli parse --idl my_program.json --output ./src/decoders
```

This will parse the my_program.json IDL file and generate the corresponding decoder code in the ./src/decoders directory.

### Implementing Processors

```rs
use carbon_core::account::{AccountDecoder, AccountMetadata, DecodedAccount};
use crate::MyCustomAccountData;

struct MyAccountProcessor;

#[async_trait]
impl Processor for MyAccountProcessor {
    type InputType = (AccountMetadata, DecodedAccount<MyCustomAccountData>);

    async fn process(
        &mut self,
        input: Self::InputType,
        metrics: Vec<Arc<dyn Metrics>>,
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

## License

We are under the [MIT license](https://github.com/sevenlabs-hq/carbon/tree/main/LICENSE.md).
