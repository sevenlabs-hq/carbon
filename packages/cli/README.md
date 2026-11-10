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
- `-n, --name <string>` - Decoder name (e.g. `pumpfun`), used for the crate name `carbon-<name>-decoder` and the struct `<Name>Decoder` (default: from the IDL)
- `-s, --standard <anchor|codama>` - Specify the IDL standard to parse (default: anchor)
- `--event-cpi-discriminator <hex>` - Event-CPI envelope for hidden-prefix Codama events, as hex bytes (default: the Anchor event tag `e445a52e51cb9a1d`)
- `-u, --url <rpcUrl>` - RPC URL for fetching IDL when using a program address
- `--program-id <address>` - Program ID (used if the IDL has no program address)
- `--postgres-mode <generic|typed>` - Postgres table storage mode (default: typed)
- `--with-postgres <boolean>` - Include Postgres wiring and deps (default: true)
- `--with-graphql <boolean>` - Include GraphQL wiring and deps (default: true)
- `--with-serde <boolean>` - Include serde feature for decoder (default: true; `--with-postgres`, `--with-graphql` and `--with-base58` also turn it on)
- `--with-base58 <boolean>` - Include base58 feature for decoder (default: false)
- `--standalone <boolean>` - Generate a standalone decoder with a `[workspace]` section (default: true); set to false for workspace dependencies
- `--description <string>` - Package description (default: derived from the decoder name)
- `--keywords <csv>` - Package keywords, comma-separated (default: solana, decoder, plus name parts)
- `--categories <csv>` - Package categories, comma-separated (default: encoding)
- `--package-version <string>` - Package version in Cargo.toml (default: 0.1.0)
- `--version-name <name>` - Decoder version name (e.g. `v1`, `v2`)
- `--no-clean` - Do not delete output directory before rendering (default: false)
- `-h, --help` - Display help for the command

The decoder is always generated as a Cargo crate. `-c, --as-crate` has no
effect; it is still accepted, with a warning, and will be removed.

#### Examples

**Parse Anchor IDL file:**

```sh
carbon-cli parse --idl my_program.json --out-dir ./src/decoders
```

**Parse from program address:**

```sh
carbon-cli parse --idl LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo --url mainnet-beta --out-dir ./src/decoders
```

**Parse Codama IDL:**

```sh
carbon-cli parse --idl my_program_codama.json --out-dir ./src/decoders --standard codama
```

When the IDL contains native Codama `eventNode` values describing event-CPI
payloads, typed event data and the `CpiEvent` instruction variant are generated
automatically. `--event-hints` has no effect; it is still accepted, with a
warning, and will be removed.

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
- `--event-cpi-discriminator <hex>` - Event-CPI envelope for hidden-prefix Codama events, as hex bytes (default: the Anchor event tag `e445a52e51cb9a1d`)
- `--program-id <address>` - Program ID (used if the IDL has no program address)
- `-s, --data-source <name>` - Name of data source
- `-m, --metrics <log|prometheus>` - Metrics to use (default: log)
- `--with-base58 <boolean>` - Include base58 feature for decoder (default: false)
- `--with-postgres <boolean>` - Include Postgres wiring and deps (default: true)
- `--postgres-mode <generic|typed>` - Postgres table storage mode (default: typed)
- `--with-graphql <boolean>` - Include GraphQL wiring and deps (default: true)
- `--with-serde <boolean>` - Include serde feature for decoder (default: true; `--with-postgres`, `--with-graphql` and `--with-base58` also turn it on)
- `--force` - Overwrite output directory if it exists (default: false)
- `--package-version <string>` - Package version in Cargo.toml (default: 0.1.0)
- `--version-name <name>` - Decoder version name (e.g. `v1`, `v2`)
- `-h, --help` - Display help for the command

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

## Related Packages

- [`@sevenlabs-hq/carbon-codama-renderer`](https://www.npmjs.com/package/@sevenlabs-hq/carbon-codama-renderer) - Codama renderer for generating Carbon-compatible Rust decoder code

## Links

- [NPM Package](https://www.npmjs.com/package/@sevenlabs-hq/carbon-cli)
- [GitHub Repository](https://github.com/sevenlabs-hq/carbon)
- [Carbon Framework Documentation](https://github.com/sevenlabs-hq/carbon#readme)

## License

MIT
