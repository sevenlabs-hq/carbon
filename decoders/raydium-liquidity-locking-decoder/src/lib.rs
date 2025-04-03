use solana_pubkey::Pubkey;

pub struct RaydiumLiquidityLockingDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");
