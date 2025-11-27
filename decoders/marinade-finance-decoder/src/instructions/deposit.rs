use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub lamports: u64,
}

pub struct DepositInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub msol_mint: solana_pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_pubkey::Pubkey,
    pub liq_pool_msol_leg_authority: solana_pubkey::Pubkey,
    pub reserve_pda: solana_pubkey::Pubkey,
    pub transfer_from: solana_pubkey::Pubkey,
    pub mint_to: solana_pubkey::Pubkey,
    pub msol_mint_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            msol_mint,
            liq_pool_sol_leg_pda,
            liq_pool_msol_leg,
            liq_pool_msol_leg_authority,
            reserve_pda,
            transfer_from,
            mint_to,
            msol_mint_authority,
            system_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            state: state.pubkey,
            msol_mint: msol_mint.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            liq_pool_msol_leg_authority: liq_pool_msol_leg_authority.pubkey,
            reserve_pda: reserve_pda.pubkey,
            transfer_from: transfer_from.pubkey,
            mint_to: mint_to.pubkey,
            msol_mint_authority: msol_mint_authority.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
