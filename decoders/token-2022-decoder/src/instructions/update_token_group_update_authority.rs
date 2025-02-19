use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa1695801edddd8cb")]
pub struct UpdateTokenGroupUpdateAuthority {
    pub new_update_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateTokenGroupUpdateAuthorityInstructionAccounts {
    pub group: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateTokenGroupUpdateAuthority {
    type ArrangedAccounts = UpdateTokenGroupUpdateAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [group, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTokenGroupUpdateAuthorityInstructionAccounts {
            group: group.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
