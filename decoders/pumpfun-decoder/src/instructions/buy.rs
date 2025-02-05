use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x66063d1201daebea")]
pub struct Buy {
    pub amount: u64,
    pub max_sol_cost: u64,
}

pub struct BuyInstructionAccounts {
    pub global: solana_sdk::pubkey::Pubkey,
    pub fee_recipient: solana_sdk::pubkey::Pubkey,
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

impl carbon_core::deserialize::ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        if accounts.len() != 12 {
            return None;
        }

        Some(BuyInstructionAccounts {
            global: accounts[0].pubkey,
            fee_recipient: accounts[1].pubkey,
            mint: accounts[2].pubkey,
            bonding_curve: accounts[3].pubkey,
            associated_bonding_curve: accounts[4].pubkey,
            associated_user: accounts[5].pubkey,
            user: accounts[6].pubkey,
            system_program: accounts[7].pubkey,
            token_program: accounts[8].pubkey,
            rent: accounts[9].pubkey,
            event_authority: accounts[10].pubkey,
            program: accounts[11].pubkey,
        })
    }
}
