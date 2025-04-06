use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x9dd6dceb6287ab1c")]
pub struct UserMetadata {
    pub referrer: solana_pubkey::Pubkey,
    pub bump: u64,
    pub user_lookup_table: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub padding1: [u64; 51],
    pub padding2: [u64; 64],
}
