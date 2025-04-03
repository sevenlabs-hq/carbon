use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x270fd04d20c36938")]
pub struct ReferrerTokenState {
    pub referrer: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub amount_unclaimed_sf: u128,
    pub amount_cumulative_sf: u128,
    pub bump: u64,
    pub padding: [u64; 31],
}
