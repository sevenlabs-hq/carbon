//! # Carbon Macros
//!
//! This crate provides powerful macros for decoding instructions dynamically.
//! It includes the `try_decode_instructions!` macro, enabling dynamic decoding
//! of instructions into various types based on specified patterns.
//!
//! ## Overview
//!
//! This module is designed for applications that utilize Carbon, where
//! transaction processing and decoding are essential. The macros in this crate
//! simplify handling diverse instruction types, reducing boilerplate code and
//! enhancing flexibility in transaction management.
//!
//! ## Modules
//!
//! - **`try_decode_ix`**: For decoding instructions dynamically.
#![no_std]

pub mod try_decode_ixs;
