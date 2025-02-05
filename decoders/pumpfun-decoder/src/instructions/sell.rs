use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x33e685a4017f83ad")]
pub struct Sell {
    pub amount: u64,
    pub min_sol_output: u64,
}

pub struct SellInstructionAccounts {
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
}

impl carbon_core::deserialize::ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global, fee_recipient, mint, bonding_curve, associated_bonding_curve, associated_user, user, system_program, associated_token_program, token_program, event_authority, program] =
            accounts
        else {
            return None;
        };

        Some(SellInstructionAccounts {
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
        })
    }
}
