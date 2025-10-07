use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DepositForBurn {
    pub burn_token: solana_pubkey::Pubkey,
    pub amount: u64,
    pub depositor: solana_pubkey::Pubkey,
    pub mint_recipient: solana_pubkey::Pubkey,
    pub destination_domain: u32,
    pub destination_token_messenger: solana_pubkey::Pubkey,
    pub destination_caller: solana_pubkey::Pubkey,
    pub max_fee: u64,
    pub min_finality_threshold: u32,
    pub hook_data: Vec<u8>,
}
