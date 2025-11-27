use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct Delete {}

pub struct DeleteInstructionAccounts {
    pub name_record: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub refund_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Delete {
    type ArrangedAccounts = DeleteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [name_record, owner, refund_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeleteInstructionAccounts {
            name_record: name_record.pubkey,
            owner: owner.pubkey,
            refund_account: refund_account.pubkey,
        })
    }
}
