//! Declarative helper macros for decoder authors.
//!
//! `no_std`-compatible — usable inside generated decoder crates without pulling
//! in `std`.
//!
//! # Components
//!
//! - [`try_decode_instructions!`] — match an instruction's data + accounts
//!   against a list of `(variant, type)` pairs and return the first hit.

#![no_std]

pub mod try_decode_ixs;
