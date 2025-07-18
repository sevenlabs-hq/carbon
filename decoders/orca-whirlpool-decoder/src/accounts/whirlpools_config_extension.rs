use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0263d7a3f01a993a")]
pub struct WhirlpoolsConfigExtension {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub config_extension_authority: solana_pubkey::Pubkey,
    pub token_badge_authority: solana_pubkey::Pubkey,
}
