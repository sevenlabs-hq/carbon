use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x29")]
pub struct UpdateGroupMemberPointer {
    pub group_member_pointer_discriminator: u8,
    pub member_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateGroupMemberPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub group_member_pointer_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGroupMemberPointer {
    type ArrangedAccounts = UpdateGroupMemberPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, group_member_pointer_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGroupMemberPointerInstructionAccounts {
            mint: mint.pubkey,
            group_member_pointer_authority: group_member_pointer_authority.pubkey,
        })
    }
}
