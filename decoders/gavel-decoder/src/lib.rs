use solana_pubkey::Pubkey;

pub struct GavelDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi");
