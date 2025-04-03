use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct Transfer {
    pub new_owner: solana_pubkey::Pubkey,
}

pub struct TransferInstructionAccounts {
    pub name_record: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub parent_name_record: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [name_record, owner, parent_name_record, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferInstructionAccounts {
            name_record: name_record.pubkey,
            owner: owner.pubkey,
            parent_name_record: parent_name_record.pubkey,
        })
    }
}
