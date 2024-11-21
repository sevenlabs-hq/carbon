use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xff136363284153ff")]
pub struct PumpdotfunWrappedSell {}

pub struct PumpdotfunWrappedSellInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub global: solana_sdk::pubkey::Pubkey,
    pub fee_recipient: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_user: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub user_wsol_token_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PumpdotfunWrappedSell {
    type ArrangedAccounts = PumpdotfunWrappedSellInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let global = accounts.get(1)?;
        let fee_recipient = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let bonding_curve = accounts.get(4)?;
        let associated_bonding_curve = accounts.get(5)?;
        let associated_user = accounts.get(6)?;
        let user = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;
        let user_wsol_token_account = accounts.get(13)?;

        Some(PumpdotfunWrappedSellInstructionAccounts {
            swap_program: swap_program.pubkey,
            global: global.pubkey,
            fee_recipient: fee_recipient.pubkey,
            mint: mint.pubkey,
            bonding_curve: bonding_curve.pubkey,
            associated_bonding_curve: associated_bonding_curve.pubkey,
            associated_user: associated_user.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            user_wsol_token_account: user_wsol_token_account.pubkey,
        })
    }
}
