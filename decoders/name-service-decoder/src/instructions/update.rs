use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01")]
pub struct Update {
    pub offset: u32,
    pub data: Vec<u8>,
}

pub struct UpdateInstructionAccounts {
    pub name_record: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub parent_name_record: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Update {
    type ArrangedAccounts = UpdateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [name_record, owner, parent_name_record, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateInstructionAccounts {
            name_record: name_record.pubkey,
            owner: owner.pubkey,
            parent_name_record: parent_name_record.pubkey,
        })
    }
}
