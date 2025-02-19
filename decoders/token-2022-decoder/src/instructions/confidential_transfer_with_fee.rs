use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct ConfidentialTransferWithFee {
    pub confidential_transfer_discriminator: u8,
    #[serde(with = "BigArray")]
    pub new_source_decryptable_available_balance: [u8; 36],
    pub equality_proof_instruction_offset: i8,
    pub transfer_amount_ciphertext_validity_proof_instruction_offset: i8,
    pub fee_sigma_proof_instruction_offset: i8,
    pub fee_ciphertext_validity_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
}

pub struct ConfidentialTransferWithFeeInstructionAccounts {
    pub source_token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination_token: solana_sdk::pubkey::Pubkey,
    pub instructions_sysvar: solana_sdk::pubkey::Pubkey,
    pub equality_record: solana_sdk::pubkey::Pubkey,
    pub transfer_amount_ciphertext_validity_record: solana_sdk::pubkey::Pubkey,
    pub fee_sigma_record: solana_sdk::pubkey::Pubkey,
    pub fee_ciphertext_validity_record: solana_sdk::pubkey::Pubkey,
    pub range_record: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfidentialTransferWithFee {
    type ArrangedAccounts = ConfidentialTransferWithFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source_token, mint, destination_token, instructions_sysvar, equality_record, transfer_amount_ciphertext_validity_record, fee_sigma_record, fee_ciphertext_validity_record, range_record, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ConfidentialTransferWithFeeInstructionAccounts {
            source_token: source_token.pubkey,
            mint: mint.pubkey,
            destination_token: destination_token.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            equality_record: equality_record.pubkey,
            transfer_amount_ciphertext_validity_record: transfer_amount_ciphertext_validity_record
                .pubkey,
            fee_sigma_record: fee_sigma_record.pubkey,
            fee_ciphertext_validity_record: fee_ciphertext_validity_record.pubkey,
            range_record: range_record.pubkey,
            authority: authority.pubkey,
        })
    }
}
