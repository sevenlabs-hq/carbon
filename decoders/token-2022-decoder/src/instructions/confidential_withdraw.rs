use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct ConfidentialWithdraw {
    pub confidential_transfer_discriminator: u8,
    pub amount: u64,
    pub decimals: u8,
    #[serde(with = "BigArray")]
    pub new_decryptable_available_balance: [u8; 36],
    pub equality_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
}

pub struct ConfidentialWithdrawInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub instructions_sysvar: solana_sdk::pubkey::Pubkey,
    pub equality_record: solana_sdk::pubkey::Pubkey,
    pub range_record: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfidentialWithdraw {
    type ArrangedAccounts = ConfidentialWithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, mint, instructions_sysvar, equality_record, range_record, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ConfidentialWithdrawInstructionAccounts {
            token: token.pubkey,
            mint: mint.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            equality_record: equality_record.pubkey,
            range_record: range_record.pubkey,
            authority: authority.pubkey,
        })
    }
}
