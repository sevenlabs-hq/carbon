#[macro_export]
macro_rules! try_decode_instructions {
    ($instruction:expr, $($variant:path => $ty:ty),* $(,)?) => {{
        $(
            if let Some(decoded_instruction) = <$ty>::deserialize($instruction.data.as_slice()) {
                Some(carbon_core::instruction::DecodedInstruction {
                    program_id: $instruction.program_id,
                    accounts: $instruction.accounts,
                    data: $variant(decoded_instruction),
                })
            } else
        )*
        {
            None
        }
    }};
}
