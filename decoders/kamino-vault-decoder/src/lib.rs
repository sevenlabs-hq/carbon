use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct KaminoVaultDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("kvauTFR8qm1dhniz6pYuBZkuene3Hfrs1VQhVRgCNrr");
