use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xce80984d70a40d02")]
pub struct Underlying {
    pub mint: solana_pubkey::Pubkey,
}
