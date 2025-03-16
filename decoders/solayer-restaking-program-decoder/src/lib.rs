use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct SolayerRestakingProgramDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey = pubkey!("sSo1iU21jBrU9VaJ8PJib1MtorefUV4fzC9GURa2KNn");
