use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x74dbcce5f974ff96")]
pub struct TokenBadge {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
}
