use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x29")]
pub struct InitializeGroupMemberPointer {
    pub group_member_pointer_discriminator: u8,
    pub authority: Option<solana_sdk::pubkey::Pubkey>,
    pub member_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeGroupMemberPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeGroupMemberPointer {
    type ArrangedAccounts = InitializeGroupMemberPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeGroupMemberPointerInstructionAccounts { mint: mint.pubkey })
    }
}
