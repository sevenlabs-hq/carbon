use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x326e5fb3c24b8cf6")]
pub struct DelegateStake {}

pub struct DelegateStakeInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub vote: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub stake_history: solana_pubkey::Pubkey,
    pub stake_config: solana_pubkey::Pubkey,
    pub stake_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DelegateStake {
    type ArrangedAccounts = DelegateStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, vote, clock, stake_history, stake_config, stake_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DelegateStakeInstructionAccounts {
            stake: stake.pubkey,
            vote: vote.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_config: stake_config.pubkey,
            stake_authority: stake_authority.pubkey,
        })
    }
}
