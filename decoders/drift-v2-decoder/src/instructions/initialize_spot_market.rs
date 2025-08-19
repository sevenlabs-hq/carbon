use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeac4802c5e0f30c9")]
pub struct InitializeSpotMarket {
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
    pub oracle_source: OracleSource,
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub active_status: bool,
    pub asset_tier: AssetTier,
    pub scale_initial_asset_weight_start: u64,
    pub withdraw_guard_threshold: u64,
    pub order_tick_size: u64,
    pub order_step_size: u64,
    pub if_total_factor: u32,
    pub name: [u8; 32],
}

pub struct InitializeSpotMarketInstructionAccounts {
    pub spot_market: solana_pubkey::Pubkey,
    pub spot_market_mint: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeSpotMarket {
    type ArrangedAccounts = InitializeSpotMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [spot_market, spot_market_mint, spot_market_vault, insurance_fund_vault, drift_signer, state, oracle, admin, rent, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeSpotMarketInstructionAccounts {
            spot_market: spot_market.pubkey,
            spot_market_mint: spot_market_mint.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            state: state.pubkey,
            oracle: oracle.pubkey,
            admin: admin.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
