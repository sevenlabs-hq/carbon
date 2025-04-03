use solana_pubkey::Pubkey;

pub struct MarginfiV2Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA");
