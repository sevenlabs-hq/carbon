

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5055d14818ceb16c")]
pub struct RemoveLiquidity{
    pub tokens: u64,
}

pub struct RemoveLiquidityInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub burn_from: solana_sdk::pubkey::Pubkey,
    pub burn_from_authority: solana_sdk::pubkey::Pubkey,
    pub transfer_sol_to: solana_sdk::pubkey::Pubkey,
    pub transfer_msol_to: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveLiquidity {
    type ArrangedAccounts = RemoveLiquidityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            lp_mint,
            burn_from,
            burn_from_authority,
            transfer_sol_to,
            transfer_msol_to,
            liq_pool_sol_leg_pda,
            liq_pool_msol_leg,
            liq_pool_msol_leg_authority,
            system_program,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RemoveLiquidityInstructionAccounts {
            state: state.pubkey,
            lp_mint: lp_mint.pubkey,
            burn_from: burn_from.pubkey,
            burn_from_authority: burn_from_authority.pubkey,
            transfer_sol_to: transfer_sol_to.pubkey,
            transfer_msol_to: transfer_msol_to.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            liq_pool_msol_leg_authority: liq_pool_msol_leg_authority.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}