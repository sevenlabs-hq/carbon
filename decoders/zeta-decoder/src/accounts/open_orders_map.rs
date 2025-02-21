use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xfa7eac0a761e03a8")]
pub struct OpenOrdersMap {
    pub user_key: solana_sdk::pubkey::Pubkey,
}
