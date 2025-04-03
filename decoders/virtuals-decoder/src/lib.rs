use solana_pubkey::Pubkey;

pub struct VirtualsDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("5U3EU2ubXtK84QcRjWVmYt9RaDyA8gKxdUrPFXmZyaki");
