/// Try-decode a Solana `Instruction` against a series of variant/type pairs,
/// returning the first variant whose `decode` and `arrange_accounts` both
/// succeed.
///
/// Each `$ty` must implement `CarbonDeserialize` (for `decode`) and
/// `ArrangeAccounts` (for the structured `accounts` payload). On a hit, the
/// macro `return`s `Some(EnumName::Variant { program_id, data, accounts })`
/// from the calling function; otherwise it yields `None`.
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
