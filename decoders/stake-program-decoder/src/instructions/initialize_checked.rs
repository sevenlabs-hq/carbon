use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdb5a3aa18b58f61c")]
pub struct InitializeChecked {}

pub struct InitializeCheckedInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub stake_authority: solana_pubkey::Pubkey,
    pub withdraw_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeChecked {
    type ArrangedAccounts = InitializeCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, rent, stake_authority, withdraw_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeCheckedInstructionAccounts {
            stake: stake.pubkey,
            rent: rent.pubkey,
            stake_authority: stake_authority.pubkey,
            withdraw_authority: withdraw_authority.pubkey,
        })
    }
}
