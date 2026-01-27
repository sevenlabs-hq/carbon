#[macro_export]
macro_rules! try_decode_instructions {
    ($instruction:expr, $($variant:path => $ty:ty),* $(,)?) => {{
        use carbon_core::deserialize::{CarbonDeserialize, ArrangeAccounts};
        $(
            if let Some(decoded_instruction) = <$ty>::deserialize($instruction.data.as_slice()) {
                if let Some(arranged_accounts) = <$ty>::arrange_accounts(&$instruction.accounts) {
                    Some(carbon_core::instruction::DecodedInstruction {
                        program_id: $instruction.program_id,
                        accounts: arranged_accounts,
                        data: $variant(decoded_instruction),
                    })
                } else {
                    None
                }
            } else
        )*
        {
            None
        }
    }};
}
