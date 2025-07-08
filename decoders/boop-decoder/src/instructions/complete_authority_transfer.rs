use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x51e95b84af1f978d")]
pub struct CompleteAuthorityTransfer {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CompleteAuthorityTransferInstructionAccounts {
    pub pending_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CompleteAuthorityTransfer {
    type ArrangedAccounts = CompleteAuthorityTransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pending_authority, config, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CompleteAuthorityTransferInstructionAccounts {
            pending_authority: pending_authority.pubkey,
            config: config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
