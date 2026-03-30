//! Shared borsh types that correct Codama mis-mappings.
//!
//! Codama auto-generates type aliases like `type OptionBool = bool;` which
//! lose the "optional trailing field" semantics present in on-chain programs.
//! This crate provides correct implementations with EOF-tolerant deserializers.

mod option_bool;

pub use option_bool::OptionBool;
