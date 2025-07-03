//! # Carbon Core
//!
//! `carbon-core` is a framework designed for building customizable and
//! extensible indexers tailored to Solana blockchain data. It facilitates
//! efficient data ingestion, transformation, and processing, supporting a wide
//! range of use cases from transaction parsing to complex instruction analysis.
//! This crate includes modular components that enable users to process
//! blockchain data flexibly and with ease.
//!
//! The true power of this framework lies when utilizing all it's components in
//! combination with one another.
//!
//! ## Modules Overview
//!
//! - **[`account`]**: Manages account data processing, including decoding and
//!   updates. Account data is processed through pipes that support custom
//!   decoders and processors.
//!
//! - **[`account_deletion`]**: Handles the deletion of accounts and processes
//!   these events in the pipeline.
//!
//! - **[`collection`]**: Defines collections for instruction decoding, allowing
//!   for customized instruction parsers that handle specific instruction sets.
//!
//! - **[`datasource`]**: Provides data ingestion capabilities, enabling the
//!   integration of external data sources into the pipeline. Supports
//!   Solana-specific data structures.
//!
//! - **[`deserialize`]**: Contains utilities for data deserialization,
//!   including helper functions for parsing Solana transactions and other
//!   binary data formats.
//!
//! - **[`error`]**: Defines error types used throughout the crate, providing
//!   consistent error handling for the framework.
//!
//! - **[`filter`]**: Provides a flexible filtering system that allows selective
//!   processing of updates based on various criteria such as datasource ID,
//!   update content, or custom logic. Filters can be applied to different
//!   types of updates (accounts, instructions, transactions, account deletions,
//!   and block details) to control which updates are processed by specific pipes.
//!
//! - **[`instruction`]**: Supports instruction parsing and processing within
//!   transactions. This module includes structures and traits for decoding and
//!   handling transaction instructions.
//!
//! - **[`metrics`]**: Facilitates performance monitoring and metric recording
//!   within the pipeline. Metrics can be customized and are recorded at each
//!   processing stage for monitoring and debugging purposes.
//!
//! - **[`pipeline`]**: Represents the core of the framework, defining the main
//!   pipeline structure that manages data flow and processing. The pipeline
//!   integrates data sources, processing pipes, and metrics to provide a
//!   complete data processing solution.
//!
//! - **[`processor`]**: Contains traits and implementations for processing data
//!   in the pipeline. This module allows for the creation of custom data
//!   processors that can be integrated into various stages of the pipeline.
//!
//! - **[`schema`]**: Defines transaction schemas, allowing for structured
//!   parsing and validation of transaction data based on specified rules.
//!   Supports complex nested instruction matching for comprehensive transaction
//!   analysis.
//!
//! - **[`transaction`]**: Manages transaction data, including metadata
//!   extraction and parsing. This module supports transaction validation and
//!   processing, enabling detailed transaction insights.
//!
//! - **[`transformers`]**: Provides utility functions for transforming and
//!   restructuring data. This module includes functions for converting Solana
//!   transaction data into formats suitable for processing within the
//!   framework.
//!
//! ## Quick Start
//!
//! To create a new `carbon-core` pipeline, start by configuring data sources,
//! processing pipes, and metrics in the [`pipeline::PipelineBuilder`]. Below is
//! a basic example demonstrating how to set up a pipeline:
//!
//! ```ignore
//! use std::sync::Arc;
//!
//! carbon_core::pipeline::Pipeline::builder()
//! .datasource(transaction_crawler)
//! .metrics(Arc::new(LogMetrics::new()))
//! .metrics(Arc::new(PrometheusMetrics::new()))
//! .instruction(
//!    TestProgramDecoder,
//!    TestProgramProcessor
//! )
//! .account(
//!     TestProgramDecoder,
//!     TestProgramAccountProcessor
//! )
//! .transaction(TEST_SCHEMA.clone(), TestProgramTransactionProcessor)
//! .account_deletions(TestProgramAccountDeletionProcessor)
//! .build()?
//! .run()
//! .await?;
//! ```
//!
//! ## Crate Features
//!
//! - **Modular Design**: Components can be easily added or replaced, allowing
//!   for a high degree of customization.
//! - **Concurrency Support**: Built with asynchronous Rust, enabling efficient
//!   data processing in parallel.
//! - **Solana-Specific**: Tailored to handle Solana blockchain data structures,
//!   making it ideal for blockchain data analysis and transaction processing.
//!
//! ## Notes
//!
//! - `carbon-core` integrates with Solana's SDK, leveraging types and data
//!   structures specific to the Solana blockchain.
//! - This framework is designed for advanced use cases, such as blockchain
//!   indexing, transaction monitoring, and custom data analysis.
//!
//! Explore each module in detail to understand their individual functions and
//! to learn how to customize and extend `carbon-core` to suit your specific
//! data processing requirements.

pub mod account;
pub mod account_deletion;
mod block_details;
pub mod collection;
pub mod datasource;
pub mod deserialize;
pub mod error;
pub mod filter;
pub mod instruction;
pub mod metrics;
pub mod pipeline;
pub mod processor;
pub mod schema;
pub mod transaction;
pub mod transformers;

pub use borsh;
#[cfg(feature = "macros")]
pub use carbon_macros::*;
#[cfg(feature = "macros")]
pub use carbon_proc_macros::*;
#[cfg(feature = "macros")]
#[doc(hidden)]
pub use log;
