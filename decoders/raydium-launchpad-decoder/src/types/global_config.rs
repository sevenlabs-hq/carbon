use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GlobalConfig {
    pub epoch: u64,
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
    pub max_share_fee_rate: u64,
    pub min_base_supply: u64,
    pub max_lock_rate: u64,
    pub min_base_sell_rate: u64,
    pub min_base_migrate_rate: u64,
    pub min_quote_fund_raising: u64,
    pub quote_mint: solana_pubkey::Pubkey,
    pub protocol_fee_owner: solana_pubkey::Pubkey,
    pub migrate_fee_owner: solana_pubkey::Pubkey,
    pub migrate_to_amm_wallet: solana_pubkey::Pubkey,
    pub migrate_to_cpswap_wallet: solana_pubkey::Pubkey,
    pub padding: [u64; 16],
}
