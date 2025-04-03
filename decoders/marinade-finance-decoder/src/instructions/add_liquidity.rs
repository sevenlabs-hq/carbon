use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb59d59438fb63448")]
pub struct AddLiquidity {
    pub lamports: u64,
}

pub struct AddLiquidityInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub lp_mint_authority: solana_pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_pubkey::Pubkey,
    pub transfer_from: solana_pubkey::Pubkey,
    pub mint_to: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidity {
    type ArrangedAccounts = AddLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, lp_mint, lp_mint_authority, liq_pool_msol_leg, liq_pool_sol_leg_pda, transfer_from, mint_to, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddLiquidityInstructionAccounts {
            state: state.pubkey,
            lp_mint: lp_mint.pubkey,
            lp_mint_authority: lp_mint_authority.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            transfer_from: transfer_from.pubkey,
            mint_to: mint_to.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
