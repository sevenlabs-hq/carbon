use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct ConfidentialTransfer {
    pub confidential_transfer_discriminator: u8,
    #[serde(with = "BigArray")]
    pub new_source_decryptable_available_balance: [u8; 36],
    pub equality_proof_instruction_offset: i8,
    pub ciphertext_validity_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
}

pub struct ConfidentialTransferInstructionAccounts {
    pub source_token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination_token: solana_sdk::pubkey::Pubkey,
    pub instructions_sysvar: solana_sdk::pubkey::Pubkey,
    pub equality_record: solana_sdk::pubkey::Pubkey,
    pub ciphertext_validity_record: solana_sdk::pubkey::Pubkey,
    pub range_record: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfidentialTransfer {
    type ArrangedAccounts = ConfidentialTransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source_token, mint, destination_token, instructions_sysvar, equality_record, ciphertext_validity_record, range_record, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ConfidentialTransferInstructionAccounts {
            source_token: source_token.pubkey,
            mint: mint.pubkey,
            destination_token: destination_token.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            equality_record: equality_record.pubkey,
            ciphertext_validity_record: ciphertext_validity_record.pubkey,
            range_record: range_record.pubkey,
            authority: authority.pubkey,
        })
    }
}
