use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x9b0caae01efacc82")]
pub struct Config {
    pub pool_fees: PoolFees,
    pub activation_duration: u64,
    pub vault_config_key: solana_sdk::pubkey::Pubkey,
    pub pool_creator_authority: solana_sdk::pubkey::Pubkey,
    pub activation_type: u8,
    pub partner_fee_numerator: u64,
    pub padding: [u8; 219],
}
