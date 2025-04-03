use solana_pubkey::Pubkey;

pub struct StableSwapDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ");
