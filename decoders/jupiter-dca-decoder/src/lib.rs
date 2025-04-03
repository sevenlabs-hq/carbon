use solana_pubkey::Pubkey;

pub struct JupiterDcaDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M");
