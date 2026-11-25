use solana_pubkey::Pubkey;

pub struct LimitlessLaunchpadDecoder;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("LimitQGBbcHeGJZd3FbQEejiCRuk1rfvwhbv8XCtAoN");

pub mod accounts;
pub mod events;
pub mod instructions;
pub mod types;
