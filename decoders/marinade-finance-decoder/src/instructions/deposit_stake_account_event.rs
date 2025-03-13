use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de7cb76604b7446e4")]
pub struct DepositStakeAccountEvent {
    pub state: solana_sdk::pubkey::Pubkey,
    pub stake: solana_sdk::pubkey::Pubkey,
    pub delegated: u64,
    pub withdrawer: solana_sdk::pubkey::Pubkey,
    pub stake_index: u32,
    pub validator: solana_sdk::pubkey::Pubkey,
    pub validator_index: u32,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub user_msol_balance: u64,
    pub msol_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
