# Contributing to Carbon

Thank you for your interest in contributing to Carbon! This document provides guidelines and information for contributors to help make the contribution process smooth and effective.

## Table of Contents

- [About Carbon](#about-carbon)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Code Style and Standards](#code-style-and-standards)
- [Continuous Integration](#continuous-integration)
- [Testing](#testing)
- [Adding New Features](#adding-new-features)
- [Adding New Decoders](#adding-new-decoders)
- [Adding New Datasources](#adding-new-datasources)
- [Submitting Changes](#submitting-changes)
- [Getting Help](#getting-help)

## About Carbon

Carbon is an indexing framework for Solana that provides a modular pipeline for sourcing data, decoding updates, and processing them to build end-to-end indexers. The project is organized as a Rust workspace with multiple crates covering different aspects of the indexing ecosystem.

## Development Setup

### Prerequisites

- **Rust**: 1.88.0 (pinned via [`rust-toolchain.toml`](rust-toolchain.toml)). Published crates target **MSRV 1.82** (set in [`clippy.toml`](clippy.toml)) — develop on 1.88, but don't reach for features newer than 1.82 in code that ships.
- **Git**: For version control.
- **Cargo**: Rust's package manager (included with Rust).
- **System libs**: Some datasources need `libclang` (for `bindgen`) — `sudo apt-get install libclang-dev` on Debian/Ubuntu, `brew install llvm` on macOS.

### Getting Started

1. **Clone the repository**:

    ```bash
    git clone https://github.com/sevenlabs-hq/carbon.git
    cd carbon
    ```

2. **Install dependencies**:

    ```bash
    cargo build
    ```

3. **Run tests**:
    ```bash
    cargo test
    ```

### Pre-commit Hooks

To activate the pre-commit hooks, run:

```bash
./.pre-commit.sh
```

On every `git commit` it runs:

- `cargo fmt --check` — formatting
- `cargo clippy --all-targets --all-features -- -D warnings` — lint, deny warnings
- `cargo sort -c -g` — `Cargo.toml` ordering
- `cargo machete` — unused dependencies
- `cargo test --all-targets --all-features` — full test suite

The same five checks run in CI (see [Continuous Integration](#continuous-integration)).

## Project Structure

The Carbon project is organized as a Rust workspace with the following main components:

### Core Crates (`crates/`)

- **`carbon-core`**: The framework — pipeline orchestration, traits, filters, metrics.
- **`carbon-macros`**: `no_std` declarative helper macros (e.g. `try_decode_instructions!`).
- **`carbon-proc-macros`**: Procedural macros — `instruction_decoder_collection!` and `InstructionType` derive.
- **`carbon-test-utils`**: JSON-fixture helpers used by decoder tests.

The TypeScript CLI (`@sevenlabs-hq/carbon-cli`) and the codama renderer live under [`packages/`](packages/), not `crates/`.

### Datasources (`datasources/`)

Update producers grouped by source — each crate ships its own README with setup details and tradeoffs:

- **Solana RPC**: `carbon-rpc-block-subscribe-datasource`, `carbon-rpc-program-subscribe-datasource`, `carbon-rpc-transaction-crawler-datasource`, `carbon-rpc-block-crawler-datasource`, `carbon-rpc-gpa-datasource`
- **Helius**: `carbon-helius-atlas-ws-datasource`, `carbon-helius-laserstream-datasource`, `carbon-helius-gpa-v2-datasource`, `carbon-helius-gtfa-datasource`
- **Geyser gRPC**: `carbon-yellowstone-grpc-datasource`, `carbon-jito-shredstream-grpc-datasource`
- **Historical / archive**: `carbon-validator-snapshot-datasource`, `carbon-jetstreamer-datasource`
- **Adapter**: `carbon-stream-message-datasource`

### Decoders (`decoders/`)

Program-specific decoders for popular Solana programs:

- **`carbon-token-program-decoder`**: SPL Token program
- **`carbon-jupiter-swap-decoder`**: Jupiter swap program
- **`carbon-raydium-amm-v4-decoder`**: Raydium AMM v4
- **`carbon-kamino-lending-decoder`**: Kamino lending
- And many more...

### Metrics (`metrics/`)

- **`carbon-log-metrics`**: Log-based metrics collection
- **`carbon-prometheus-metrics`**: Prometheus metrics export

### Examples (`examples/`)

Standalone, runnable indexers — one crate per indexing pattern:

- **`yellowstone-grpc`**: Real-time pipeline (Yellowstone gRPC, with LaserStream and Jito Shredstream variants)
- **`block-subscribe-rpc`**: Real-time pipeline over public Solana RPC
- **`gpa-rpc`**: Loading current program state via `getProgramAccounts`
- **`transaction-crawler-rpc`**: Per-program transaction history
- **`snapshot-validator`**: Loading state from a validator snapshot file
- **`jetstreamer`**: Bounded-range historical backfill from an archive
- **`versioned-decoders`**: Routing across program upgrades with breaking IDL changes
- **`postgres-graphql`**: Persisting decoded data to Postgres with a GraphQL query layer
- **`custom-datasource`**: Reference for implementing your own `Datasource`

See [`examples/README.md`](examples/README.md) for the use-case index.

## Code Style and Standards

### Rust Standards

Carbon follows standard Rust conventions and best practices:

1. **Formatting**: Use `cargo fmt` to format code
2. **Linting**: Use `cargo clippy` for linting
3. **Documentation**: Document all public APIs with doc comments

### Code Quality Checks

Run the following commands to ensure code quality:

```bash
# Format code
./scripts/cargo-fmt.sh

# Run clippy with strict settings
./scripts/cargo-clippy.sh

# Run tests
cargo test

# Check for unused dependencies
cargo machete
```

### Clippy Configuration

The project uses a strict clippy configuration defined in `clippy.toml`:

- Minimum Rust version: 1.82
- Maximum stack size for large types: 128 bytes
- Denies warnings, default trait access, arithmetic side effects, manual let-else, and used underscore binding

## Continuous Integration

Every PR runs the workflow in [`.github/workflows/check.yml`](.github/workflows/check.yml):

| Check    | Command                                                      |
| -------- | ------------------------------------------------------------ |
| Format   | `cargo fmt --check`                                          |
| Lint     | `cargo clippy --all-targets --all-features -- -D warnings`   |
| Sort     | `cargo sort -c -g`                                           |
| Unused   | `cargo machete`                                              |
| Tests    | `cargo test --all-targets --all-features`                    |

All five must pass before a PR is mergeable. Run the same locally via `./.pre-commit.sh` plus `cargo test --workspace`.

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p carbon-core

# Run tests with output
cargo test -- --nocapture
```

### Test Structure

- **Unit tests**: Located in `src/` files with `#[cfg(test)]` modules
- **Integration tests**: Located in `tests/` directories
- **Examples**: Working examples in the `examples/` directory serve as integration tests

### Test Utilities

The `carbon-test-utils` crate provides common testing utilities and fixtures for:

- Mock datasources
- Test data generation
- Common test setup patterns

## Adding New Features

### Feature Development Process

1. **Create a feature branch**

2. **Implement the feature** following the code style guidelines

3. **Add tests** for your new functionality

4. **Update documentation** including README files and doc comments

5. **Run quality checks**:
    ```bash
    ./scripts/cargo-clippy.sh
    ./scripts/cargo-fmt.sh
    cargo test
    ```

### Adding New Crates

When adding a new crate to the workspace:

1. **Create the crate structure** in the appropriate directory
2. **Add to workspace** in `Cargo.toml`:
    ```toml
    members = [
        # ... existing members
        "your-new-crate"
    ]
    ```
3. **Add dependencies** to the workspace dependencies section
4. **Update publish script** in `scripts/publish-crate.sh` if the crate should be published

## Adding New Decoders

### Decoder Structure

A generated decoder crate follows this layout:

```
decoders/your-program-decoder/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── accounts/         # account types + decoder
│   │   ├── postgres/     # optional: sqlx row impls
│   │   └── graphql/      # optional: juniper schemas
│   ├── instructions/     # instruction types + decoder
│   │   ├── postgres/
│   │   └── graphql/
│   ├── types/            # shared IDL-defined types
│   │   └── graphql/
│   └── graphql/          # top-level query roots
└── tests/
    └── fixtures/         # captured on-chain JSON payloads
```

The `postgres/` and `graphql/` modules are gated by Cargo features (`--with-postgres`, `--with-graphql` at scaffold time) and only generated when enabled.

### Creating a New Decoder

1. **Generate decoder using CLI** (recommended):

    ```bash
    carbon-cli parse --idl <program-address-or-idl-path> -u mainnet-beta --out-dir ./decoders/your-program-decoder
    ```

2. **Manual creation**:
    - Create the directory structure
    - Implement `AccountDecoder` and `InstructionDecoder` traits
    - Add proper error handling
    - Include comprehensive tests

3. **Add to workspace**:
    - Update `Cargo.toml` workspace dependencies
    - Add to publish script if needed

### Decoder Best Practices

- **Error handling**: Use proper error types and provide meaningful error messages
- **Documentation**: Document all public APIs and complex logic
- **Testing**: Include unit tests and integration tests with real transaction data
- **Performance**: Optimize for performance, especially for high-frequency updates

## Adding New Datasources

### Datasource Structure

Each datasource follows a consistent structure:

```
datasources/your-datasource/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs
```

### Creating a New Datasource

1. **Implement the `Datasource` trait**:

    ```rust
    use {
        async_trait::async_trait,
        carbon_core::{
            datasource::{Datasource, DatasourceId, Update, UpdateType},
            error::CarbonResult,
        },
        tokio_util::sync::CancellationToken,
    };

    pub struct YourDatasource;

    #[async_trait]
    impl Datasource for YourDatasource {
        async fn consume(
            &self,
            id: DatasourceId,
            sender: tokio::sync::mpsc::Sender<(Update, DatasourceId)>,
            cancellation_token: CancellationToken,
        ) -> CarbonResult<()> {
            // produce updates and `sender.send((update, id.clone()))` until cancelled
            Ok(())
        }

        fn update_types(&self) -> Vec<UpdateType> {
            vec![UpdateType::AccountUpdate, UpdateType::Transaction]
        }
    }
    ```

    The full set of update types is `AccountUpdate`, `Transaction`, `AccountDeletion`, and `BlockDetails`. Declare all variants your datasource may emit. A worked, runnable reference: [`examples/custom-datasource`](examples/custom-datasource).

2. **Add configuration options** for flexibility
3. **Include proper error handling** and retry logic
4. **Add comprehensive tests** with mocked data

### Datasource Best Practices

- **Reliability**: Implement proper error handling and retry mechanisms
- **Performance**: Optimize for throughput and low latency
- **Configuration**: Provide flexible configuration options
- **Monitoring**: Include metrics and logging for observability

## Submitting Changes

### Pull Request Process

1. **Fork the repository** and create a feature branch

2. **Make your changes** following the guidelines above

3. **Test thoroughly**:

    ```bash
    cargo test
    ./scripts/cargo-clippy.sh
    ./scripts/cargo-fmt.sh
    ```

4. **Update documentation** as needed

5. **Create a pull request** with:
    - Clear description of changes
    - Link to any related issues
    - Screenshots or examples if applicable

### Pull Request Guidelines

- **Title**: Use conventional commit format (e.g., "feat: add new decoder for X program")
- **Description**: Explain what, why, and how
- **Tests**: Ensure all tests pass
- **Documentation**: Update relevant documentation
- **Breaking changes**: Clearly mark and explain breaking changes

### Commit Message Format

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Version Management

- **Version**: Managed in `Cargo.toml` workspace package section
- **Rust version**: Specified in `Cargo.toml` and `clippy.toml`
- **Dependencies**: All workspace dependencies are centralized

## Getting Help

- **Issues**: Use GitHub issues for bug reports and feature requests
- **Discussions**: Use GitHub discussions for questions and general discussion
- **Documentation**: Check the README and example projects
- **Examples**: Review the `examples/` directory for working implementations

## Code of Conduct

Please be respectful and inclusive in all interactions. We welcome contributors from all backgrounds and experience levels.

## License

By contributing to Carbon, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Carbon! Your contributions help make Solana indexing more accessible and powerful for the entire ecosystem.
