use solana_pubkey::Pubkey;

pub struct OpenbookV2Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb");
