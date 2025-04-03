use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd07f1501c2bec446")]
pub struct InitializeConfig {
    pub fee_authority: solana_pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_pubkey::Pubkey,
    pub default_protocol_fee_rate: u16,
}

pub struct InitializeConfigInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeConfig {
    type ArrangedAccounts = InitializeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, funder, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeConfigInstructionAccounts {
            config: config.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
