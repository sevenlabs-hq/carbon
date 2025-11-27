use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1e1e77f0bfe30c10")]
pub struct LiquidUnstake {
    pub msol_amount: u64,
}

pub struct LiquidUnstakeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub msol_mint: solana_pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_pubkey::Pubkey,
    pub treasury_msol_account: solana_pubkey::Pubkey,
    pub get_msol_from: solana_pubkey::Pubkey,
    pub get_msol_from_authority: solana_pubkey::Pubkey,
    pub transfer_sol_to: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidUnstake {
    type ArrangedAccounts = LiquidUnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            msol_mint,
            liq_pool_sol_leg_pda,
            liq_pool_msol_leg,
            treasury_msol_account,
            get_msol_from,
            get_msol_from_authority,
            transfer_sol_to,
            system_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(LiquidUnstakeInstructionAccounts {
            state: state.pubkey,
            msol_mint: msol_mint.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            treasury_msol_account: treasury_msol_account.pubkey,
            get_msol_from: get_msol_from.pubkey,
            get_msol_from_authority: get_msol_from_authority.pubkey,
            transfer_sol_to: transfer_sol_to.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
