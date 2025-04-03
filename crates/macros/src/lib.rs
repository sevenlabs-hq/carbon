//! # Carbon Macros
//!
//! This crate provides powerful macros for building and processing transaction
//! schemas and decoding instructions dynamically. It includes two main modules:
//!
//! - **`schema`**: Offers the `schema!` macro to construct hierarchical
//!   transaction schemas with flexible node types, ideal for organizing and
//!   validating complex transaction structures.
//! - **`try_decode_ix`**: Includes the `try_decode_instructions!` macro,
//!   enabling dynamic decoding of instructions into various types based on
//!   specified patterns.
//!
//! ## Overview
//!
//! These modules are designed for applications that utilize Carbon, where
//! transaction processing and decoding are essential. The macros in this crate
//! simplify handling diverse instruction types and assembling transaction
//! schemas, reducing boilerplate code and enhancing flexibility in transaction
//! management.
//!
//! ## Modules
//!
//! - **`schema`**: For building transaction schemas.
//! - **`try_decode_ix`**: For decoding instructions dynamically.
#![no_std]

pub mod schemas;
pub mod try_decode_ixs;
