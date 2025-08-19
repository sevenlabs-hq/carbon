use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8409e5767576753e")]
pub struct InitializePerpMarket {
    pub market_index: u16,
    pub amm_base_asset_reserve: u128,
    pub amm_quote_asset_reserve: u128,
    pub amm_periodicity: i64,
    pub amm_peg_multiplier: u128,
    pub oracle_source: OracleSource,
    pub contract_tier: ContractTier,
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub imf_factor: u32,
    pub active_status: bool,
    pub base_spread: u32,
    pub max_spread: u32,
    pub max_open_interest: u128,
    pub max_revenue_withdraw_per_period: u64,
    pub quote_max_insurance: u64,
    pub order_step_size: u64,
    pub order_tick_size: u64,
    pub min_order_size: u64,
    pub concentration_coef_scale: u128,
    pub curve_update_intensity: u8,
    pub amm_jit_intensity: u8,
    pub name: [u8; 32],
}

pub struct InitializePerpMarketInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePerpMarket {
    type ArrangedAccounts = InitializePerpMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, oracle, rent, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializePerpMarketInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
