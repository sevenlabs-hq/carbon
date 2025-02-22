use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct DriftDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");
