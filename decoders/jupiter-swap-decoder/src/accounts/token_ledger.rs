use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x9cf709bc366c554d")]
pub struct TokenLedger {
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
}
