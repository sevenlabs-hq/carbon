use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct SetAuthority {
    pub authority_type: AuthorityType,
    pub new_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct SetAuthorityInstructionAccounts {
    pub owned: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetAuthority {
    type ArrangedAccounts = SetAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owned, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetAuthorityInstructionAccounts {
            owned: owned.pubkey,
            owner: owner.pubkey,
        })
    }
}
