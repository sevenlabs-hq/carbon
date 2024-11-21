use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3eecf81cdee8b649")]
pub struct MarinadeDeposit {}

pub struct MarinadeDepositInstructionAccounts {
    pub marinade_finance_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub transfer_from: solana_sdk::pubkey::Pubkey,
    pub mint_to: solana_sdk::pubkey::Pubkey,
    pub msol_mint_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub user_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MarinadeDeposit {
    type ArrangedAccounts = MarinadeDepositInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let marinade_finance_program = accounts.get(0)?;
        let state = accounts.get(1)?;
        let msol_mint = accounts.get(2)?;
        let liq_pool_sol_leg_pda = accounts.get(3)?;
        let liq_pool_msol_leg = accounts.get(4)?;
        let liq_pool_msol_leg_authority = accounts.get(5)?;
        let reserve_pda = accounts.get(6)?;
        let transfer_from = accounts.get(7)?;
        let mint_to = accounts.get(8)?;
        let msol_mint_authority = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let user_wsol_token_account = accounts.get(12)?;
        let temp_wsol_token_account = accounts.get(13)?;
        let user_transfer_authority = accounts.get(14)?;
        let payer = accounts.get(15)?;
        let wsol_mint = accounts.get(16)?;
        let rent = accounts.get(17)?;

        Some(MarinadeDepositInstructionAccounts {
            marinade_finance_program: marinade_finance_program.pubkey,
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
            user_wsol_token_account: user_wsol_token_account.pubkey,
            temp_wsol_token_account: temp_wsol_token_account.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            payer: payer.pubkey,
            wsol_mint: wsol_mint.pubkey,
            rent: rent.pubkey,
        })
    }
}
