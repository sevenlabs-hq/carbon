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

/// Attempts to decode an instruction into a specific variant type.
///
/// The `try_decode_instructions!` macro takes an instruction and tries to
/// decode it into one of the provided variant types. If decoding is successful,
/// it returns a `DecodedInstruction` object with the decoded data wrapped in
/// the specified variant. If none of the variant types match, it returns
/// `None`.
///
/// This macro is useful for handling multiple potential instruction types
/// dynamically, enabling streamlined processing of instructions without
/// manually matching each type.
///
/// # Syntax
///
/// ```ignore
/// try_decode_instructions!(instruction, VariantA => TypeA, VariantB => TypeB, ...);
/// ```
///
/// - `$instruction`: The instruction to decode.
/// - `$variant`: The enum variant to wrap the decoded instruction data.
/// - `$ty`: The type to which the instruction data should be deserialized.
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
///     MyEnum::VariantOne => TypeOne,
///     MyEnum::VariantTwo => TypeTwo,
/// );
/// ```
///
/// # Parameters
///
/// - `$instruction`: The instruction being decoded, which must include
///   `program_id`, `accounts`, and `data` fields. The `data` field should be a
///   byte slice compatible with the deserialization process.
/// - `$variant`: Enum variants that wrap the deserialized data. These variants
///   should correspond to valid instruction types within the context.
/// - `$ty`: The type for each variant, which must implement a `deserialize`
///   method to convert the instruction data into the appropriate form.
///
/// # Returns
///
/// Returns an `Option<DecodedInstruction>` that contains the decoded
/// instruction wrapped in the specified variant type if decoding is successful.
/// If no variant type matches, it returns `None`.
///
/// # Notes
///
/// - Ensure that each `$ty` type implements a `deserialize` method, as this is
///   necessary for the macro to attempt decoding. The deserialization method
///   should handle byte slices.
/// - The macro iterates over each variant type sequentially, returning the
///   first successful match. If no types match, `None` is returned.
/// - This macro is especially useful for processing complex transactions where
///   multiple instruction types are possible, improving flexibility and
///   reducing boilerplate code.
#[macro_export]
macro_rules! try_decode_instructions {
    ($instruction:expr, $($variant:path => $ty:ty),* $(,)?) => {{
        use carbon_core::deserialize::CarbonDeserialize;
        $(
            if let Some(decoded_instruction) = <$ty>::deserialize($instruction.data.as_slice()) {
                Some(carbon_core::instruction::DecodedInstruction {
                    program_id: $instruction.program_id,
                    accounts: $instruction.accounts.clone(),
                    data: $variant(decoded_instruction),
                })
            } else
        )*
        {
            None
        }
    }};
}
