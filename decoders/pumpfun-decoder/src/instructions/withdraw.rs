use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {}

pub struct WithdrawInstructionAccounts {
    pub global: solana_sdk::pubkey::Pubkey,
    pub last_withdraw: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_user: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let global = accounts.get(0)?;
        let last_withdraw = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let bonding_curve = accounts.get(3)?;
        let associated_bonding_curve = accounts.get(4)?;
        let associated_user = accounts.get(5)?;
        let user = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(WithdrawInstructionAccounts {
            global: global.pubkey,
            last_withdraw: last_withdraw.pubkey,
            mint: mint.pubkey,
            bonding_curve: bonding_curve.pubkey,
            associated_bonding_curve: associated_bonding_curve.pubkey,
            associated_user: associated_user.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
