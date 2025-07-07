# Contributing to Carbon

Thank you for your interest in contributing to Carbon! This document provides guidelines and information for contributors to help make the contribution process smooth and effective.

## Table of Contents

- [About Carbon](#about-carbon)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Code Style and Standards](#code-style-and-standards)
- [Testing](#testing)
- [Adding New Features](#adding-new-features)
- [Adding New Decoders](#adding-new-decoders)
- [Adding New Datasources](#adding-new-datasources)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## About Carbon

Carbon is an indexing framework for Solana that provides a modular pipeline for sourcing data, decoding updates, and processing them to build end-to-end indexers. The project is organized as a Rust workspace with multiple crates covering different aspects of the indexing ecosystem.

## Development Setup

### Prerequisites

- **Rust**: Version 1.82 or higher (see `rust-toolchain.toml` for exact version)
- **Git**: For version control
- **Cargo**: Rust's package manager (included with Rust)

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

This will register the following checks that run on each commit:

- **fmt**: Checks code formatting using `cargo fmt --check`
- **clippy**: Runs `cargo clippy` to catch potential issues
- **cargo_sort**: Uses `cargo-sort` to ensure Cargo.toml files are sorted correctly
- **machete**: Checks for unused Cargo dependencies using `cargo-machete`

## Project Structure

The Carbon project is organized as a Rust workspace with the following main components:

### Core Crates (`crates/`)

- **`carbon-core`**: The main framework providing pipeline orchestration
- **`carbon-cli`**: Command-line interface for generating decoders and scaffolding projects
- **`carbon-macros`**: Procedural macros for the framework
- **`carbon-proc-macros`**: Additional procedural macros
- **`carbon-test-utils`**: Testing utilities and helpers
- **`carbon-gql-server`**: GraphQL server implementation
- **`carbon-postgres-client`**: PostgreSQL client for data storage

### Datasources (`datasources/`)

Data source implementations for various Solana data streams:

- **`carbon-rpc-block-subscribe-datasource`**: WebSocket-based block subscription
- **`carbon-rpc-program-subscribe-datasource`**: Program-specific account updates
- **`carbon-yellowstone-grpc-datasource`**: Yellowstone gRPC Geyser client
- **`carbon-helius-atlas-ws-datasource`**: Helius Atlas WebSocket integration
- **`carbon-jito-shredstream-grpc-datasource`**: JITO shredstream integration
- **`carbon-rpc-block-crawler-datasource`**: Historical block crawling
- **`carbon-rpc-transaction-crawler-datasource`**: Historical transaction crawling

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

Working examples demonstrating various use cases:

- **`block-finality-alerts`**: Block processing example
- **`jupiter-swap-alerts`**: Jupiter swap monitoring
- **`kamino-alerts`**: Kamino lending monitoring
- **`token-indexing`**: Token account indexing with PostgreSQL
- And more...

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

Each decoder follows a consistent structure:

```
decoders/your-program-decoder/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   │   ├── account/
│   │   │   ├── mod.rs
│   │   │   └── decoder.rs
│   │   ├── instruction/
│   │   │   ├── mod.rs
│   │   │   └── decoder.rs
│   │   └── types/
│   │       ├── mod.rs
│   │       └── types.rs
│   └── tests/
│       └── fixtures/
```

### Creating a New Decoder

1. **Generate decoder using CLI** (recommended):

   ```bash
   carbon-cli parse --idl program_address -u mainnet-beta --output ./decoders/your-program-decoder
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
   use carbon_core::datasource::{Datasource, Update, UpdateType};
   use async_trait::async_trait;

   pub struct YourDatasource;

   #[async_trait]
   impl Datasource for YourDatasource {
       async fn consume(
           &self,
           sender: &tokio::sync::mpsc::UnboundedSender<Update>,
           cancellation_token: CancellationToken,
       ) -> CarbonResult<()> {
           // Implementation
       }

       fn update_types(&self) -> Vec<UpdateType> {
           vec![UpdateType::AccountUpdate, UpdateType::Transaction]
       }
   }
   ```

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
