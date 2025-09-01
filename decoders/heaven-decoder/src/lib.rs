use solana_pubkey::Pubkey;

pub struct HeavenDecoder;

pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o");
