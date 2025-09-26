//! Compatibility helpers for converting various v2/SDK-legacy transaction error
//! representations (for example those produced by older helius/ws SDKs) into
//! the v3 `solana_transaction_error::TransactionError` used in this workspace.

use solana_program::instruction::InstructionError;
use solana_transaction_error::TransactionError;

/// Convert a textual/UI-style transaction error into a v3 `TransactionError`.
///
/// This helper is intentionally conservative: it attempts to parse numeric
/// custom error codes when present and falls back to a generic
/// `InstructionError::Custom(0)` when the source error cannot be mapped more
/// precisely. You can extend the mapping below for more accurate conversions
/// as needed.
pub fn convert_ui_transaction_error<S: AsRef<str>>(input: S) -> TransactionError {
    let s = input.as_ref();

    // Try to parse a raw integer error code (some Ui errors encode custom
    // program errors as integers or numeric strings). If successful, map to
    // TransactionError::Custom which is the closest v3 equivalent.
    if let Ok(code) = s.parse::<i64>() {
        return TransactionError::InstructionError(0, InstructionError::Custom(code as u32));
    }

    // Map a few common textual patterns to more specific variants. This is
    // intentionally small — add cases here for any patterns your data source
    // emits frequently.
    if s.contains("AccountNotFound") {
        return TransactionError::InstructionError(0, InstructionError::UninitializedAccount);
    }

    if s.contains("InstructionError") {
        // The UI text sometimes embeds instruction-level errors. We don't have
        // enough structured information here to reconstruct the full
        // InstructionError in most cases, so return a generic InstructionError
        // with a Custom(0) code.
        return TransactionError::InstructionError(0, InstructionError::Custom(0));
    }

    // Default fallback: return a generic InstructionError::Custom(0). This
    // makes sure callers expecting a TransactionError can proceed even if the
    // source error is opaque.
    TransactionError::InstructionError(0, InstructionError::Custom(0))
}
