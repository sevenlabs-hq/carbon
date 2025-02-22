use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct LifinityAmmV2Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c");
