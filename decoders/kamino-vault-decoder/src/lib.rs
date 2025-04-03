use solana_pubkey::Pubkey;

pub struct KaminoVaultDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("kvauTFR8qm1dhniz6pYuBZkuene3Hfrs1VQhVRgCNrr");
