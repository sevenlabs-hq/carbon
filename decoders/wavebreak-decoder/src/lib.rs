use solana_pubkey::Pubkey;

pub struct WavebreakDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("waveQX2yP3H1pVU8djGvEHmYg8uamQ84AuyGtpsrXTF");
