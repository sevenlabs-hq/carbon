use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x181ec828051c0777")]
pub struct Create {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct CreateInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_bonding_curve: solana_sdk::pubkey::Pubkey,
    pub global: solana_sdk::pubkey::Pubkey,
    pub mpl_token_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        if accounts.len() != 14 {
            return None;
        }

        Some(CreateInstructionAccounts {
            mint: accounts[0].pubkey,
            mint_authority: accounts[1].pubkey,
            bonding_curve: accounts[2].pubkey,
            associated_bonding_curve: accounts[3].pubkey,
            global: accounts[4].pubkey,
            mpl_token_metadata: accounts[5].pubkey,
            metadata: accounts[6].pubkey,
            user: accounts[7].pubkey,
            system_program: accounts[8].pubkey,
            token_program: accounts[9].pubkey,
            associated_token_program: accounts[10].pubkey,
            rent: accounts[11].pubkey,
            event_authority: accounts[12].pubkey,
            program: accounts[13].pubkey,
        })
    }
}
