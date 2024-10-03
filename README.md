# Carbon

Carbon is an advanced indexing framework designed for developers building on Solana. It streamlines the process of working with indexers, offering a smooth experience while maintaining the flexibility and complexity required for advanced use cases.

With Carbon, you can track multiple data sources, allowing you to choose how to handle incoming data. Some of the supported data sources include:

- Yellowstone gRPC
- Transaction Crawler
- Google BigTable
- Helius Atlas WS

Once you've selected a data source, Carbon allows you to:

- Execute custom business logic when specific accounts or instructions are included in a transaction.
- Define transaction schemas — combinations of instructions that help you recognize and interpret transactions that meet particular criteria, with which you are able to execute custom business logic.

Carbon is packed with tools that make working with Solana indexers both powerful and easy to use. Whether you're working with accounts, instructions, or entire transaction flows, Carbon provides the flexibility to design your logic while handling the complexity behind the scenes.

Let’s start by lookig at an example of Carbon usage.

# The Carbon Pipeline

The Carbon framework is designed to be easy to use. To facilitate this, we’ve created a pipeline system where you can chain components together to suit your specific use case. By defining expectations for our indexer, you can easily set up the necessary components to fulfill your application's needs.

## Example Pipeline

Here’s an example of what a typical pipeline looks like:

```rust
carbon_core::pipeline::Pipeline::builder()
        .datasource(Datasource)
        .account(ProgramDecoder, ProgramAccountProcessor)
        .instruction(ProgramDecoder, ProgramInstructionProcessor)
        .transaction(TRANSACTION_SCHEMA.clone(), ProgramTransactionProcessor)
        .build()?
        .run()
        .await?;
```

In this example, you can observe how we chain multiple pipes, each handling a specific aspect of the data processing workflow. What’s important to note is that you can use multiple instances of the same type of pipe with different decoders and processors, allowing for flexible configurations.

The pipeline consists of four key parts, which we will break down below.

---

## 1. Datasource

The `Datasource` is any struct that implements our `Datasource` trait. This trait requires two functions to be implemented:

- `consume`
- `update_types`

These functions define how data is processed and updated in the pipeline.

As mentioned above, Carbon has several premade decoders:

- Yellowstone gRPC
- Transaction Crawler
- Google BigTable
- Helius Atlas WS

Feel free to inspect and tweak each of these for clarity and usage.

---

## 2. Account Pipe

The `Account` pipe takes two parameters: `ProgramDecoder` and `AccountProcessor`.

- The `ProgramDecoder` can be generated using the Carbon CLI if the program has an Anchor IDL, or it can be custom made by the developer.
- The `AccountProcessor` is a custom implementation that contains the necessary business logic.

The `ProgramDecoder` implements the `AccountDecoder` trait, allowing it to decode accounts as they are processed in the pipeline. For reference, you can consult our pre-built decoders to understand how this works in practice.

TODO: Example of AccountProcessor?

---

## 3. Instruction Pipe

The `Instruction` pipe also takes two parameters: `ProgramDecoder` and `InstructionProcessor`.

- The `ProgramDecoder`, as with the `Account` pipe, can be generated using the Carbon CLI for programs with an Anchor IDL, or it can be created manually.
- The `InstructionProcessor` should be custom-made, as it contains the business logic for processing instructions.

The `ProgramDecoder` implements the `InstructionDecoder` trait, which enables it to decode instructions. Again, you can refer to our pre-built decoders for guidance on how to implement this in your project.

TODO: Example of InstructionProcessor?

---

## 4. Transaction Pipe

The `Transaction` pipe has two parameters: the `TRANSACTION_SCHEMA` and the `TransactionProcessor`.

### Transaction Schema

The `TRANSACTION_SCHEMA` defines the structure of the transactions that the pipeline should look for. It acts as a pattern for recognizing transactions of interest, and it consists of ordered instructions that you expect the transaction to include.

Here are a few examples of transaction schemas:

### Basic Example

```rust
static JUPITER_SCHEMA: Lazy<TransactionSchema<AllInstructions>> = Lazy::new(|| {
    schema![
        any
        [
            AllInstructionTypes::JupSwap(JupiterInstructionType::SharedAccountsRoute),
            "shared_accounts_route_ix_1",
            []
        ]
        any
    ]
});
```

In this example, the indexer is instructed to look for any transaction containing a `SharedAccountsRoute` instruction from the Jupiter program. The placement of `any` before and after the instruction signifies that this instruction can occur at any position within the transaction.

### Complex Example

```rust
static MORE_COMPLEX_SCHEMA: Lazy<TransactionSchema<AllInstructions>> = Lazy::new(|| {
    schema![
        [
            AllInstructionTypes::JupSwap(JupiterInstructionType::SharedAccountsRoute),
            "shared_accounts_route_ix_1",
            [
                any
                [
                    AllInstructionTypes::JupSwap(JupiterInstructionType::SwapEvent),
                    "swap_event_ix_1",
                    []
                ]
            ]
        ]
        any
    ]
});
```

In this more complex example, we’re specifying that the transaction must contain a `shared_accounts_route` instruction, but, by omitting `any` from the beginning, we show that we want it as the first instruction within the transaction. Additionally, one of the inner instructions of it must be a `swap_event` instruction, and it must be the last inner instruction within `shared_accounts_route`. When making a schema, you should list all the instructions that you are interested in, as well as those that help you identify the transaction as useful.

This schema showcases the recursive complexity and flexibility of Carbon’s transaction schemas.

---

Transaction Processor

Once a schema is matched, the `TransactionProcessor` is applied. The processor executes the custom business logic based on the transaction data.

Here’s an example:

```rust
#[derive(Clone, Debug, Deserialize)]
pub struct JupiterOutput {
    pub shared_accounts_route_ix_1: DecodedInstruction<JupiterInstruction>,
    pub swap_event_ix_1: DecodedInstruction<JupiterInstruction>,
}

pub struct JupiterTransactionProcessor;

#[async_trait]
impl Processor for JupiterTransactionProcessor {
    type InputType = JupiterOutput;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()> {
        // Business logic...
        Ok(())
    }
}
```

In this example, the `JupiterOutput` struct matches the names of instructions defined in the schema, allowing the data to be automatically parsed and passed to the business logic for processing.

## Decoder Collection

In the previous examples, we've used types that are essential for ensuring the smooth operation of the Carbon framework. The `instruction_decoder_collection!` macro simplifies the process of setting up everything needed to work with transaction schemas and pipelines.

By utilizing these types within your schemas and pipelines, you can create a seamless development experience with Carbon. This design ensures that your indexing process remains efficient and organized, allowing you to focus on implementing business logic rather than worrying about the structure of instructions and programs.

### Example Usage

Here’s an example of how this macro is used:

```rust
instruction_decoder_collection!(
    AllInstructions, AllInstructionTypes, AllPrograms,
    MeteoraSwap => MeteoraInstructionDecoder => MeteoraInstruction,
    OrcaSwap => OrcaInstructionDecoder => OrcaInstruction,
    JupSwap => JupiterDecoder => JupiterInstruction,
    RaydiumClmm => AmmV3Decoder => AmmV3Instruction,
);
```

### Breakdown

Each row after the first represents a program you’re interested in indexing. Let’s break down the columns:

- **First Column**: Represents the name of the program you are indexing (you can name it however you like).
- **Second Column**: Contains the Decoder struct for the program, which should be generated or manually created.
- **Third Column**: Contains the enum of all instructions for the program, representing each instruction with its associated data, which will also be generated or manually created.

#### First Row Definition

The first row defines three enum types, and while you can name them as you wish, we recommend the following for clarity:

- `AllInstructions`
- `AllInstructionTypes`
- `AllPrograms`

These enums will serve different purposes in indexing and decoding.

### Enum Definitions

#### AllInstructions

`AllInstructions` is an enum that aggregates all the programs you want to index, with each enum variant corresponding to a program's instruction enum. For example:

```rust
pub enum AllInstructions {
    JupSwap(JupiterInstruction::SwapEvent(/* data, accounts ... */)),
}
```

This enum contains the program-specific instruction enums with their associated data and accounts.

#### AllInstructionTypes

`AllInstructionTypes` is structured similarly to `AllInstructions`, but it excludes instruction data and accounts from the enum variants. This is useful for schema matching:

```rust
pub enum AllInstructionTypes {
    JupSwap(JupiterInstructionType::SwapEvent),
}
```

This is the enum you'll use when defining transaction schemas.

#### AllPrograms

The third enum simply enumerates the different programs you want to index, allowing you to manage them collectively.

# Carbon CLI

Carbon CLI is a command-line tool designed to streamline development within the Carbon framework. It simplifies code generation, making it easier for developers to work with Anchor IDLs and generate the necessary Rust code for their programs. By integrating seamlessly with the Carbon framework, the CLI helps maintain a productive development workflow.

## Commands

### `parse`

#### Overview

The `parse` command automates the generation of a Rust crate based on a provided Anchor IDL. The resulting crate includes many of the necessary types and structures to be used within the Carbon framework. While the new IDL format (v30.1+) is supported, we recommend using the legacy format (before v30.1) for maximum compatibility, especially when working with accounts.

#### Usage

The `parse` command accepts two parameters, with one being mandatory:

- `--idl`: Specifies the path to the IDL file (required).
- `--output`: Specifies the path where the generated Rust crate will be placed (optional).

If the `--output` parameter is omitted, you should be located in `crates/cli` when running the CLI, then the decoder will be generated in the default directory. After the crate is generated, you can inspect its contents and import it into your program as needed.

**Example:**

```bash
cargo run parse --idl path/to/idl.json --output path/to/output/crate
```

For more details on the generated output, see the [Decoders](#decoders) section.

## Decoders

The `Decoders` directory contains pre-built decoder examples that can be used within the Carbon framework. These decoders are implemented as individual crates, each structured with three key directories: `instructions`, `accounts`, and `types`. These directories contain the logic needed to decode instructions, accounts, and types in real-time as they are processed.

Each decoder crate should implement two essential traits:

- `InstructionDecoder`
- `AccountDecoder`

These traits allow the decoder to translate incoming instructions into meaningful data, making them useful in the program’s logic.

### Enums and Instruction Decoding

The decoder crate also includes enums representing all instruction variants from the program, including events. These enums are highly useful for pattern matching and data extraction during instruction processing.

### Pre-built Decoders

We provide several pre-built decoders for widely used Solana programs, including:

- **System Program**
- **Token Program**
  TODO: Write all published on cargo

These examples can serve as a reference for developing custom decoders for non-Anchor programs. Refer to the examples when building your own custom decoders for other Solana programs.
