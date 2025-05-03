use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5e837db8b7187de5")]
pub struct CancelAuthorityTransfer {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelAuthorityTransferInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelAuthorityTransfer {
    type ArrangedAccounts = CancelAuthorityTransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, config, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelAuthorityTransferInstructionAccounts {
            authority: authority.pubkey,
            config: config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
