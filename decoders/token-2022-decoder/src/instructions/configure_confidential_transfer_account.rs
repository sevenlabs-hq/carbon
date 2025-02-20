use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct ConfigureConfidentialTransferAccount {
    pub confidential_transfer_discriminator: u8,
    #[serde(with = "BigArray")]
    pub decryptable_zero_balance: [u8; 36],
    pub maximum_pending_balance_credit_counter: u64,
    pub proof_instruction_offset: i8,
}

pub struct ConfigureConfidentialTransferAccountInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub instructions_sysvar_or_context_state: solana_sdk::pubkey::Pubkey,
    pub record: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigureConfidentialTransferAccount {
    type ArrangedAccounts = ConfigureConfidentialTransferAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, mint, instructions_sysvar_or_context_state, record, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ConfigureConfidentialTransferAccountInstructionAccounts {
            token: token.pubkey,
            mint: mint.pubkey,
            instructions_sysvar_or_context_state: instructions_sysvar_or_context_state.pubkey,
            record: record.pubkey,
            authority: authority.pubkey,
        })
    }
}
