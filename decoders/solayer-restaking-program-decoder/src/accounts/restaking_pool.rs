use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c05648f7d5e1ad6")]
pub struct RestakingPool {
    pub lst_mint: solana_pubkey::Pubkey,
    pub rst_mint: solana_pubkey::Pubkey,
    pub bump: u8,
}
