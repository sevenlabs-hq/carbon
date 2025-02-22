use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct WeightedSwapDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW");
