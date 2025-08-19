use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xadc166d2db897178")]
pub struct Authorize {
    pub new_authority: solana_pubkey::Pubkey,
    pub stake_authorize: StakeAuthorize,
}

pub struct AuthorizeInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Authorize {
    type ArrangedAccounts = AuthorizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, clock, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AuthorizeInstructionAccounts {
            stake: stake.pubkey,
            clock: clock.pubkey,
            authority: authority.pubkey,
        })
    }
}
