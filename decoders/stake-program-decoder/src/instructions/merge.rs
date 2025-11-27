use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x948dec2fae7e456f")]
pub struct Merge {}

pub struct MergeInstructionAccounts {
    pub to: solana_pubkey::Pubkey,
    pub from: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub stake_history: solana_pubkey::Pubkey,
    pub stake_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Merge {
    type ArrangedAccounts = MergeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            to,
            from,
            clock,
            stake_history,
            stake_authority,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(MergeInstructionAccounts {
            to: to.pubkey,
            from: from.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_authority: stake_authority.pubkey,
        })
    }
}
