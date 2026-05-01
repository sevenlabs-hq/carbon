//! Multi-decoder routing primitive for transaction-level instruction dispatch.
//!
//! # Overview
//!
//! `Pipeline::transaction(...)` evaluates each instruction in a transaction
//! against a unified enum. `InstructionDecoderCollection` defines this enum,
//! where each variant wraps the `InstructionType` of a registered decoder.
//! `parse_instruction` selects the matching variant for a given instruction.
//!
//! Typical usage is a hand-written wrapper enum over all relevant decoders,
//! with `parse_instruction` attempting each decoder sequentially.
//!
//! # Notes
//!
//! - `Eq + Hash + Serialize` are required for internal routing and persistence.
//! - Generated decoders (e.g. Codama) usually lack these bounds, so the wrapper
//!   enum is maintained manually or via macro.

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
    Clone + std::fmt::Debug + Send + Sync + Eq + std::hash::Hash + Serialize + 'static
{
    type InstructionType: Clone + std::fmt::Debug + PartialEq + Eq + Send + Sync + 'static;

    fn parse_instruction(instruction: &solana_instruction::Instruction) -> Option<Self>;

    fn get_type(&self) -> Self::InstructionType;
}
