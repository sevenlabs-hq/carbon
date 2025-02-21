use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x0a68c0cb813c2802")]
pub struct WhitelistInsuranceAccount {
    pub nonce: u8,
    pub user_key: solana_sdk::pubkey::Pubkey,
}
