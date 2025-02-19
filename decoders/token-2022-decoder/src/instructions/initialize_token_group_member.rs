use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9820deb0dfed7486")]
pub struct InitializeTokenGroupMember {}

pub struct InitializeTokenGroupMemberInstructionAccounts {
    pub member: solana_sdk::pubkey::Pubkey,
    pub member_mint: solana_sdk::pubkey::Pubkey,
    pub member_mint_authority: solana_sdk::pubkey::Pubkey,
    pub group: solana_sdk::pubkey::Pubkey,
    pub group_update_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTokenGroupMember {
    type ArrangedAccounts = InitializeTokenGroupMemberInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [member, member_mint, member_mint_authority, group, group_update_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeTokenGroupMemberInstructionAccounts {
            member: member.pubkey,
            member_mint: member_mint.pubkey,
            member_mint_authority: member_mint_authority.pubkey,
            group: group.pubkey,
            group_update_authority: group_update_authority.pubkey,
        })
    }
}
