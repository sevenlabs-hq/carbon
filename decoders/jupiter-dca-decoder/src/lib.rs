use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct JupiterDcaDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey = pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M");
