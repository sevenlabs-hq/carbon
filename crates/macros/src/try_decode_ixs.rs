//! # Instruction Decoding Module
//!
//! The `try_decode_ix` module provides the `try_decode_instructions!` macro, a
//! flexible and powerful tool for dynamically decoding instructions into
//! multiple potential types.
//!
//! ## Usage
//!
//! To use the `try_decode_instructions!` macro, specify the instruction to
//! decode along with a series of variant-type pairs. The macro attempts to
//! decode the instruction into each type sequentially, returning the first
//! successful match. If no match is found, `None` is returned.

/// Attempts to decode an instruction into a unified enum variant.
///
/// The `try_decode_instructions!` macro takes an instruction and tries to
/// decode it into one of the provided instruction types. If decoding is
/// successful, it returns the instruction enum with both the decoded data
/// and arranged accounts in a single struct variant. If no variant type
/// matches, it returns `None`.
///
/// This macro is useful for handling multiple potential instruction types
/// dynamically, enabling streamlined processing of instructions without
/// manually matching each type.
///
/// # Syntax
///
/// ```ignore
/// try_decode_instructions!(
///     instruction,
///     PROGRAM_ID,
///     MyEnum::VariantA => TypeA,
///     MyEnum::VariantB => TypeB,
/// );
/// ```
///
/// - `$instruction`: The instruction to decode.
/// - `$program_id`: The program ID for this instruction set.
/// - `$enum_name::$variant`: The full path to the enum variant.
/// - `$ty`: The instruction data type that implements `CarbonDeserialize` and `ArrangeAccounts`.
///
/// # Example
///
/// ```ignore
/// use carbon_macros::try_decode_instructions;
///
/// let instruction = Instruction { /* initialize with program_id, accounts, and data */ };
///
/// let decoded = try_decode_instructions!(
///     instruction,
///     PROGRAM_ID,
///     MyEnum::VariantOne => TypeOne,
///     MyEnum::VariantTwo => TypeTwo,
/// );
/// ```
///
/// # Parameters
///
/// - `$instruction`: The instruction being decoded, which must include
///   `program_id`, `accounts`, and `data` fields.
/// - `$program_id`: The program ID to be stored in the resulting enum variant.
/// - `$enum_name::$variant`: The enum name and variant name separated by `::`.
///   The enum must have struct variants with `program_id`, `data`, and `accounts` fields.
/// - `$ty`: The instruction data type, which must implement a `decode(&[u8]) -> Option<Self>`
///   method for data deserialization and `ArrangeAccounts` for account arrangement.
///
/// # Returns
///
/// Returns an `Option<EnumType>` containing the instruction enum variant with
/// both decoded data and arranged accounts if successful. Returns `None` if
/// deserialization or account arrangement fails.
///
/// # Notes
///
/// - Each instruction type must implement a `decode` method and the `ArrangeAccounts` trait.
/// - The macro automatically arranges accounts during decoding, eliminating the need
///   for manual arrangement in processors.
/// - The resulting enum variant contains three fields: `program_id`, `data`, and `accounts`.
/// - The macro iterates through each variant sequentially, returning the first successful match.
#[macro_export]
macro_rules! try_decode_instructions {
    ($instruction:expr, $program_id:expr, $($enum_name:ident :: $variant:ident => $ty:ty),* $(,)?) => {{
        use carbon_core::deserialize::ArrangeAccounts;
        $(
            if let Some(data) = <$ty>::decode($instruction.data.as_slice()) {
                if let Some(accounts) = <$ty>::arrange_accounts(&$instruction.accounts) {
                    let result = $enum_name::$variant {
                        program_id: $program_id,
                        data: data,
                        accounts: accounts,
                    };
                    return Some(result);
                }
            }
        )*
        None
    }};
}
