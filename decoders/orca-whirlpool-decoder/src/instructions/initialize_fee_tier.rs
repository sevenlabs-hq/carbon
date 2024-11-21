use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb74a9ca070022a1e")]
pub struct InitializeFeeTier {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

pub struct InitializeFeeTierInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub fee_tier: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeFeeTier {
    type ArrangedAccounts = InitializeFeeTierInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let fee_tier = accounts.get(1)?;
        let funder = accounts.get(2)?;
        let fee_authority = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(InitializeFeeTierInstructionAccounts {
            config: config.pubkey,
            fee_tier: fee_tier.pubkey,
            funder: funder.pubkey,
            fee_authority: fee_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
