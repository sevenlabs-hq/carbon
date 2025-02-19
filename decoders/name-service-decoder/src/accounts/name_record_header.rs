use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x319893826f292f59")]
pub struct NameRecordHeader {
    pub parent_name: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub class: solana_sdk::pubkey::Pubkey,
}
