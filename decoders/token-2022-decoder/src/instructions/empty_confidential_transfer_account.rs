use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct EmptyConfidentialTransferAccount {
    pub confidential_transfer_discriminator: u8,
    pub proof_instruction_offset: i8,
}

pub struct EmptyConfidentialTransferAccountInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub instructions_sysvar_or_context_state: solana_pubkey::Pubkey,
    pub record: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EmptyConfidentialTransferAccount {
    type ArrangedAccounts = EmptyConfidentialTransferAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, instructions_sysvar_or_context_state, record, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(EmptyConfidentialTransferAccountInstructionAccounts {
            token: token.pubkey,
            instructions_sysvar_or_context_state: instructions_sysvar_or_context_state.pubkey,
            record: record.pubkey,
            authority: authority.pubkey,
        })
    }
}
