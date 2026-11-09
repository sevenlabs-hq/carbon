//! Multi-decoder routing primitive for transaction-level instruction dispatch.
//!
//! # Overview
//!
//! `Pipeline::transaction(...)` evaluates each instruction in a transaction
//! against a unified enum. `InstructionDecoderCollection` defines this enum,
//! where each variant wraps the `InstructionType` of a registered decoder.
//! `parse_instruction` selects the matching variant for a given instruction.
//!
//! Typical usage is `instruction_decoder_collection!`, which generates the
//! enum and its implementation from the decoders' instruction enums.
//!
//! # Notes
//!
//! - The enum must be `Serialize`, so generated decoders need their `serde`
//!   feature: only with it do their instruction enums derive `Serialize`,
//!   `Deserialize` and `InstructionType`, which the macro relies on.

use serde::Serialize;

/// Enum-based instruction router used by `Pipeline::transaction(...)`.
///
/// Implemented as a wrapper over multiple decoders, enabling unified
/// dispatch of heterogeneous instruction types.
///
/// # Associated Types
///
/// - [`InstructionType`](Self::InstructionType): Discriminant returned by
///   [`get_type`](Self::get_type), used for routing within the pipe.
pub trait InstructionDecoderCollection:
    Clone + std::fmt::Debug + Send + Sync + Serialize + 'static
{
    type InstructionType: Clone + std::fmt::Debug + PartialEq + Eq + Send + Sync + 'static;

    fn parse_instruction(instruction: &solana_instruction::Instruction) -> Option<Self>;

    fn get_type(&self) -> Self::InstructionType;
}
