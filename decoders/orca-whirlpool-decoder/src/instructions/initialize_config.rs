use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd07f1501c2bec446")]
pub struct InitializeConfig {
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
    pub default_protocol_fee_rate: u16,
}

pub struct InitializeConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeConfig {
    type ArrangedAccounts = InitializeConfigInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let funder = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(InitializeConfigInstructionAccounts {
            config: config.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
