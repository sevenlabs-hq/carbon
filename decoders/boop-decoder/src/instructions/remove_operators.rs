use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2a145953de25046d")]
pub struct RemoveOperators {
    pub operators: Vec<solana_pubkey::Pubkey>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveOperatorsInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveOperators {
    type ArrangedAccounts = RemoveOperatorsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, authority, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RemoveOperatorsInstructionAccounts {
            config: config.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
