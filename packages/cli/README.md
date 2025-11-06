# @sevenlabs-hq/carbon-cli

[![npm version](https://img.shields.io/npm/v/@sevenlabs-hq/carbon-cli.svg)](https://www.npmjs.com/package/@sevenlabs-hq/carbon-cli)
[![npm downloads](https://img.shields.io/npm/dm/@sevenlabs-hq/carbon-cli.svg)](https://www.npmjs.com/package/@sevenlabs-hq/carbon-cli)

Generate decoders and scaffold indexers for Solana programs from Anchor or Codama IDL files.

## Installation

```sh
# Install globally
npm install -g @sevenlabs-hq/carbon-cli

# Or use npx (no installation required)
npx @sevenlabs-hq/carbon-cli
```

## Usage

The Carbon CLI provides two main commands: `parse` and `scaffold`.

### Parse Command

Generate decoder code from IDL files or program addresses.

```sh
carbon-cli parse [OPTIONS]
```

#### Options

- `-i, --idl <fileOrAddress>` - Path to an IDL json file or a Solana program address
- `-o, --out-dir <dir>` - Output directory for generated code
- `-c, --as-crate` - Generate as a Cargo crate layout (default: false)
- `-s, --standard <anchor|codama>` - Specify the IDL standard to parse (default: anchor)
- `--event-hints <csv>` - Comma-separated names of defined types to parse as CPI Events (Codama only)
- `-u, --url <rpcUrl>` - RPC URL for fetching IDL when using a program address
- `--no-clean` - Do not delete output directory before rendering (default: false)

#### Examples

**Parse Anchor IDL file:**

```sh
carbon-cli parse --idl my_program.json --out-dir ./src/decoders
```

**Parse from program address:**

```sh
carbon-cli parse --idl LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo --url mainnet-beta --out-dir ./src/decoders
```

**Parse Codama IDL with event hints:**

```sh
carbon-cli parse --idl my_program_codama.json --out-dir ./src/decoders --standard codama --event-hints "BuyEvent,CreatePoolEvent"
```

### Scaffold Command

Generate a complete project skeleton with decoder and indexer setup.

```sh
carbon-cli scaffold [OPTIONS]
```

#### Options

- `-n, --name <string>` - Name of your project
- `-o, --out-dir <dir>` - Output directory
- `-d, --decoder <name>` - Decoder name (auto-detected from IDL)
- `--idl <fileOrAddress>` - IDL file or program address
- `--idl-standard <anchor|codama>` - IDL standard
- `--idl-url <rpcUrl>` - RPC URL for fetching IDL (when using program address)
- `--event-hints <csv>` - Event hints for Codama IDL
- `-s, --data-source <name>` - Name of data source
- `-m, --metrics <log|prometheus>` - Metrics to use (default: log)
- `--with-postgres <boolean>` - Include Postgres wiring and deps (default: true)
- `--with-graphql <boolean>` - Include GraphQL wiring and deps (default: true)
- `--with-serde <boolean>` - Include serde feature for decoder (default: false)
- `--force` - Overwrite output directory if it exists (default: false)

#### Examples

**Scaffold with generated decoder from IDL:**

```sh
carbon-cli scaffold --name my-project --out-dir . --idl ./idl.json --idl-standard anchor --data-source rpc-block-subscribe
```

**Scaffold with Helius Laserstream datasource:**

```sh
carbon-cli scaffold --name my-project --out-dir . --idl ./idl.json --idl-standard anchor --data-source helius-laserstream
```

## Interactive Mode

Both commands support interactive mode when run without required options:

```sh
# Interactive parse
carbon-cli parse

# Interactive scaffold
carbon-cli scaffold
```

## Available Data Sources

- `rpc-block-subscribe` - Uses Solana WS JSON RPC blockSubscribe
- `rpc-program-subscribe` - Uses Solana WS JSON RPC programSubscribe
- `rpc-transaction-crawler` - Crawls historical transactions
- `helius-laserstream` - Helius Laserstream datasource
- `yellowstone-grpc` - Yellowstone gRPC datasource
- `jito-shredstream-grpc` - JITO Shredstream gRPC

## Related Packages

- [`@sevenlabs-hq/carbon-codama-renderer`](https://www.npmjs.com/package/@sevenlabs-hq/carbon-codama-renderer) - Codama renderer for generating Carbon-compatible Rust decoder code

## Links

- [NPM Package](https://www.npmjs.com/package/@sevenlabs-hq/carbon-cli)
- [GitHub Repository](https://github.com/sevenlabs-hq/carbon)
- [Carbon Framework Documentation](https://github.com/sevenlabs-hq/carbon#readme)

## License

MIT
