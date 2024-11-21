use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct CreateConfigAccount {}

pub struct CreateConfigAccountInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateConfigAccount {
    type ArrangedAccounts = CreateConfigAccountInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let amm_config = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let rent = accounts.get(4)?;

        Some(CreateConfigAccountInstructionAccounts {
            admin: admin.pubkey,
            amm_config: amm_config.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
