//! Instruction decoding into a collection's output type.

use crate::error::BoxError;

/// Decodes an instruction into a collection variant, or returns `Ok(None)` for a non-match.
pub trait InstructionDecoderCollection: Sized {
    fn decode_instruction(
        instruction: &solana_instruction::Instruction,
    ) -> Result<Option<Self>, BoxError>;
}
