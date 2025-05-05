use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd22b65d7778c6ada")]
pub struct InitiateAuthorityTransfer {
    pub new_authority: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitiateAuthorityTransferInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitiateAuthorityTransfer {
    type ArrangedAccounts = InitiateAuthorityTransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, config, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitiateAuthorityTransferInstructionAccounts {
            authority: authority.pubkey,
            config: config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
