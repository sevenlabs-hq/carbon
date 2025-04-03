use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct Create {
    pub hashed_name: Vec<u8>,
    pub lamports: u64,
    pub space: u32,
}

pub struct CreateInstructionAccounts {
    pub system_program: solana_pubkey::Pubkey,
    pub funding_account: solana_pubkey::Pubkey,
    pub name_record: solana_pubkey::Pubkey,
    pub account_class: solana_pubkey::Pubkey,
    pub parent_name_record: solana_pubkey::Pubkey,
    pub parent_name_record_class: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [system_program, funding_account, name_record, account_class, parent_name_record, parent_name_record_class, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateInstructionAccounts {
            system_program: system_program.pubkey,
            funding_account: funding_account.pubkey,
            name_record: name_record.pubkey,
            account_class: account_class.pubkey,
            parent_name_record: parent_name_record.pubkey,
            parent_name_record_class: parent_name_record_class.pubkey,
        })
    }
}
