use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe011749de6d41eda")]
pub struct AssetV1 {
    pub key: Key,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub update_authority: UpdateAuthority,
    pub name: String,
    pub uri: String,
    pub seq: Option<u64>,
}
