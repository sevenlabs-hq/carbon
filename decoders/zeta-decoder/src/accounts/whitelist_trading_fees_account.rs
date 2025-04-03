use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xdb27bda689f354ef")]
pub struct WhitelistTradingFeesAccount {
    pub nonce: u8,
    pub user_key: solana_pubkey::Pubkey,
}
