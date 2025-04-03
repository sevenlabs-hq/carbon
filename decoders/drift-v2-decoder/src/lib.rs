use solana_pubkey::Pubkey;

pub struct DriftDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");
