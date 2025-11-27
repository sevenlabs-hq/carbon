use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6e02d95144ae78d9")]
pub struct WhitelistDepositAccount {
    pub nonce: u8,
    pub user_key: solana_pubkey::Pubkey,
}
