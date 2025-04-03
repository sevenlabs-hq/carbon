use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x74dbcce5f974ff96")]
pub struct TokenBadge {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
}
