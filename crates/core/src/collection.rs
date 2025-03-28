//! Provides traits and utilities for decoding and managing collections of
//! Solana instructions.
//!
//! This module defines the `InstructionDecoderCollection` trait, which enables
//! decoding of Solana `Instruction` objects into structured types, allowing for
//! streamlined processing of instructions in a `carbon-core` pipeline. The
//! trait abstracts the decoding logic, providing an easy way to parse various
//! instruction types and retrieve their corresponding metadata for further
//! processing.
//!
//! # Overview
//!
//! The `InstructionDecoderCollection` trait is designed to support the handling
//! of different Solana instructions within a unified pipeline. Implementations
//! of this trait are expected to define how each instruction is decoded and
//! represented as an internal type, facilitating efficient management and
//! manipulation of instruction data.
//!
//! Key features include:
//! - **Instruction Type Association**: Associates instructions with specific
//!   types, allowing for the decoding of multiple instruction types within a
//!   single collection.
//! - **Parsing Capability**: Provides a standardized method to parse Solana
//!   instructions into custom types suitable for application logic.
//! - **Type Retrieval**: Includes a method for retrieving the instruction type
//!   associated with each instance, enabling easy type-based routing or
//!   processing.
//!
//! # Notes
//!
//! - Implementations must be `Clone`, `Debug`, `Send`, `Sync`, `Eq`, `Hash`,
//!   and `Serialize`, making them suitable for concurrent and distributed
//!   environments where instruction collections need to be transmitted, cloned,
//!   or processed in parallel.
//! - This module assumes familiarity with Solana's `Instruction` type and its
//!   role in transaction processing.
//!
//! Use the `InstructionDecoderCollection` trait to build flexible, type-safe
//! instruction handling within your application, benefiting from simplified
//! parsing and type management capabilities.

use {crate::instruction::DecodedInstruction, serde::Serialize};

/// A trait for defining collections of Solana instructions, enabling parsing
/// and type-based management.
///
/// The `InstructionDecoderCollection` trait provides an interface for decoding
/// Solana `Instruction` objects into custom types that can be processed within
/// the `carbon-core` pipeline. This trait requires implementing methods to
/// parse instructions and retrieve associated instruction types, allowing for
/// flexible handling of different instruction variants in a single collection.
///
/// ## Associated Types
///
/// - `InstructionType`: Defines the specific type of instruction being handled,
///   which is expected to be `Clone`, `Debug`, `PartialEq`, `Eq`, `Send`, and
///   `Sync`. This type is used to classify decoded instructions, making them
///   easier to process based on their category or type.
///
/// ## Required Methods
///
/// ### `parse_instruction`
///
/// This method is responsible for converting a Solana `Instruction` object into
/// a `DecodedInstruction` containing the custom type defined by the
/// implementor. The parsed instruction can then be processed within the
/// pipeline according to application-specific logic.
///
/// - **Parameters**:
///   - `instruction`: A reference to a `solana_instruction::Instruction`,
///     representing the raw instruction to be decoded.
/// - **Returns**: An `Option<DecodedInstruction<Self>>` containing the decoded
///   instruction if successful, or `None` if parsing fails or the instruction
///   is unsupported.
///
/// ### `get_type`
///
/// Retrieves the instruction type associated with an instruction. This type can
/// be used to classify instructions or route them to specific processing logic
/// based on their category.
///
/// - **Returns**: An instance of `Self::InstructionType`, representing the
///   specific type of instruction.
///
/// ## Notes
///
/// - This trait requires implementors to be thread-safe (`Send` and `Sync`) and
///   serializable (`Serialize`), which is particularly useful for distributed
///   systems where instruction collections are transmitted across network
///   boundaries.
/// - The `parse_instruction` method must be implemented with care, as it
///   determines how raw instruction data is transformed into a decoded form,
///   impacting subsequent processing within the pipeline.
pub trait InstructionDecoderCollection:
    Clone + std::fmt::Debug + Send + Sync + Eq + std::hash::Hash + Serialize + 'static
{
    type InstructionType: Clone + std::fmt::Debug + PartialEq + Eq + Send + Sync + 'static;

    fn parse_instruction(
        instruction: &solana_instruction::Instruction,
    ) -> Option<DecodedInstruction<Self>>;
    fn get_type(&self) -> Self::InstructionType;
}
